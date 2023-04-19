use base64::Engine;
use futures_util::{
    future::OptionFuture,
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use tokio::{
    net::TcpStream,
    select,
    sync::{
        broadcast::{self, error::RecvError},
        mpsc,
    },
    time::Instant,
};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message},
    WebSocketStream,
};

use crate::{
    config::{BASE64_ENGINE, CHALLENGE_LENGTH, GAME_SENDER_CAPACITY},
    game::{Broadcast, GameMessage, PlayerMessage},
    request::{Request, Requester},
    response::{
        self,
        err::mal_req::MalformedRequest,
        ok::{
            in_game::{
                game::{players::Players, status::Status, Game},
                InGame,
            },
            Ok,
        },
        Response,
    },
};

pub struct Client<'a> {
    pub peer_address: String,
    pub close: bool,
    write: &'a mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pub log_in_challenge: Option<(i64, [u8; CHALLENGE_LENGTH], Instant)>,
    pub log_in: Option<i64>,
    pub game_handle: (mpsc::Sender<GameMessage>, mpsc::Receiver<GameMessage>),
    pub game: (
        Option<broadcast::Receiver<Broadcast>>,
        Option<mpsc::Sender<PlayerMessage>>,
    ),
}

impl<'a> Client<'a> {
    pub fn new(
        peer_address: String,
        write: &'a mut SplitSink<WebSocketStream<TcpStream>, Message>,
    ) -> Self {
        Self {
            peer_address,
            close: false,
            write,
            log_in_challenge: None,
            log_in: None,
            game_handle: mpsc::channel(GAME_SENDER_CAPACITY),
            game: (None, None),
        }
    }

    pub async fn send(&mut self, data: Vec<u8>) -> Result<(), ()> {
        self.write.send(Message::Binary(data)).await.map_err(|_| {
            self.close = true;
        })
    }
}

pub async fn handle(socket: TcpStream) {
    // Get the peer address of the connector
    let peer_address = socket
        .peer_addr()
        .map_or_else(|_| "unknown".to_owned(), |address| address.to_string());

    // Attempt to upgrade to a WebSocket
    let Ok(ws_stream) = accept_async(socket).await else {
        eprintln!("{peer_address}: Error during handshake!");
        return;
    };

    // Split the connection
    let (mut write, mut read) = ws_stream.split();

    println!("New connection: {peer_address}");
    // Create the client
    let mut client = Client::new(peer_address, &mut write);

    while !client.close {
        let in_game = client.game.0.is_some();
        let game_broadcast: OptionFuture<_> =
            client.game.0.as_mut().map(broadcast::Receiver::recv).into();

        select! {
            update = read.next() => handle_socket(&mut client, update).await,
            update = client.game_handle.1.recv() => handle_game_message(&mut client, update).await,
            update = game_broadcast, if in_game => handle_game_broadcast(&mut client, update).await,
        }
    }

    println!("{}: Disconnected", client.peer_address);
}

impl<'a, 'b> Client<'a> {
    pub async fn handle_message(&'b mut self, data: &'b [u8])
    where
        'a: 'b,
    {
        match Request::parse(data) {
            Ok(request) => {
                if let Err(err) = request.run(self).await {
                    self.send(Response::Err(err).into()).await.ok();
                }
            }
            Err(error) => {
                self.send(Response::from(error).into()).await.ok();
            }
        };
    }
}

async fn handle_socket<'a>(client: &mut Client<'a>, update: Option<Result<Message, Error>>) {
    let Some(Ok(message)) = update else {
        client.close = true;
        return;
    };

    match message {
        Message::Binary(data) => {
            client.handle_message(&data).await;
        }
        Message::Text(encoded_data) => {
            // Decode the text to binary data
            let length = (encoded_data.len() + 3) / 4 * 3;
            let mut data = Vec::with_capacity(length);

            if BASE64_ENGINE.decode_vec(encoded_data, &mut data).is_err() {
                client.send(MalformedRequest::b64().into()).await.ok();
                return;
            }

            client.handle_message(&data).await;
        }

        Message::Ping(data) => {
            // Echo the data
            if client
                .write
                .send(Message::Pong(data.clone()))
                .await
                .is_err()
            {
                client.close = true;
            }
        }
        Message::Pong(_data) => {}

        Message::Close(_) => {
            client.close = true;
        }
        Message::Frame(_) => (),
    }
}

async fn handle_game_message<'a>(client: &mut Client<'a>, update: Option<GameMessage>) {
    let Some(message) = update else { return };

    match message {
        GameMessage::Join(broadcast) => {
            client.game.0 = Some(broadcast);
        }
        GameMessage::JoinRejection(reason) => {
            client.game = (None, None);
            client
                .send(
                    Response::Err(response::err::Error::InvalReq(
                        response::err::inval_req::InvalidRequest::Game(reason),
                    ))
                    .into(),
                )
                .await
                .ok();
        }
        GameMessage::NotGameHost => {
            client
                .send(
                    Response::Err(response::err::Error::InvalReq(
                        response::err::inval_req::InvalidRequest::Perm(
                            response::err::inval_req::perms::Permissions::NotGameHost,
                        ),
                    ))
                    .into(),
                )
                .await
                .ok();
        }
        GameMessage::TooFewPlayers => {
            client
                .send(
                    Response::Err(response::err::Error::InvalReq(
                        response::err::inval_req::InvalidRequest::Game(
                            response::err::inval_req::game::Game::TooFewPlayers,
                        ),
                    ))
                    .into(),
                )
                .await
                .ok();
        }
    }
}

async fn handle_game_broadcast<'a>(
    client: &mut Client<'a>,
    update: Option<Result<Broadcast, RecvError>>,
) {
    let Some(Ok(message)) = update else { return };

    match message {
        Broadcast::Join(id) => {
            client
                .send(
                    Response::Ok(Ok::InGame(InGame::Game(Game::Players(Players::Join(id))))).into(),
                )
                .await
                .ok();
        }
        Broadcast::Leave(_player, _deltas) => todo!(),
        Broadcast::Start { players, board } => {
            client
                .send(
                    Response::Ok(Ok::InGame(InGame::Game(Game::Status(Status::Start(
                        &players, &board,
                    )))))
                    .into(),
                )
                .await
                .ok();
        }
        Broadcast::Turn(_player) => todo!(),
        Broadcast::Move {
            player: _player,
            deltas: _deltas,
            points: _points,
        } => todo!(),
    }
}
