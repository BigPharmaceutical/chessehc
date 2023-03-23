use super::{Responder, Response};

pub mod in_game;
pub mod inval_req;
pub mod mal_req;

use self::{in_game::InGame, inval_req::InvalidRequest, mal_req::MalformedRequest};
pub enum Error {
    IG(InGame),
    InvalReq(InvalidRequest),
    MalReq(MalformedRequest),
}

impl Responder for Error {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::IG(_) => 1,
            Self::InvalReq(_) => 2,
            Self::MalReq(_) => 3,
        } << 5;

        match self {
            Self::IG(err) => err.write(buffer),
            Self::InvalReq(err) => err.write(buffer),
            Self::MalReq(err) => err.write(buffer),
        }
    }
}

impl From<Error> for Response {
    fn from(value: Error) -> Self {
        Self::Err(value)
    }
}
