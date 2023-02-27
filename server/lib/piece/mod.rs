use std::fmt::Debug;

use crate::{
    board::Board,
    logic::{Coordinate, Move},
};

pub trait Piece
where
    Self: Debug,
{
    /// Whether the piece can be taken
    fn takeable(&self) -> bool {
        true
    }
    /// The number of points gained capturing this piece
    fn capture_points(&self) -> u8;
    /// Whether this piece's attacks can be blocked
    /// (determines whether the attacks need to be recalculated when blocking)
    fn blockable(&self) -> bool;
    /// Whether this piece can be castled by a king (for Castling)
    fn castleable(&self) -> bool {
        false
    }

    /// The number of times a piece has moved (includes being castled)
    fn moves(&self) -> u16;
    /// The move number where the pawn took its first move of two places and its direction,
    /// if it is a pawn and those conditions apply (for En Passant)
    fn pawn_first_move(&self) -> Option<(u16, i8)> {
        None
    }
    /// Add the piece's attacks to the board
    fn add_attacks(&mut self, board: &mut Board, piece_id: usize, from: Coordinate);

    /// Validates a move: 'from' -> 'to'
    fn is_valid_move(
        &self,
        target: Option<&(u8, Box<dyn Piece>)>,
        board: &Board,
        r#move: &Move,
        to: &Coordinate,
    ) -> bool;
    #[allow(unused)]
    /// Run while the piece is being moved: 'from' -> 'to'
    fn mid_move(
        &mut self,
        board: &mut Board,
        r#move: &Move,
        to: &Coordinate,
    ) -> Option<Box<dyn Piece>> {
        None
    }
}

mod pawn;
pub use pawn::*;
