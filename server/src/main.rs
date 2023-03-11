mod server;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let result = server::main("localhost:8082").await;
    println!("{result:?}");
}
