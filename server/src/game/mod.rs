use std::{collections::HashMap, sync::Mutex};

use chessehc::{
    delta::PartialDelta, piece_set::PieceSet, standard_pieces::StandardCompatiblePieceSet,
};
use lazy_static::lazy_static;
use nohash_hasher::{BuildNoHashHasher, IntMap};
use tokio::sync::{broadcast, mpsc};

use crate::{
    config::{
        GAME_BROADCAST_CAPACITY, GAME_MAX_CODE_SEARCH_TRIES, GAME_RECEIVER_CAPACITY, PLAYER_LIMIT,
    },
    response::err::inval_req,
};

use self::code::{next_token, token_to_code};

mod code;
pub use code::*;

lazy_static! {
    static ref GAMES: Mutex<IntMap<u64, mpsc::Sender<PlayerMessage>>> =
        Mutex::new(HashMap::with_hasher(BuildNoHashHasher::default()));
}

pub type PartialDeltas = Vec<PartialDelta<<StandardCompatiblePieceSet as PieceSet>::PieceId>>;

#[derive(Debug, Clone)]
pub enum Broadcast {
    Join(i64),
    Leave(i64, Option<PartialDeltas>),
    Start {
        players: Vec<i64>,
        board: Vec<(u8, <StandardCompatiblePieceSet as PieceSet>::PieceId)>,
    },
    Turn(i64),
    Move {
        player: i64,
        deltas: PartialDeltas,
        points: u16,
    },
}

pub enum PlayerMessage {
    Join(i64, mpsc::Sender<GameMessage>),
    Leave(i64),
    Start(i64),
}

#[allow(clippy::module_name_repetitions)]
pub enum GameMessage {
    Join(broadcast::Receiver<Broadcast>),
    JoinRejection(inval_req::game::Game),
    NotGameHost,
    TooFewPlayers,
}

type NewGame = (
    String,
    broadcast::Receiver<Broadcast>,
    mpsc::Sender<PlayerMessage>,
);

async fn game_handler(
    token: u64,
    tb: broadcast::Sender<Broadcast>,
    mut receiver: mpsc::Receiver<PlayerMessage>,
    host_id: i64,
    host_sender: mpsc::Sender<GameMessage>,
) {
    let mut players = vec![(host_id, Some(host_sender))];
    let mut game: Option<chessehc::game::Game<StandardCompatiblePieceSet>> = None;

    while let Some(msg) = receiver.recv().await {
        match msg {
            PlayerMessage::Join(player_id, tp) => {
                if game.is_some() {
                    tp.send(GameMessage::JoinRejection(inval_req::game::Game::Started))
                        .await
                        .ok();
                    continue;
                }

                if players.len() >= PLAYER_LIMIT.into() {
                    tp.send(GameMessage::JoinRejection(inval_req::game::Game::Full))
                        .await
                        .ok();
                    continue;
                }

                if players.iter().any(|(id, _)| id == &player_id) {
                    tp.send(GameMessage::JoinRejection(inval_req::game::Game::InThis))
                        .await
                        .ok();
                    continue;
                }

                let rb = tb.subscribe();
                if let Err(err) = tp.send(GameMessage::Join(rb)).await {
                    eprintln!("Error Sending Join Confirmation: {err}");
                    continue;
                };

                players.push((player_id, Some(tp)));
                tb.send(Broadcast::Join(player_id))
                    .expect("error sending broadcast");
            }
            PlayerMessage::Leave(player_id) => {
                let Some(index) = players.iter().position(|(id, _)| id == &player_id) else {
                    eprintln!("Error Removing Player From Game: player {player_id} is not in the game!");
                    continue;
                };
                players[index].1 = None;
                let index = u8::try_from(index).expect("too many players in game");

                let deltas = game.as_mut().map(|game| {
                    game.remove_player(index)
                        .expect("failed to remove player from game")
                });
                tb.send(Broadcast::Leave(player_id, deltas))
                    .expect("error sending broadcast");
            }
            PlayerMessage::Start(id) => {
                let number_of_players =
                    u8::try_from(players.len()).expect("too many players in game");

                let Some(tp) = players
                    .iter_mut()
                    .find(|(player_id, _)| *player_id == id)
                    .expect("player not in game").1.as_mut() else {
                    eprintln!("got message from left player");
                    continue;
                };

                if id != host_id {
                    if let Err(err) = tp.send(GameMessage::NotGameHost).await {
                        eprintln!("Error Sending Error: {err}");
                    }
                    continue;
                }

                if number_of_players < 2 {
                    if let Err(err) = tp.send(GameMessage::TooFewPlayers).await {
                        eprintln!("Error Sending Error: {err}");
                    }
                }

                let player_ids: Vec<i64> = players.iter().map(|(id, _)| *id).collect();

                let new_game = chessehc::game::Game::new(
                    number_of_players,
                    8,
                    7 * u16::from(number_of_players),
                );
                let (_, _, board) = new_game.board().export();
                game = Some(new_game);

                tb.send(Broadcast::Start {
                    players: player_ids,
                    board,
                })
                .expect("error sending broadcast");
            }
        }
    }

    let mut games = match GAMES.lock() {
        Ok(game) => game,
        Err(err) => err.into_inner(),
    };

    if games.remove(&token).is_none() {
        eprintln!(
            "Error Removing Game: {} was not in the game list!",
            token_to_code(token)
        );
    }

    games.shrink_to_fit();
    drop(games);
}

pub fn create(account_id: i64, th: mpsc::Sender<GameMessage>) -> Result<NewGame, ()> {
    let mut games = match GAMES.lock() {
        Ok(games) => games,
        Err(err) => err.into_inner(),
    };

    let token = 'code_gen: {
        let mut token = next_token();
        for _ in 0..GAME_MAX_CODE_SEARCH_TRIES {
            if !games.contains_key(&token) {
                break 'code_gen token;
            }
            token = next_token();
        }
        eprintln!("Error: gave up generating a code after {GAME_MAX_CODE_SEARCH_TRIES} tries!");
        return Err(());
    };

    let (tb, rb) = broadcast::channel(GAME_BROADCAST_CAPACITY);
    let (tx, rx) = mpsc::channel(GAME_RECEIVER_CAPACITY);

    games.insert(token, tx.clone());
    tokio::spawn(game_handler(token, tb, rx, account_id, th));

    Ok((token_to_code(token), rb, tx))
}

pub fn get(token: u64) -> Option<mpsc::Sender<PlayerMessage>> {
    let games = match GAMES.lock() {
        Ok(games) => games,
        Err(err) => err.into_inner(),
    };

    games.get(&token).cloned()
}
