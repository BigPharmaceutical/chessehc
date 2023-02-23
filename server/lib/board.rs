use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::{
    logic::{Coordinate, Move},
    piece::Piece,
};

pub struct Board {
    players: u8,
    width: usize,
    height: usize,
    board: Vec<Vec<Option<(u8, Box<dyn Piece>)>>>,
    turn: u16,
    kings: Vec<Coordinate>,
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
            kings: Vec::new(),
        }
    }

    #[must_use]
    /// Get the number of players
    pub const fn players(&self) -> u8 {
        self.players
    }

    #[must_use]
    /// Get the width
    pub const fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    /// Get the height
    pub const fn height(&self) -> usize {
        self.height
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
    ) -> Result<&Option<(u8, Box<dyn Piece>)>, MoveError> {
        let Some(Some(piece)) = self.board.get(coordinate.1).map(|row| row.get(coordinate.0)) else {
            return Err(MoveError::CoordinateOffBoard(coordinate, self.width, self.height));
        };

        Ok(piece)
    }

    /// Get the mutable entry at a coordinate
    pub fn get_position_mut(
        &mut self,
        coordinate: Coordinate,
    ) -> Result<&mut Option<(u8, Box<dyn Piece>)>, MoveError> {
        let Some(Some(piece)) = self.board.get_mut(coordinate.1).map(|row| row.get_mut(coordinate.0)) else {
            return Err(MoveError::CoordinateOffBoard(coordinate, self.width, self.height));
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

        // Check that the player owns the piece
        if piece.0 != r#move.player {
            return Ok(false);
        }

        let to = r#move.from.add(&r#move.delta, self);

        // Get the target entry, to save doing it in the piece's logic
        let target = self.get_position(to)?;

        // If there is a piece at the target and it is not takeable, return false
        if let Some(target) = target {
            if !target.1.takeable() || target.0 == piece.0 {
                return Ok(false);
            }
        }

        // Get the piece to validate the move
        Ok(piece.1.is_valid_move(target, self, r#move))
    }

    /// Perform a move on the board
    pub fn perform_move(&mut self, r#move: &Move) -> Result<u8, MoveError> {
        // If the move is not valid, return an error
        if !self.is_valid_move(r#move)? {
            return Err(MoveError::InvalidMove(*r#move));
        }

        let to = r#move.from.add(&r#move.delta, self);

        // Remove the target from the board
        let target = self.get_position_mut(to)?;
        let taken_piece = target.take();

        // Remove the piece from the board
        let piece = self.get_position_mut(r#move.from)?.take();

        // Run the piece's mid_move logic
        if let Some(mut piece) = piece {
            piece.1.mid_move(self, r#move);
        }

        self.turn += 1;

        // Return the points gained
        Ok(taken_piece.map_or(0, |taken_piece| taken_piece.1.capture_points()))
    }
}

#[derive(Debug)]
pub enum MoveError {
    CoordinateOffBoard(Coordinate, usize, usize),
    NoPiece(Coordinate),
    InvalidMove(Move),
}

impl Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MoveError::{CoordinateOffBoard, InvalidMove, NoPiece};
        match *self {
            CoordinateOffBoard(coordinate, width, height) => writeln!(
                f,
                "coordinate {coordinate:?} off the board of dimensions ({width}, {height})"
            ),
            NoPiece(coordinate) => writeln!(f, "no piece at {coordinate:?}"),
            InvalidMove(r#move) => writeln!(f, "move is not valid: {move:?}"),
        }
    }
}

impl Error for MoveError {}

#[allow(non_upper_case_globals)]
pub const new: &dyn Fn(u8, usize, usize) -> Board = &Board::new;
