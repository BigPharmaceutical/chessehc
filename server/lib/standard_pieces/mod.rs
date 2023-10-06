#![allow(clippy::new_ret_no_self)]

use std::{
    error,
    fmt::{Debug, Display},
    num::TryFromIntError,
};

use crate::{
    board::Board,
    coordinate::{Coordinate, CoordinateDelta},
    delta,
    piece_set::PieceSet,
    r#move::Move,
};

#[cfg(not(feature = "standard_pieces_send"))]
pub type StandardCompatiblePieceSet = Box<dyn StandardCompatiblePiece>;
#[cfg(feature = "standard_pieces_send")]
pub type StandardCompatiblePieceSet = Box<dyn StandardCompatiblePiece + Send>;
pub type Delta = delta::Delta<StandardCompatiblePieceSet>;

pub trait StandardCompatiblePiece
where
    Self: Debug,
{
    fn type_id(&self) -> u8;

    fn capture_points(&self) -> Option<u16>;

    fn blockable(&self) -> bool;

    fn player(&self) -> u8;

    /// If the piece can be in check, return whether it is in check, otherwise return None.
    ///
    /// # Errors
    /// [`Error`]
    fn is_in_check(
        &self,
        _board: &Board<StandardCompatiblePieceSet>,
        _position: &Coordinate,
    ) -> Result<Option<bool>, Error> {
        Ok(None)
    }

    /// Return the valid moves of a piece.
    /// Returns moves in the form `(to, data)`.
    ///
    /// # Errors
    /// [`Error`]
    fn valid_moves(
        &self,
        board: &Board<StandardCompatiblePieceSet>,
        from: &Coordinate,
        turn: u16,
        n_players: u8,
    ) -> Result<Vec<(Coordinate, u8)>, Error>;

    /// Returns the positions that a piece is attacking.
    ///
    /// # Errors
    /// [`Error`]
    fn attacking(
        &self,
        board: &Board<StandardCompatiblePieceSet>,
        from: &Coordinate,
    ) -> Result<Vec<Coordinate>, Error>;

    /// Method to be run mid-move
    ///
    /// # Errors
    /// [`Error`]
    fn mid_move(
        &mut self,
        board: &Board<StandardCompatiblePieceSet>,
        r#move: &Move,
        turn: u16,
        n_players: u8,
    ) -> Result<(Vec<Delta>, u16), Error>;

    // Custom
    fn clone(&self) -> StandardCompatiblePieceSet;

    fn can_en_passant(
        &self,
        _intermediate: &Coordinate,
        _turn: u16,
        _n_players_in_play: u8,
    ) -> bool {
        false
    }

    fn can_castle(&self) -> bool {
        false
    }

    fn mid_castle(&mut self) {}
}

impl Clone for StandardCompatiblePieceSet {
    fn clone(&self) -> Self {
        StandardCompatiblePiece::clone(&**self)
    }
}

// Map the methods
impl PieceSet for StandardCompatiblePieceSet {
    type Error = Error;
    type PieceId = u8;

    const NONE_ID: Self::PieceId = 0;

    fn type_id(&self) -> Self::PieceId {
        (**self).type_id()
    }

    fn capture_points(&self) -> Option<u16> {
        (**self).capture_points()
    }

    fn blockable(&self) -> bool {
        (**self).blockable()
    }

    fn player(&self) -> u8 {
        (**self).player()
    }

    fn is_in_check(
        &self,
        board: &Board<Self>,
        position: &Coordinate,
    ) -> Result<Option<bool>, Error> {
        (**self).is_in_check(board, position)
    }

    fn valid_moves(
        &self,
        board: &Board<Self>,
        from: &Coordinate,
        turn: u16,
        n_players: u8,
    ) -> Result<Vec<(Coordinate, u8)>, Error> {
        (**self).valid_moves(board, from, turn, n_players)
    }

    fn attacking(&self, board: &Board<Self>, from: &Coordinate) -> Result<Vec<Coordinate>, Error> {
        (**self).attacking(board, from)
    }

    fn mid_move(
        &mut self,
        board: &Board<Self>,
        r#move: &Move,
        turn: u16,
        n_players: u8,
    ) -> Result<(Vec<Delta>, u16), Self::Error> {
        (**self).mid_move(board, r#move, turn, n_players)
    }
}

#[derive(Debug)]
pub enum Error {
    PositionOrDeltaTooLarge(usize, TryFromIntError),
    BoardError(Box<crate::error::Error<StandardCompatiblePieceSet>>),
    IntermediatePositionNotOnBoard(Coordinate, CoordinateDelta),
    InvalidPieceId(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::{
            BoardError, IntermediatePositionNotOnBoard, InvalidPieceId, PositionOrDeltaTooLarge,
        };
        match self {
            PositionOrDeltaTooLarge(distance, err) => {
                write!(f, "Position or delta too large: {distance}!\n{err}")
            }
            BoardError(err) => write!(f, "Board: {err}"),
            IntermediatePositionNotOnBoard(from, delta) => write!(
                f,
                "Position between two valid positions should be on board: {from} + {delta}!"
            ),
            InvalidPieceId(id) => write!(f, "Invalid piece id: {id}!"),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        use Error::{BoardError, PositionOrDeltaTooLarge};
        match self {
            PositionOrDeltaTooLarge(_, err) => Some(err),
            BoardError(err) => Some(err),
            _ => None,
        }
    }
}

// Re-export the pieces
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
