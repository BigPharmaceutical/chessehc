use crate::{board::{Board, Coordinate, RawBoard}, logic::Move};

use super::Piece;

pub struct Pawn(u16, u8);

impl Piece for Pawn {
    fn capture_points(&self) -> u8 {
        1
    }

    fn moves(&self) -> u16 {
        todo!()
    }

    fn pawn_first_move(&self) -> Option<u8> {
        todo!()
    }

    fn is_valid_move(&self, target: Option<&dyn Piece>, board: &Board, from: Coordinate, to: Coordinate) -> bool {
        todo!()
    }

    fn mid_move(
            &mut self,
            board: RawBoard,
            r#move: &Move
        ) -> Option<Box<dyn Piece>> {
        todo!()
    }
}
