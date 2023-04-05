use self::{in_game::InGame, public::Public};

use super::Responder;

pub mod in_game;
pub mod public;

pub enum Ok<'a> {
    Public(Public<'a>),
    Confirmation(u8),
    GameId(&'a str),
    InGame(InGame),
}

impl<'a> Responder for Ok<'a> {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::Public(_) => 0,
            Self::Confirmation(_) => 1,
            Self::GameId(_) => 2,
            Self::InGame(_) => 3,
        } << 5;

        match self {
            Self::Public(res) => res.write(buffer),
            Self::Confirmation(op_code) => buffer.push(op_code),
            Self::GameId(token) => buffer.extend_from_slice(token.as_bytes()),
            Self::InGame(res) => res.write(buffer),
        }
    }
}
