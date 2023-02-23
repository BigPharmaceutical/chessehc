use std::{
    error::Error,
    fmt::{self, Display}, mem,
};

use crate::{piece::Piece, logic::Move};

pub type Coordinate = (usize, usize);

pub struct Board {
    width: usize,
    height: usize,
    board: Vec<Vec<Option<Box<dyn Piece>>>>,
}

pub type RawBoard<'a> = (&'a mut Vec<Vec<Option<Box<dyn Piece>>>>, Coordinate);

impl Board {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            board: std::iter::repeat_with(|| std::iter::repeat_with(|| None).take(width).collect())
                .take(height)
                .collect(),
        }
    }

    pub fn get_position(&self, coordinate: Coordinate) -> Result<&Option<Box<dyn Piece>>, MoveError> {
        let Some(Some(piece)) = self.board.get(coordinate.1).map(|row| row.get(coordinate.0)) else {
            return Err(MoveError::CoordinateOffBoard(coordinate, (self.width, self.height)));
        };

        Ok(piece)
    }

    pub fn get_position_mut(&mut self, coordinate: Coordinate) -> Result<&mut Option<Box<dyn Piece>>, MoveError> {
        let Some(Some(piece)) = self.board.get_mut(coordinate.1).map(|row| row.get_mut(coordinate.0)) else {
            return Err(MoveError::CoordinateOffBoard(coordinate, (self.width, self.height)));
        };

        Ok(piece)
    }

    pub fn is_valid_move(&self, r#move: &Move) -> Result<bool, MoveError> {
        let piece = self.get_position(r#move.from)?;

        let Some(piece) = piece else {
            return Err(MoveError::NoPiece(r#move.from));
        };

        let target = self.get_position(r#move.to)?;

        todo!()
    }

    pub fn perform_move(&mut self, r#move: &Move) -> Result<u8, MoveError> {
        if !self.is_valid_move(r#move)? {
            return Err(MoveError::InvalidMove(*r#move));
        }
        
        let target = self.get_position_mut(r#move.to)?;
        let taken_piece = mem::replace(target, None);
        
        let piece = self.get_position_mut(r#move.from)?;
        let piece = mem::replace(piece, None);

        if let Some(mut piece) = piece {
            piece.mid_move((&mut self.board, (self.width, self.height)), r#move);
        }

        Ok(taken_piece.map_or(0, |taken_piece| taken_piece.capture_points()))
    }
}

#[derive(Debug)]
pub enum MoveError {
    CoordinateOffBoard(Coordinate, Coordinate),
    NoPiece(Coordinate),
    InvalidMove(Move),
}

impl Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MoveError::{CoordinateOffBoard, NoPiece, InvalidMove};
        match *self {
            CoordinateOffBoard(coordinate, size) => writeln!(f, "coordinate {coordinate:?} off the board of dimensions {size:?}"),
            NoPiece(coordinate) => writeln!(f, "no piece at {coordinate:?}"),
            InvalidMove(r#move) => writeln!(f, "move is not valid: {:?}", r#move),
        }
    }
}

impl Error for MoveError {}

#[allow(non_upper_case_globals)]
pub const new: &dyn Fn(usize, usize) -> Board = &Board::new;
