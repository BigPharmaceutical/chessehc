use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::{logic::Move, piece::Piece};

pub type Coordinate = (usize, usize);

pub struct Board {
    players: u8,
    width: usize,
    height: usize,
    board: Vec<Vec<Option<Box<dyn Piece>>>>,
    turn: u16,
}

impl Board {
    #[must_use]
    /// Create a new board
    pub fn new(players: u8, width: usize, height: usize) -> Self {
        Self {
            players,
            width,
            height,
            board: std::iter::repeat_with(|| std::iter::repeat_with(|| None).take(width).collect())
                .take(height)
                .collect(),
            turn: 0,
        }
    }

    #[must_use]
    /// Get the number of players
    pub const fn players(&self) -> u8 {
        self.players
    }

    #[must_use]
    /// Get the current turn number
    pub const fn turn(&self) -> u16 {
        self.turn
    }

    /// Get the entry at a coordinate
    pub fn get_position(
        &self,
        coordinate: Coordinate,
    ) -> Result<&Option<Box<dyn Piece>>, MoveError> {
        let Some(Some(piece)) = self.board.get(coordinate.1).map(|row| row.get(coordinate.0)) else {
            return Err(MoveError::CoordinateOffBoard(coordinate, (self.width, self.height)));
        };

        Ok(piece)
    }

    /// Get the mutable entry at a coordinate
    pub fn get_position_mut(
        &mut self,
        coordinate: Coordinate,
    ) -> Result<&mut Option<Box<dyn Piece>>, MoveError> {
        let Some(Some(piece)) = self.board.get_mut(coordinate.1).map(|row| row.get_mut(coordinate.0)) else {
            return Err(MoveError::CoordinateOffBoard(coordinate, (self.width, self.height)));
        };

        Ok(piece)
    }

    /// Validate a move on the board
    pub fn is_valid_move(&self, r#move: &Move) -> Result<bool, MoveError> {
        // Get the entry to be moved, and make sure there's a piece there
        let piece = self.get_position(r#move.from)?;

        let Some(piece) = piece else {
            return Err(MoveError::NoPiece(r#move.from));
        };

        // Get the target entry, to save doing it in the piece's logic
        let target = self.get_position(r#move.to)?;

        // If there is a piece at the target and it is not takeable, return false
        if let Some(target) = target {
            if !target.takeable() {
                return Ok(false);
            }
        }

        // Get the piece to validate the move
        Ok(piece.is_valid_move(target, self, r#move))
    }

    /// Perform a move on the board
    pub fn perform_move(&mut self, r#move: &Move) -> Result<u8, MoveError> {
        // If the move is not valid, return an error
        if !self.is_valid_move(r#move)? {
            return Err(MoveError::InvalidMove(*r#move));
        }

        // Remove the target from the board
        let target = self.get_position_mut(r#move.to)?;
        let taken_piece = target.take();

        // Remove the piece from the board
        let piece = self.get_position_mut(r#move.from)?.take();

        // Run the piece's mid_move logic
        if let Some(mut piece) = piece {
            piece.mid_move(self, r#move);
        }

        self.turn += 1;

        // Return the points gained
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
        use MoveError::{CoordinateOffBoard, InvalidMove, NoPiece};
        match *self {
            CoordinateOffBoard(coordinate, size) => writeln!(
                f,
                "coordinate {coordinate:?} off the board of dimensions {size:?}"
            ),
            NoPiece(coordinate) => writeln!(f, "no piece at {coordinate:?}"),
            InvalidMove(r#move) => writeln!(f, "move is not valid: {move:?}"),
        }
    }
}

impl Error for MoveError {}

#[allow(non_upper_case_globals)]
pub const new: &dyn Fn(u8, usize, usize) -> Board = &Board::new;
