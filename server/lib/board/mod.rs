use std::{fmt::Display, num::TryFromIntError};

use crate::{
    logic::{Coordinate, Move},
    piece::Piece,
};

pub mod spot;

use spot::Spot;

pub struct Board {
    players: u8,
    width: i16,
    height: i16,
    pieces: Vec<(u8, Box<dyn Piece>)>,
    turn: u16,
    board: Vec<Vec<Spot>>,
}

impl Board {
    pub fn new(players: u8, width: usize, height: usize) -> Result<Self, TryFromIntError> {
        Ok(Self {
            players,
            width: i16::try_from(width)?,
            height: i16::try_from(height)?,
            pieces: Vec::new(),
            turn: 0,
            board: vec![vec![Spot::default(); width]; height],
        })
    }

    pub fn add_piece(
        &mut self,
        player: u8,
        piece: Box<dyn Piece>,
        coordinate: Coordinate,
    ) -> Result<(), Error> {
        let piece_id = self.pieces.len();
        self.pieces.push((player, piece));
        self.get_mut(coordinate)?
            .place(piece_id)
            .map_err(|err| Error::from_spot_error(err, coordinate))
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

    pub fn get(&self, coordinate: Coordinate) -> Result<&Spot, Error> {
        let (Ok(x), Ok(y)) = (usize::try_from(coordinate.0), usize::try_from(coordinate.1)) else {
            return Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        };

        match self.board.get(y).map(|rank| rank.get(x)) {
            Some(Some(spot)) => Ok(spot),
            _ => Err(Error::CoordinateNotOnBoard(
                coordinate,
                self.width,
                self.height,
            )),
        }
    }

    pub fn get_mut(&mut self, coordinate: Coordinate) -> Result<&mut Spot, Error> {
        let (Ok(x), Ok(y)) = (usize::try_from(coordinate.0), usize::try_from(coordinate.1)) else {
            return Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        };

        match self.board.get_mut(y).map(|rank| rank.get_mut(x)) {
            Some(Some(spot)) => Ok(spot),
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

    pub fn attack(&mut self, coordinate: Coordinate, piece: usize) -> Result<(), Error> {
        self.get_mut(coordinate)?.attack(piece);
        Ok(())
    }

    pub fn unattack(&mut self, coordinate: Coordinate, piece: usize) -> Result<(), Error> {
        self.get_mut(coordinate)?.unattack(piece);
        Ok(())
    }

    pub fn remove_attacks(&mut self, piece_id: usize) {
        for rank in &mut self.board {
            for spot in rank {
                spot.unattack(piece_id);
            }
        }
    }

    pub fn is_valid_move(&self, r#move: Move) -> Result<bool, Error> {
        let to = r#move.from.add(&r#move.delta, self);
        let target = self.get(to)?.get(self);
        let Some(piece) = self.get(r#move.from)?.get(self) else {
            return Err(Error::NoPieceAtSpot(r#move.from));
        };
        Ok(piece.1.is_valid_move(target, self, &r#move, &to))
    }

    pub fn make_move(&mut self, r#move: Move) -> Result<(), Error> {
        let to = r#move.from.add(&r#move.delta, self);
        let target = self.get(to)?.get(self);
        let Some(piece) = self.get(r#move.from)?.get(self) else {
            return Err(Error::NoPieceAtSpot(r#move.from));
        };
        if !piece.1.is_valid_move(target, self, &r#move, &to) {
            return Err(Error::InvalidMove(r#move));
        }

        let spot = self.get_mut(r#move.from)?;
        let piece_id = spot
            .take()
            .expect("no piece at spot when taking to make move");

        self.get_piece_mut(piece_id)
            .expect("invalid piece id when taking move")
            .1
            .mid_move(self, &r#move, &to);

        let spot = self.get_mut(to)?;
        let taken = spot.take();
        if let Err(err) = spot.place(piece_id) {
            return Err(Error::from_spot_error(err, to));
        };

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    CoordinateNotOnBoard(Coordinate, i16, i16),
    SpotError(spot::Error, Coordinate),
    NoPieceAtSpot(Coordinate),
    PieceDoesNotExist(usize),
    InvalidMove(Move),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoordinateNotOnBoard(coordinate, width, height) => write!(
                f,
                "Coordinate {coordinate} not on board of dimensions ({width}, {height})!"
            ),
            Self::SpotError(err, coordinate) => write!(f, "Spot {coordinate}: {err}"),
            Self::NoPieceAtSpot(coordinate) => write!(f, "No piece at {coordinate}"),
            Self::PieceDoesNotExist(piece_id) => write!(f, "The piece {piece_id} does not exist!"),
            Self::InvalidMove(r#move) => write!(f, "Move {move} is not valid"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SpotError(err, _) => Some(err),
            _ => None,
        }
    }
}

impl Error {
    #[must_use]
    pub const fn from_spot_error(value: spot::Error, coordinate: Coordinate) -> Self {
        Self::SpotError(value, coordinate)
    }
}
