use futures_util::{
    future,
    stream::{SplitSink, StreamExt},
    SinkExt, TryStreamExt,
};
use tokio::{net::TcpStream, time::interval};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

pub async fn handle(socket: TcpStream) {
    // Get the peer address of the connector
    let peer_address = socket
        .peer_addr()
        .map(|address| address.to_string())
        .unwrap_or("unknown".to_owned());

    // Attempt to upgrade to a WebSocket
    let Ok(ws_stream) = accept_async(socket).await else {
        eprintln!("{peer_address}: Error during handshake!");
        return;
    };

    println!("New connection: {peer_address}");

    // Split the connection
    let (mut write, mut read) = ws_stream.split();

    while let Some(Ok(message)) = read.next().await {
        match message {
            Message::Text(data) => {
                println!("{peer_address}: {data}");
            }
            Message::Binary(data) => {
                println!("{peer_address}: {data:?}");
            }
            Message::Ping(data) => {
                if write.send(Message::Pong(data.to_owned())).await.is_err() {
                    break;
                }
            }
            Message::Pong(_data) => {}
            Message::Close(_) => break,
            _ => (),
        }
    }

    println!("{peer_address}: Disconnected");
}
