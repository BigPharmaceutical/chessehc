use crate::{board::Board, logic::Move};

use super::Piece;

pub struct Pawn(u16, Option<u16>);

impl Piece for Pawn {
    fn capture_points(&self) -> u8 {
        1
    }

    fn moves(&self) -> u16 {
        self.0
    }

    fn pawn_first_move(&self) -> Option<u16> {
        self.1
    }

    fn is_valid_move(&self, target: &Option<Box<dyn Piece>>, board: &Board, r#move: &Move) -> bool {
        todo!()
    }

    fn mid_move(&mut self, board: &mut Board, r#move: &Move) -> Option<Box<dyn Piece>> {
        self.0 += 1;

        todo!()
    }
}
