use async_trait::async_trait;

use super::{handler::Client, response::Result};
use crate::server::response::err::mal_req::MalformedRequest;

pub mod acc;
pub mod game;
pub mod ig;
pub mod r#pub;

use self::{acc::Account, game::Game, ig::InGame, r#pub::Public};

#[async_trait]
pub trait Requester<'a>
where
    Self: Sized + 'a,
{
    fn parse(buffer: &'a [u8]) -> Result<Self>;

    async fn run<'b>(self, client: &'a mut Client<'b>) -> Result<()>
    where
        'b: 'a;
}

pub enum Request<'a> {
    Pub(Public<'a>),
    Acc(Account),
    Game(Game),
    IG(InGame),
}

#[async_trait]
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

    async fn run<'b>(self, _client: &'a mut Client<'b>) -> Result<()>
    where
        'b: 'a,
    {
        todo!()
    }
}
