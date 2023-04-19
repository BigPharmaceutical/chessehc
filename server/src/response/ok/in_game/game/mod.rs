use crate::response::Responder;

use self::{players::Players, status::Status};

pub mod players;
pub mod status;

pub enum Game<'a> {
    Code(String),
    Players(Players),
    Status(Status<'a>),
}

impl<'a> Responder for Game<'a> {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::Code(_) => 0,
            Self::Players(_) => 1,
            Self::Status(_) => 2,
        } << 2;

        match self {
            Self::Code(code) => buffer.extend_from_slice(code.as_bytes()),
            Self::Players(res) => res.write(buffer),
            Self::Status(res) => res.write(buffer),
        }
    }
}
