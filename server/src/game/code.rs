// The locking behavior of the mutex is needed
#![allow(clippy::mutex_integer)]

use std::sync::Mutex;

use base64::Engine;
use lazy_static::lazy_static;
use rand::Rng;

use crate::config::BASE64_ENGINE;

const GAME_TOKEN_RNG_M: u64 = 1 << 40;

lazy_static! {
    static ref GAME_TOKEN_RNG: (Mutex<u64>, u64, u64, u32) = {
        let mut rng = rand::thread_rng();
        (
            Mutex::new(rng.gen_range(0..(GAME_TOKEN_RNG_M - 1))),
            rng.gen_range(0..(GAME_TOKEN_RNG_M / 4)) * 4 + 1,
            rng.gen_range(1..(GAME_TOKEN_RNG_M / 2)) * 2 + 1,
            rng.gen_range(1..(1 << 20)),
        )
    };
}

fn rng_next() -> u64 {
    let mut x = match GAME_TOKEN_RNG.0.lock() {
        Ok(x) => x,
        Err(err) => err.into_inner(),
    };

    *x = GAME_TOKEN_RNG
        .1
        .wrapping_mul(*x)
        .wrapping_add(GAME_TOKEN_RNG.2)
        & (GAME_TOKEN_RNG_M - 1);
    *x
}

fn num_to_token(num: u64, mut shift: u32) -> u64 {
    let mut bytes = num.to_be_bytes();

    for byte in &mut bytes[3..] {
        *byte = byte.rotate_right(shift & 7);
        shift >>= 3;
    }

    u64::from_be_bytes(bytes)
}

pub fn next_token() -> u64 {
    let num = rng_next();
    num_to_token(num, GAME_TOKEN_RNG.3)
}

#[allow(clippy::module_name_repetitions)]
pub fn token_to_code(token: u64) -> String {
    let token_bytes = token.to_be_bytes();
    let mut code = String::with_capacity(8);
    BASE64_ENGINE.encode_string(&token_bytes[3..], &mut code);
    code
}

#[allow(clippy::module_name_repetitions)]
pub fn code_to_token<T>(code: T) -> Result<u64, ()>
where
    T: AsRef<[u8]>,
{
    let mut token_bytes = [0; 6];
    BASE64_ENGINE
        .decode_slice(code, &mut token_bytes)
        .map_err(|_| ())?;

    let mut number_bytes = [0; 8];
    number_bytes[3..].copy_from_slice(&token_bytes[..5]);
    Ok(u64::from_be_bytes(number_bytes))
}
