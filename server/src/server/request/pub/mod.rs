use crate::server::response::err::{mal_req::MalformedRequest, Error};

use super::Requester;

pub mod log_in;

use self::log_in::LogIn;

pub enum Public<'a> {
    Status,
    Profile,
    CreateAccount,
    LI(LogIn<'a>),
}

impl<'a> Requester<'a> for Public<'a> {
    fn parse(buffer: &'a [u8]) -> Result<Self, Error> {
        let byte_zero = buffer.first().ok_or(MalformedRequest::op_err())?;

        Ok(match (byte_zero >> 4) & 0b11 {
            0 => Self::Status,
            1 => Self::Profile,
            2 => Self::CreateAccount,
            3 => Self::LI(LogIn::parse(buffer)?),
            _ => return Err(MalformedRequest::op_err()),
        })
    }

    fn run(self, _client: &mut crate::server::handler::Client) {
        todo!()
    }
}
