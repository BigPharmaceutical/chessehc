use crate::response::Responder;

use self::players::Players;

pub mod players;

pub enum Game {
    GameCode(String),
    Players(Players),
}

impl Responder for Game {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::GameCode(_) => 0,
            Self::Players(_) => 1,
        } << 2;

        match self {
            Self::GameCode(code) => buffer.extend_from_slice(code.as_bytes()),
            Self::Players(res) => res.write(buffer),
        }
    }
}
