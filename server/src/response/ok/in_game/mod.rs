use crate::response::Responder;

use self::{board::Board, game::Game};

pub mod board;
pub mod game;

pub enum InGame<'a> {
    Game(Game<'a>),
    Board(Board),
}

impl<'a> Responder for InGame<'a> {
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
