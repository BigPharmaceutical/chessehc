use std::error::Error;

use tokio::net::TcpListener;

pub mod handler;

pub async fn main(address: &str) -> Result<(), Box<dyn Error>> {
    // Create TCP listener
    let listener = TcpListener::bind(address).await?;
    println!("* Listening on: {address}");

    loop {
        // Wait for inbound socket
        let (socket, _) = listener.accept().await?;

        // Spawn handler for socket
        tokio::spawn(handler::handle(socket));
    }
}
