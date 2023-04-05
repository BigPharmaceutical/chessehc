use crate::{
    response::{err::mal_req::MalformedRequest, Result},
    server::handler::Client,
};

use super::{Requester, RequesterRunResult};

pub enum InGame {
    Game,
    Board,
    Manage,
    Leave,
}

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

    fn run<'b>(self, _client: &'a mut Client<'b>) -> RequesterRunResult<'a>
    where
        'b: 'a,
    {
        todo!()
    }
}
