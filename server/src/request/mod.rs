use std::pin::Pin;

use futures_util::Future;

use super::{response::Result, server::handler::Client};
use crate::response::err::mal_req::MalformedRequest;

pub mod acc;
pub mod game;
pub mod ig;
pub mod r#pub;

use self::{acc::Account, game::Game, ig::InGame, r#pub::Public};

pub type RequesterRunResult<'a> = Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

pub trait Requester<'a>
where
    Self: Sized + 'a,
{
    /// Attempts to parse the buffer as a request
    ///
    /// # Errors
    /// [`crate::response::err::Error`]
    fn parse(buffer: &'a [u8]) -> Result<Self>;

    fn run<'b>(self, client: &'a mut Client<'b>) -> RequesterRunResult<'a>
    where
        'b: 'a;
}

pub enum Request<'a> {
    Pub(Public<'a>),
    Acc(Account<'a>),
    Game(Game),
    IG(InGame),
}

impl<'a> Requester<'a> for Request<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 6) & 0b11 {
            0 => Self::Pub(Public::parse(buffer)?),
            1 => Self::Acc(Account::parse(buffer)?),
            2 => Self::Game(Game::parse(buffer)?),
            3 => Self::IG(InGame::parse(buffer)?),
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run<'b>(self, client: &'a mut Client<'b>) -> RequesterRunResult<'a>
    where
        'b: 'a,
    {
        match self {
            Self::Pub(req) => req.run(client),
            Self::Acc(req) => req.run(client),
            Self::Game(req) => req.run(client),
            Self::IG(req) => req.run(client),
        }
    }
}
