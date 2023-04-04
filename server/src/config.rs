use lazy_static::lazy_static;
use regex::Regex;

// DO NOT CHANGE
// The environment variable must be DATABASE_URL for `sqlx::query` to work
pub const DATABASE_URL_ENV_VARIABLE: (&str, &str) = (
    "DATABASE_URL",
    "postgres://<user>@<password>@<address>/chessehc",
);
pub const BIND_URL_ENV_VARIABLE: (&str, &str) = ("SERVER_URL", "<address>:<port>");

pub const USERNAME_MIN_LENGTH: usize = 3;
pub const USERNAME_MAX_LENGTH: usize = 15;

pub const LOGIN_TIMEOUT_SECS: u64 = 10;

lazy_static! {
    static ref USERNAME_REGEX: Regex =
        Regex::new(r"^[!./0-9?A-Z_a-z]*$").expect("failed to compile regex");
}

#[must_use]
pub fn validate_username(text: &str) -> bool {
    USERNAME_REGEX.is_match(text)
}
