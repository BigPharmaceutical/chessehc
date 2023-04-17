use crate::response::Responder;

use self::{board::Board, game::Game};

pub mod board;
pub mod game;

pub enum InGame {
    Game(Game),
    Board(Board),
}

impl Responder for InGame {
    fn write(self, buffer: &mut Vec<u8>) {
        let Some(byte_zero) = buffer.get_mut(0) else { return };

        *byte_zero |= match &self {
            Self::Game(_) => 0,
            Self::Board(_) => 1,
        } << 4;

        match self {
            Self::Game(res) => res.write(buffer),
            Self::Board(res) => res.write(buffer),
        }
    }
}
