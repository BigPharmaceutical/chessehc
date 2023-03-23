use crate::server::response::err::{mal_req::MalformedRequest, Error};

use super::Requester;

pub enum Account {
    ChangeUsername,
    ChangeKey,
    Delete,
}

impl<'a> Requester<'a> for Account {
    fn parse(buffer: &[u8]) -> Result<Self, Error> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 4) & 0b11 {
            0 => Self::ChangeUsername,
            2 => Self::ChangeKey,
            3 => Self::Delete,
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run(self, _client: &mut crate::server::handler::Client) {
        todo!()
    }
}
