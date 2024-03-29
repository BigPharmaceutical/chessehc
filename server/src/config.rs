use base64::{alphabet, engine};
use lazy_static::lazy_static;
use regex::Regex;

// The environment variable must be DATABASE_URL for `sqlx::query` to work
pub const DATABASE_URL_ENV_VARIABLE: (&str, &str) = (
    "DATABASE_URL",
    "postgres://<user>:<password>@<address>/chessehc",
);
pub const BIND_URL_ENV_VARIABLE: (&str, &str) = ("SERVER_URL", "<address>:<port>");

pub const GAME_BROADCAST_CAPACITY: usize = 10;
pub const GAME_RECEIVER_CAPACITY: usize = 20;
pub const GAME_SENDER_CAPACITY: usize = 5;

pub const GAME_MAX_CODE_SEARCH_TRIES: u8 = 200;

pub const CHALLENGE_LENGTH: usize = 32;
pub const LOGIN_TIMEOUT_SECS: u64 = 10;

pub const USERNAME_MIN_LENGTH: usize = 3;
pub const USERNAME_MAX_LENGTH: usize = 15;

pub const PLAYER_LIMIT: u8 = 32;

lazy_static! {
    static ref USERNAME_REGEX: Regex =
        Regex::new(r"^[!./0-9?A-Z_a-z]*$").expect("failed to compile regex");
}

#[must_use]
pub fn validate_username(text: &str) -> bool {
    USERNAME_REGEX.is_match(text)
}

/// Configuration without padding when encoding and optional padding when decoding
pub const BASE64_CONFIG: engine::GeneralPurposeConfig = engine::GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);

/// Base64 engine with `BASE64_CONFIG`
pub const BASE64_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::STANDARD, BASE64_CONFIG);
