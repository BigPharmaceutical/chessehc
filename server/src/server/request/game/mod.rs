use crate::server::response::err::{mal_req::MalformedRequest, Error};

use super::Requester;

pub enum Game {
    Create,
    Join,
}

impl<'a> Requester<'a> for Game {
    fn parse(buffer: &[u8]) -> Result<Self, Error> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 5) & 0b1 {
            0 => Self::Create,
            1 => Self::Join,
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run(self, _client: &mut crate::server::handler::Client) {
        todo!()
    }
}
