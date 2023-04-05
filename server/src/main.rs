#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo
)]

use dotenv::dotenv;
use std::env;

use crate::config::{BIND_URL_ENV_VARIABLE, DATABASE_URL_ENV_VARIABLE};

mod config;
mod db;
mod request;
mod response;
mod server;
mod game;

macro_rules! get_env_var {
    ( $var:ident, $def:expr ) => {
        let Ok($var) = env::var($def.0) else {
            eprintln!("Environment variable \"{}\" is not set! (like \"{}\")", $def.0, $def.1);
            return;
        };
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    get_env_var!(db_url, DATABASE_URL_ENV_VARIABLE);
    get_env_var!(bind_url, BIND_URL_ENV_VARIABLE);

    println!("Starting Chessehc server:");

    if let Err(err) = db::init(&db_url).await {
        eprintln!("Database Error: {err}");
        return;
    }

    let result = server::main(&bind_url).await;
    println!("{result:?}");
}
