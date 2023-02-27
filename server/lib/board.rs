use std::{fmt::Display, mem, num::TryFromIntError};

use crate::{
    logic::{Coordinate, Move},
    piece::{self, Piece},
};

pub struct Board {
    players: u8,
    width: i16,
    height: i16,
    pieces: Vec<(u8, Box<dyn Piece>)>,
    points: Vec<u16>,
    turn: u16,
    board: Vec<Vec<Option<usize>>>,
}

impl Board {
    pub fn new(players: u8, width: usize, height: usize) -> Result<Self, TryFromIntError> {
        Ok(Self {
            players,
            width: i16::try_from(width)?,
            height: i16::try_from(height)?,
            pieces: Vec::new(),
            points: vec![0; players.into()],
            turn: 0,
            board: vec![vec![None; width]; height],
        })
    }

    pub fn add_piece(
        &mut self,
        player: u8,
        piece: Box<dyn Piece>,
        coordinate: Coordinate,
    ) -> Result<usize, Error> {
        if player >= self.players {
            return Err(Error::PlayerDoesNotExist(player));
        }
        let piece_id = self.pieces.len();
        self.pieces.push((player, piece));
        Ok(*self.get_id_mut(coordinate)?.insert(piece_id))
    }

    #[must_use]
    pub const fn width(&self) -> i16 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> i16 {
        self.height
    }

    #[must_use]
    pub const fn players(&self) -> u8 {
        self.players
    }

    #[must_use]
    pub const fn turn(&self) -> u16 {
        self.turn
    }

    pub fn get(&self, coordinate: Coordinate) -> Result<Option<&(u8, Box<dyn Piece>)>, Error> {
        let (Ok(x), Ok(y)) = (usize::try_from(coordinate.0), usize::try_from(coordinate.1)) else {
            return Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        };

        match self.board.get(y).map(|rank| rank.get(x)) {
            Some(Some(&Some(piece_id))) => Ok(Some(self.get_piece(piece_id)?)),
            Some(Some(None)) => Ok(None),
            _ => Err(Error::CoordinateNotOnBoard(
                coordinate,
                self.width,
                self.height,
            )),
        }
    }

    pub fn get_mut(
        &mut self,
        coordinate: Coordinate,
    ) -> Result<Option<&mut (u8, Box<dyn Piece>)>, Error> {
        let (Ok(x), Ok(y)) = (usize::try_from(coordinate.0), usize::try_from(coordinate.1)) else {
            return Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        };

        match self.board.get(y).map(|rank| rank.get(x)) {
            Some(Some(&Some(piece_id))) => Ok(Some(self.get_piece_mut(piece_id)?)),
            Some(Some(None)) => Ok(None),
            _ => Err(Error::CoordinateNotOnBoard(
                coordinate,
                self.width,
                self.height,
            )),
        }
    }

    pub fn get_id(&self, coordinate: Coordinate) -> Result<&Option<usize>, Error> {
        let (Ok(x), Ok(y)) = (usize::try_from(coordinate.0), usize::try_from(coordinate.1)) else {
            return Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        };

        match self.board.get(y).map(|rank| rank.get(x)) {
            Some(Some(piece_id)) => Ok(piece_id),
            _ => Err(Error::CoordinateNotOnBoard(
                coordinate,
                self.width,
                self.height,
            )),
        }
    }

    pub fn get_id_mut(&mut self, coordinate: Coordinate) -> Result<&mut Option<usize>, Error> {
        let (Ok(x), Ok(y)) = (usize::try_from(coordinate.0), usize::try_from(coordinate.1)) else {
            return Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        };

        match self.board.get_mut(y).map(|rank| rank.get_mut(x)) {
            Some(Some(piece_id)) => Ok(piece_id),
            _ => Err(Error::CoordinateNotOnBoard(
                coordinate,
                self.width,
                self.height,
            )),
        }
    }

    pub fn get_piece(&self, piece_id: usize) -> Result<&(u8, Box<dyn Piece>), Error> {
        self.pieces
            .get(piece_id)
            .ok_or(Error::PieceDoesNotExist(piece_id))
    }

    pub fn get_piece_mut(&mut self, piece_id: usize) -> Result<&mut (u8, Box<dyn Piece>), Error> {
        self.pieces
            .get_mut(piece_id)
            .ok_or(Error::PieceDoesNotExist(piece_id))
    }

    pub fn is_valid_move(&self, r#move: Move) -> Result<bool, Error> {
        if r#move.player >= self.players {
            return Err(Error::PlayerDoesNotExist(r#move.player));
        }

        let to = r#move.from.add(&r#move.delta, self);

        let Some(piece) = self.get(r#move.from)? else {
            return Err(Error::NoPieceAtSpot(r#move.from));
        };

        if piece.0 != r#move.player {
            return Ok(false);
        }

        let target = self.get(to)?;
        if let Some(target_piece) = target {
            if target_piece.0 == r#move.player || !target_piece.1.takeable() {
                return Ok(false);
            }
        }

        Ok(piece.1.is_valid_move(target, self, &r#move, &to))
    }

    pub fn make_move(&mut self, r#move: Move) -> Result<(), Error> {
        let to = r#move.from.add(&r#move.delta, self);

        if !self.is_valid_move(r#move)? {
            return Err(Error::InvalidMove(r#move));
        }

        let Some(piece_id) = self.get_id_mut(r#move.from)?.take() else {
            return Err(Error::NoPieceAtSpot(r#move.from));
        };
        let mut piece = mem::replace(self.get_piece_mut(piece_id)?, (0, Box::new(piece::Dummy)));

        piece.1.mid_move(self, &r#move, &to);

        let mut to_spot = self.get_id_mut(to)?.replace(piece_id);
        if let Some(taken_id) = to_spot {
            let taken = self.get_piece(taken_id)?;
            *self
                .points
                .get_mut(usize::from(r#move.player))
                .expect("could not get player points whilst making move") +=
                u16::from(taken.1.capture_points());
        }
        to_spot.replace(piece_id);

        *self.get_piece_mut(piece_id)? = piece;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    CoordinateNotOnBoard(Coordinate, i16, i16),
    NoPieceAtSpot(Coordinate),
    PieceDoesNotExist(usize),
    PlayerDoesNotExist(u8),
    InvalidMove(Move),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoordinateNotOnBoard(coordinate, width, height) => write!(
                f,
                "Coordinate {coordinate} not on board of dimensions ({width}, {height})!"
            ),
            Self::NoPieceAtSpot(coordinate) => write!(f, "No piece at {coordinate}"),
            Self::PieceDoesNotExist(piece_id) => write!(f, "The piece {piece_id} does not exist!"),
            Self::PlayerDoesNotExist(player_id) => {
                write!(f, "The player {player_id} does not exist!")
            }
            Self::InvalidMove(r#move) => write!(f, "Move {move} is not valid"),
        }
    }
}

impl std::error::Error for Error {}
