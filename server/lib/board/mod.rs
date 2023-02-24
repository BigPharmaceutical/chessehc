use std::fmt::Display;

use crate::{piece::Piece, logic::Coordinate};

mod spot;

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
    #[must_use]
    pub fn new(players: u8, width: i16, height: i16) -> Self {
        Self {
            players,
            width,
            height,
            pieces: Vec::new(),
            turn: 0,
            board: vec![vec![Spot::default(); width as usize]; height as usize]
        }
    }

    pub fn add_piece(&mut self, player: u8, piece: Box<dyn Piece>, coordinate: Coordinate) -> Result<(), Error> {
        let piece_id = self.pieces.len();
        self.pieces.push((player, piece));
        self.get_mut(coordinate)?.place(piece_id).map_err(|err| Error::from_spot_error(err, coordinate))
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
    pub const fn turn(&self) -> u16 {
        self.turn
    }

    pub fn get(&self, coordinate: Coordinate) -> Result<&Spot, Error> {
        match self.board.get(coordinate.1 as usize).map(|rank| rank.get(coordinate.0 as usize)) {
            Some(Some(spot)) => Ok(spot),
            _ => Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        }
    }

    pub fn get_mut(&mut self, coordinate: Coordinate) -> Result<&mut Spot, Error> {
        match self.board.get_mut(coordinate.1 as usize).map(|rank| rank.get_mut(coordinate.0 as usize)) {
            Some(Some(spot)) => Ok(spot),
            _ => Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        }
    }

    pub fn get_piece(&self, piece_id: usize) -> Result<&(u8, Box<dyn Piece>), Error> {
        self.pieces.get(piece_id).ok_or(Error::PieceDoesNotExist(piece_id))
    }

    pub fn attack(&mut self, coordinate: Coordinate, piece: usize) -> Result<(), Error> {
        self.get_mut(coordinate)?.attack(piece);
        Ok(())
    }

    pub fn unattack(&mut self, coordinate: Coordinate, piece: usize) -> Result<(), Error> {
        self.get_mut(coordinate)?.unattack(piece);
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    CoordinateNotOnBoard(Coordinate, i16, i16),
    SpotError(spot::Error, Coordinate),
    PieceDoesNotExist(usize),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoordinateNotOnBoard(coordinate, width, height) => write!(f, "Coordinate {coordinate} not on board of dimensions ({width}, {height})!"),
            Self::SpotError(err, coordinate) => write!(f, "Spot {coordinate}: {err}"),
            Self::PieceDoesNotExist(piece_id) => write!(f, "The piece {piece_id} does not exist!"),
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
