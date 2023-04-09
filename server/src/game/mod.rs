use std::{collections::HashMap, sync::Mutex};

use chessehc::standard_pieces::StandardCompatiblePieceSet;
use lazy_static::lazy_static;
use nohash_hasher::{BuildNoHashHasher, IntMap};
use tokio::sync::{broadcast, mpsc};

use crate::config::{GAME_BROADCAST_CAPACITY, GAME_MAX_CODE_SEARCH_TRIES, GAME_RECEIVER_CAPACITY};

use self::code::{next_token, token_to_code};

mod code;
pub use code::*;

lazy_static! {
    static ref GAMES: Mutex<IntMap<u64, mpsc::Sender<PlayerMessage>>> =
        Mutex::new(HashMap::with_hasher(BuildNoHashHasher::default()));
}

#[derive(Clone)]
pub enum Broadcast {
    Join(i64),
    Leave(i64),
}

pub enum PlayerMessage {
    Join(i64, mpsc::Sender<GameMessage>),
    Leave(i64),
}

#[allow(clippy::module_name_repetitions)]
pub enum GameMessage {
    Join(broadcast::Receiver<Broadcast>),
}

type NewGame = (
    String,
    broadcast::Receiver<Broadcast>,
    mpsc::Sender<PlayerMessage>,
);

macro_rules! broadcast {
    ( $tb:expr, $val:expr ) => {
        if let Err(err) = $tb.send($val) {
            eprintln!("Error Sending Broadcast: {err}");
        }
    };
}

async fn game_handler(
    token: u64,
    tb: broadcast::Sender<Broadcast>,
    mut receiver: mpsc::Receiver<PlayerMessage>,
    host_id: i64,
    host_sender: mpsc::Sender<GameMessage>,
) {
    let mut players = vec![(host_id, host_sender)];
    let mut game: Option<chessehc::game::Game<StandardCompatiblePieceSet>> = None;

    while let Some(msg) = receiver.recv().await {
        match msg {
            PlayerMessage::Join(player_id, tp) => {
                if game.is_some() {}

                let rb = tb.subscribe();
                if let Err(err) = tp.send(GameMessage::Join(rb)).await {
                    eprintln!("Error Joining Game: {err}");
                    continue;
                };

                players.push((player_id, tp));
                broadcast!(tb, Broadcast::Join(player_id));
            }
            PlayerMessage::Leave(_) => todo!(),
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
