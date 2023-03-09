use std::{error, fmt};

use crate::{coordinate::Coordinate, piece_set::PieceSet, r#move::Move};

#[derive(Debug)]
pub enum Error<Set: PieceSet> {
    CoordinateNotOnBoard(Coordinate, usize, usize),
    NoPieceAtSpot(Coordinate),
    SpotOccupied(Coordinate, Option<Set>),
    PieceOwnedByWrongPlayer(Coordinate, u8),
    PieceNotCapturable(Coordinate),
    InvalidMove(Move),
    PieceError(Set::Error),
}

impl<Set: PieceSet> fmt::Display for Error<Set> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::{
            CoordinateNotOnBoard, InvalidMove, NoPieceAtSpot, PieceError, PieceNotCapturable,
            PieceOwnedByWrongPlayer, SpotOccupied,
        };
        match self {
            CoordinateNotOnBoard(coordinate, width, height) => write!(
                f,
                "{coordinate}: Coordinate not on board! width: {width}, height: {height}"
            ),
            NoPieceAtSpot(position) => write!(f, "{position}: No piece at spot!"),
            SpotOccupied(position, _) => write!(f, "{position}: Spot already occupied!"),
            PieceOwnedByWrongPlayer(position, player) => {
                write!(f, "{position}: Piece at spot belongs to {player}!")
            }
            PieceNotCapturable(position) => {
                write!(f, "{position}: Piece at spot is not capturable!")
            }
            InvalidMove(r#move) => write!(f, "Move is invalid: {move:?}"),
            PieceError(error) => write!(f, "Piece error: {error}"),
        }
    }
}

impl<Set: PieceSet> error::Error for Error<Set> {
    fn cause(&self) -> Option<&dyn error::Error> {
        use Error::PieceError;
        match self {
            PieceError(err) => Some(err),
            _ => None,
        }
    }
}
