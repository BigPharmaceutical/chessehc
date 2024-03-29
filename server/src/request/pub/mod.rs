use std::borrow::Cow;

use ed25519_dalek::{VerifyingKey, PUBLIC_KEY_LENGTH};

use sqlx::error::DatabaseError;

use crate::{
    config::{validate_username, USERNAME_MAX_LENGTH, USERNAME_MIN_LENGTH},
    db::account,
    response::{
        err::{
            inval_req::{
                auth::{challenge::Challenge, id::Identity, Authentication},
                InvalidRequest,
            },
            mal_req::{mal_bin::MalformedBinary, MalformedRequest},
            Error,
        },
        ok, Response, Result,
    },
    server::handler::Client,
};

use super::{Requester, RequesterRunResult};

pub mod log_in;
pub mod profile;

use self::{log_in::LogIn, profile::Profile};

pub enum Public<'a> {
    Status,
    Profile(Profile<'a>),
    CreateAccount(Cow<'a, str>, &'a [u8]),
    LI(LogIn<'a>),
}

impl<'a> Requester<'a> for Public<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 4) & 0b11 {
            0 => Self::Status,
            1 => Self::Profile(Profile::parse(buffer)?),
            2 => parse_new_account(buffer)?,
            3 => Self::LI(LogIn::parse(buffer)?),
            _ => unreachable!(),
        })
    }

    fn run<'b>(self, client: &'a mut Client<'b>) -> RequesterRunResult<'a>
    where
        'b: 'a,
    {
        match self {
            Self::Status => todo!(),
            Self::Profile(req) => req.run(client),
            Self::CreateAccount(username, public_key) => {
                Box::pin(create_new_account(username, public_key, client))
            }
            Self::LI(req) => req.run(client),
        }
    }
}

fn parse_new_account(buffer: &[u8]) -> Result<Public> {
    let Some(null_separator_position) = buffer.iter().position(|&byte| byte == 0) else {
        return Err(Error::MalReq(MalformedRequest::Bin(MalformedBinary::Data)));
    };

    if !(USERNAME_MIN_LENGTH..=USERNAME_MAX_LENGTH).contains(&null_separator_position) {
        return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
            Identity::InvalidUsername,
        ))));
    }

    if buffer.len() - null_separator_position - 1 != PUBLIC_KEY_LENGTH {
        return Err(Error::InvalReq(InvalidRequest::Auth(
            Authentication::Challenge(Challenge::InvalidPublicKey),
        )));
    }

    let (name_bytes, public_key_bytes) = (
        &buffer[1..null_separator_position],
        &buffer[null_separator_position + 1..],
    );
    let name = String::from_utf8_lossy(name_bytes);

    if !validate_username(&name) {
        return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
            Identity::InvalidUsername,
        ))));
    }

    Ok(Public::CreateAccount(name, public_key_bytes))
}

async fn create_new_account<'a, 'b>(
    username: Cow<'a, str>,
    public_key: &[u8],
    client: &mut Client<'b>,
) -> Result<()> {
    let mut verifying_key_bytes = [0; PUBLIC_KEY_LENGTH];
    verifying_key_bytes.copy_from_slice(public_key);
    if VerifyingKey::from_bytes(&verifying_key_bytes).is_err() {
        return Err(Error::InvalReq(InvalidRequest::Auth(
            Authentication::Challenge(Challenge::InvalidPublicKey),
        )));
    }

    let id = match account::add(&username, public_key).await {
        Ok(id) => id,
        Err(err) => {
            return match err.as_database_error().map(DatabaseError::code) {
                Some(Some(err_code)) if err_code == "23505" => Err(Error::InvalReq(
                    InvalidRequest::Auth(Authentication::Id(Identity::UsernameInUse)),
                )),
                _ => {
                    eprintln!("Database Error: {err}");
                    Err(Error::Server)
                }
            }
        }
    };

    client.log_in = Some(id);

    client
        .send(
            Response::Ok(ok::Ok::Public(ok::public::Public::Profile(
                ok::public::profile::Profile::Account(
                    ok::public::profile::account::Account::AccountId(id),
                ),
            )))
            .into(),
        )
        .await
        .ok();

    Ok(())
}
