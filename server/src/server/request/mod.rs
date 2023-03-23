use super::{handler::Client, response::err::Error};
use crate::server::response::err::mal_req::MalformedRequest;

pub mod acc;
pub mod game;
pub mod ig;
pub mod r#pub;

use self::{acc::Account, game::Game, ig::InGame, r#pub::Public};

pub trait Requester<'a>
where
    Self: Sized,
{
    fn parse(buffer: &'a [u8]) -> Result<Self, Error>;

    fn run(self, client: &mut Client);
}

pub enum Request<'a> {
    Pub(Public<'a>),
    Acc(Account),
    Game(Game),
    IG(InGame),
}

impl<'a> Requester<'a> for Request<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self, Error> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 6) & 0b11 {
            0 => Self::Pub(Public::parse(buffer)?),
            1 => Self::Acc(Account::parse(buffer)?),
            2 => Self::Game(Game::parse(buffer)?),
            3 => Self::IG(InGame::parse(buffer)?),
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run(self, _client: &mut Client) {
        todo!()
    }
}
