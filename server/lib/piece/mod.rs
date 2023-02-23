use crate::{board::{Board, Coordinate, RawBoard}, logic::Move};

pub trait Piece {
    /// The number of points gained capturing this piece
    fn capture_points(&self) -> u8;
    /// Whether this piece can be castled by a king (for Castling)
    fn castleable(&self) -> bool {
        false
    }

    /// The number of times a piece has moved (includes being castled)
    fn moves(&self) -> u16;
    /// The number of places moved in the first turn or zero if no move has been made, if this is a pawn (for En Passant)
    fn pawn_first_move(&self) -> Option<u8> {
        None
    }

    /// Validates a move: 'from' -> 'to'
    fn is_valid_move(&self, target: Option<&dyn Piece>, board: &Board, from: Coordinate, to: Coordinate) -> bool;
    #[allow(unused)]
    /// Run while the piece is being moved: 'from' -> 'to'
    fn mid_move(
        &mut self,
        board: RawBoard,
        r#move: &Move
    ) -> Option<Box<dyn Piece>> {
        None
    }
}

mod pawn;
pub use pawn::*;
