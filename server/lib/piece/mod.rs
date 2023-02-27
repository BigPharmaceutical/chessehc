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

    /// Checks if the piece is attacking a coordinate
    fn is_attacking(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool;

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
    fn moves(&self) -> u16 {
        0
    }
    fn is_attacking(&self, _board: &Board, _from: &Coordinate, _to: &Coordinate) -> bool {
        false
    }
    fn is_valid_move(
        &self,
        _target: Option<&(u8, Box<dyn Piece>)>,
        _board: &Board,
        _move: &Move,
        _to: &Coordinate,
    ) -> bool {
        false
    }
}

mod bishop;
mod pawn;
pub use bishop::*;
pub use pawn::*;
