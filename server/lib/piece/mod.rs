use std::fmt::Debug;

use crate::{
    board::Board,
    logic::{Coordinate, Move},
};

mod pieces;
pub use pieces::*;

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
    fn pawn_first_move(&self) -> Option<(u16, Coordinate)> {
        None
    }

    /// Add the piece's attacks to the board
    fn add_attacks(&self, board: &mut Board, piece_id: usize, from: Coordinate);

    /// Validates a move: 'from' -> 'to'
    fn is_valid_move(
        &self,
        board: &Board,
        pieces: &Pieces,
        target: Option<&(u8, Box<dyn Piece>)>,
        r#move: &Move,
        to: &Coordinate,
    ) -> bool;
    #[allow(unused)]
    /// Increment moves
    fn increment_moves(&mut self);
    /// Run while the piece is being moved: 'from' -> 'to'
    fn mid_move(
        &mut self,
        _board: &mut Board,
        _pieces: &mut Pieces,
        _move: &Move,
        _to: &Coordinate,
    ) -> (u8, Option<Box<dyn Piece>>) {
        (0, None)
    }
}

#[derive(Debug)]
pub struct Dummy;
impl Piece for Dummy {
    fn capture_points(&self) -> u8 {
        0
    }
    fn blockable(&self) -> bool {
        false
    }
    fn moves(&self) -> u16 {
        0
    }
    fn add_attacks(&self, _board: &mut Board, _piece_id: usize, _from: Coordinate) {}
    fn is_valid_move(
        &self,
        _board: &Board,
        _pieces: &Pieces,
        _target: Option<&(u8, Box<dyn Piece>)>,
        _move: &Move,
        _to: &Coordinate,
    ) -> bool {
        false
    }
    fn increment_moves(&mut self) {}
}

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
pub use bishop::*;
pub use king::*;
pub use knight::*;
pub use pawn::*;
pub use queen::*;
pub use rook::*;
