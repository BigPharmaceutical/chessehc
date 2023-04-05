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
        todo!()
    }
}
