use crate::{
    request::{Requester, RequesterRunResult},
    response::{err::mal_req::MalformedRequest, Result},
    server::handler::Client,
};

use self::account::Account;

pub mod account;

pub enum Profile<'a> {
    Account(Account<'a>),
}

impl<'a> Requester<'a> for Profile<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 3) & 0b1 {
            0 => Self::Account(Account::parse(buffer)?),
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run<'b>(self, client: &'a mut Client<'b>) -> RequesterRunResult<'a>
    where
        'b: 'a,
    {
        match self {
            Self::Account(req) => req.run(client),
        }
    }
}
