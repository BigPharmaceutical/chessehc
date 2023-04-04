use async_trait::async_trait;

use crate::server::{
    handler::Client,
    response::{
        err::mal_req::MalformedRequest,
        Result,
    },
};

use super::Requester;

pub enum InGame {
    Game,
    Board,
    Manage,
    Leave,
}

#[async_trait]
impl<'a> Requester<'a> for InGame {
    fn parse(buffer: &[u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 4) & 0b11 {
            0 => Self::Game,
            1 => Self::Board,
            2 => Self::Manage,
            3 => Self::Leave,
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
