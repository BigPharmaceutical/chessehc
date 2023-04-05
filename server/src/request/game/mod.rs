use crate::{
    response::{err::mal_req::MalformedRequest, Result},
    server::handler::Client,
};

use super::{Requester, RequesterRunResult};

pub enum Game {
    Create,
    Join,
}

impl<'a> Requester<'a> for Game {
    fn parse(buffer: &[u8]) -> Result<Self> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 5) & 0b1 {
            0 => Self::Create,
            1 => Self::Join,
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
