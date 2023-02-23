use crate::{board::Board, logic::Move};

pub trait Piece {
    /// The number of points gained capturing this piece
    fn capture_points(&self) -> u8;
    /// Whether this piece can be castled by a king (for Castling)
    fn castleable(&self) -> bool {
        false
    }
    /// Whether the piece can be taken
    fn takeable(&self) -> bool {
        true
    }

    /// The number of times a piece has moved (includes being castled)
    fn moves(&self) -> u16;
    /// The move number where the pawn took its first move of two places, if it is a pawn and those conditions apply (for En Passant)
    fn pawn_first_move(&self) -> Option<u16> {
        None
    }

    /// Validates a move: 'from' -> 'to'
    fn is_valid_move(
        &self,
        target: &Option<(u8, Box<dyn Piece>)>,
        board: &Board,
        r#move: &Move,
    ) -> bool;
    #[allow(unused)]
    /// Run while the piece is being moved: 'from' -> 'to'
    fn mid_move(&mut self, board: &mut Board, r#move: &Move) -> Option<Box<dyn Piece>> {
        None
    }
}

mod pawn;
pub use pawn::*;
