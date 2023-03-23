use crate::server::response::err::{mal_req::MalformedRequest, Error};

use super::Requester;

pub enum InGame {
    Game,
    Board,
    Manage,
    Leave,
}

impl<'a> Requester<'a> for InGame {
    fn parse(buffer: &[u8]) -> Result<Self, Error> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 4) & 0b11 {
            0 => Self::Game,
            1 => Self::Board,
            2 => Self::Manage,
            3 => Self::Leave,
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run(self, _client: &mut crate::server::handler::Client) {
        todo!()
    }
}
