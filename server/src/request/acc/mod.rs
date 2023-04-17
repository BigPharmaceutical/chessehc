use std::borrow::Cow;

use ed25519_dalek::PUBLIC_KEY_LENGTH;

use crate::{
    config::{validate_username, USERNAME_MAX_LENGTH, USERNAME_MIN_LENGTH},
    db::account,
    response::{
        err::{
            inval_req::{
                auth::{challenge::Challenge, id::Identity, Authentication},
                perms::Permissions,
                InvalidRequest,
            },
            mal_req::MalformedRequest,
            Error,
        },
        ok::Ok,
        Response, Result,
    },
    server::handler::Client,
};

use super::{Requester, RequesterRunResult};

const LOG_OUT_OP_CODE: u8 = 0b0100_0000;
const CHANGE_USERNAME_OP_CODE: u8 = 0b0101_0000;
const CHANGE_PUBLIC_KEY_OP_CODE: u8 = 0b0110_0000;
const DELETE_OP_CODE: u8 = 0b0111_0000;

pub enum Account<'a> {
    LogOut,
    ChangeUsername(Cow<'a, str>),
    ChangeKey(&'a [u8]),
    Delete,
}

impl<'a> Requester<'a> for Account<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 4) & 0b11 {
            0 => Self::LogOut,
            1 => {
                if !(USERNAME_MIN_LENGTH..=USERNAME_MAX_LENGTH).contains(&(buffer.len() - 1)) {
                    return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
                        Identity::InvalidUsername,
                    ))));
                }

                let name = String::from_utf8_lossy(&buffer[1..]);

                if !validate_username(&name) {
                    return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
                        Identity::InvalidUsername,
                    ))));
                }

                Self::ChangeUsername(name)
            }
            2 => {
                if buffer.len() - 1 != PUBLIC_KEY_LENGTH {
                    return Err(Error::InvalReq(InvalidRequest::Auth(
                        Authentication::Challenge(Challenge::InvalidPublicKey),
                    )));
                }

                Self::ChangeKey(&buffer[1..])
            }
            3 => Self::Delete,
            _ => unreachable!(),
        })
    }

    fn run<'b>(self, client: &'a mut Client<'b>) -> RequesterRunResult<'a>
    where
        'b: 'a,
    {
        match self {
            Self::LogOut => Box::pin(log_out(client)),
            Self::ChangeUsername(new_username) => Box::pin(change_username(client, new_username)),
            Self::ChangeKey(new_public_key) => Box::pin(change_public_key(client, new_public_key)),
            Self::Delete => Box::pin(delete_account(client)),
        }
    }
}

async fn log_out<'a>(client: &mut Client<'a>) -> Result<()> {
    if client.log_in.take().is_none() {
        return Err(Error::InvalReq(InvalidRequest::Perm(
            Permissions::NotLoggedIn,
        )));
    }

    client
        .send(Response::Ok(Ok::Confirmation(LOG_OUT_OP_CODE)).into())
        .await
        .ok();

    Ok(())
}

async fn change_username<'a, 'b>(
    client: &mut Client<'a>,
    new_username: Cow<'b, str>,
) -> Result<()> {
    let Some(account_id) = client.log_in else {
        return Err(Error::InvalReq(InvalidRequest::Perm(Permissions::NotLoggedIn)));
    };

    match account::set_username(account_id, &new_username).await {
        Ok(true) => (),
        Ok(false) => {
            return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
                Identity::UnknownId,
            ))))
        }
        Err(_) => return Err(Error::Server),
    }

    client
        .send(Response::Ok(Ok::Confirmation(CHANGE_USERNAME_OP_CODE)).into())
        .await
        .ok();

    Ok(())
}

async fn change_public_key<'a>(client: &mut Client<'a>, new_public_key: &[u8]) -> Result<()> {
    let Some(account_id) = client.log_in else {
        return Err(Error::InvalReq(InvalidRequest::Perm(Permissions::NotLoggedIn)));
    };

    match account::set_public_key(account_id, new_public_key).await {
        Ok(true) => (),
        Ok(false) => {
            return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
                Identity::UnknownId,
            ))))
        }
        Err(_) => return Err(Error::Server),
    }

    client
        .send(Response::Ok(Ok::Confirmation(CHANGE_PUBLIC_KEY_OP_CODE)).into())
        .await
        .ok();

    Ok(())
}

async fn delete_account<'a>(client: &mut Client<'a>) -> Result<()> {
    let Some(account_id) = client.log_in.take() else {
        return Err(Error::InvalReq(InvalidRequest::Perm(Permissions::NotLoggedIn)));
    };

    match account::delete(account_id).await {
        Ok(true) => (),
        Ok(false) => {
            return Err(Error::InvalReq(InvalidRequest::Auth(Authentication::Id(
                Identity::UnknownId,
            ))))
        }
        Err(_) => return Err(Error::Server),
    }

    client
        .send(Response::Ok(Ok::Confirmation(DELETE_OP_CODE)).into())
        .await
        .ok();

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::request::{acc::Account, Request, Requester};

    use super::{
        CHANGE_PUBLIC_KEY_OP_CODE, CHANGE_USERNAME_OP_CODE, DELETE_OP_CODE, LOG_OUT_OP_CODE,
    };

    #[test]
    fn test_log_out_op_code() {
        let request = [LOG_OUT_OP_CODE];

        let log_out = Request::parse(&request);
        assert!(
            matches!(log_out, Ok(Request::Acc(Account::LogOut))),
            "op-code {LOG_OUT_OP_CODE:0>8b} is not the log out op-code"
        );
    }

    #[test]
    fn test_change_username_op_code() {
        let request = [CHANGE_USERNAME_OP_CODE, b'u', b's', b'e', b'r'];

        let change_username = Request::parse(&request);
        assert!(
            matches!(
                change_username,
                Ok(Request::Acc(Account::ChangeUsername(_)))
            ),
            "op-code {CHANGE_USERNAME_OP_CODE:0>8b} is not the change username op-code"
        );
    }

    #[test]
    fn test_change_public_key_op_code() {
        let request = [
            CHANGE_PUBLIC_KEY_OP_CODE,
            0x57,
            0x37,
            0x86,
            0x92,
            0xa2,
            0x6f,
            0x85,
            0x1e,
            0xf2,
            0x62,
            0xbd,
            0x94,
            0xf2,
            0x79,
            0x62,
            0x44,
            0xf0,
            0x03,
            0x1f,
            0xba,
            0x6e,
            0x60,
            0x76,
            0x4a,
            0x1d,
            0x63,
            0x13,
            0x80,
            0x4a,
            0x70,
            0xfa,
            0x66,
        ];

        let change_public_code = Request::parse(&request);
        assert!(
            matches!(change_public_code, Ok(Request::Acc(Account::ChangeKey(_)))),
            "op-code {CHANGE_PUBLIC_KEY_OP_CODE:0>8b} is not the change username op-code"
        );
    }

    #[test]
    fn test_delete_op_code() {
        let request = [DELETE_OP_CODE];

        let delete = Request::parse(&request);
        assert!(
            matches!(delete, Ok(Request::Acc(Account::Delete))),
            "op-code {DELETE_OP_CODE:0>8b} is not the change username op-code"
        );
    }
}
