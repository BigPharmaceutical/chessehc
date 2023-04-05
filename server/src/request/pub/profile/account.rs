use std::borrow::Cow;

use crate::{
    config::{validate_username, USERNAME_MAX_LENGTH, USERNAME_MIN_LENGTH},
    db::account,
    request::{Requester, RequesterRunResult},
    response::{
        err::{
            inval_req::{
                auth::{id::Identity, Authentication},
                InvalidRequest,
            },
            mal_req::{mal_bin::MalformedBinary, MalformedRequest},
            Error,
        },
        ok::{
            public::{
                profile::{self, Profile},
                Public,
            },
            Ok,
        },
        Response, Result,
    },
    server::handler::Client,
};

pub enum Account<'a> {
    GetUsername(i64),
    LookupUsername(Cow<'a, str>),
}

impl<'a> Requester<'a> for Account<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 2) & 0b1 {
            0 => {
                if buffer.len() != 9 {
                    return Err(Error::MalReq(MalformedRequest::Bin(MalformedBinary::Data)));
                }

                let mut account_id_bytes = [0; 8];
                account_id_bytes.copy_from_slice(&buffer[1..]);
                let account_id = i64::from_be_bytes(account_id_bytes);

                Self::GetUsername(account_id)
            }
            1 => {
                let name_bytes = &buffer[1..];

                if !(USERNAME_MIN_LENGTH..=USERNAME_MAX_LENGTH).contains(&name_bytes.len()) {
                    return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
                        Identity::InvalidUsername,
                    ))));
                }

                let name = String::from_utf8_lossy(name_bytes);

                if !validate_username(&name) {
                    return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
                        Identity::InvalidUsername,
                    ))));
                }

                Self::LookupUsername(name)
            }
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run<'b>(self, client: &'a mut Client<'b>) -> RequesterRunResult<'a>
    where
        'b: 'a,
    {
        match self {
            Self::GetUsername(account_id) => Box::pin(get_username(account_id, client)),
            Self::LookupUsername(username) => Box::pin(lookup_username(username, client)),
        }
    }
}

async fn get_username<'a>(id: i64, client: &mut Client<'a>) -> Result<()> {
    match account::get_username(id).await {
        Ok(Some(username)) => {
            client
                .send(
                    Response::Ok(Ok::Public(Public::Profile(Profile::Account(
                        profile::account::Account::Username(&username),
                    ))))
                    .into(),
                )
                .await
                .ok();
            Ok(())
        }
        Ok(None) => Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
            Identity::UnknownId,
        )))),
        Err(_) => Err(Error::Server),
    }
}

async fn lookup_username<'a, 'b>(username: Cow<'b, str>, client: &mut Client<'a>) -> Result<()>
where
    'a: 'b,
{
    match account::lookup(&username).await {
        Ok(Some(id)) => {
            client
                .send(
                    Response::Ok(Ok::Public(Public::Profile(Profile::Account(
                        profile::account::Account::AccountId(id),
                    ))))
                    .into(),
                )
                .await
                .ok();
            Ok(())
        }
        Ok(None) => Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
            Identity::UnknownUsername,
        )))),
        Err(_) => Err(Error::Server),
    }
}
