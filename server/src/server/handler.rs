use base64::Engine;
use futures_util::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use tokio::{net::TcpStream, time::Instant, sync::{broadcast, mpsc}};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

use crate::{
    config::{BASE64_ENGINE, CHALLENGE_LENGTH, GAME_SENDER_CAPACITY},
    request::{Request, Requester},
    response::{err::mal_req::MalformedRequest, Response}, game::{Broadcast, PlayerMessage, GameMessage},
};

pub struct Client<'a> {
    pub peer_address: String,
    pub close: bool,
    write: &'a mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pub log_in_challenge: Option<(i64, [u8; CHALLENGE_LENGTH], Instant)>,
    pub log_in: Option<i64>,
    pub game_handle: (mpsc::Sender<GameMessage>, mpsc::Receiver<GameMessage>),
    pub game: Option<(
        broadcast::Receiver<Broadcast>,
        mpsc::Sender<PlayerMessage>,
    )>,
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
            game: None,
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
        let Some(Ok(message)) = read.next().await else { break };

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
                    continue;
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
                    break;
                }
            }
            Message::Pong(_data) => {}

            Message::Close(_) => break,
            Message::Frame(_) => (),
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
