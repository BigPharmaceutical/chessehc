use std::{fmt::Display, mem, num::TryFromIntError};

use crate::{
    logic::{Coordinate, Move},
    piece::{self, Piece, Pieces},
};

mod spot;
pub use spot::*;

pub struct Board {
    players: u8,
    width: i16,
    height: i16,
    // pieces: Vec<(u8, Box<dyn Piece>)>,
    points: Vec<u16>,
    turn: u16,
    board: Vec<Vec<Spot>>,
    // board: Vec<Vec<Option<usize>>>,
}

impl Board {
    pub fn new(players: u8, width: usize, height: usize) -> Result<Self, TryFromIntError> {
        Ok(Self {
            players,
            width: i16::try_from(width)?,
            height: i16::try_from(height)?,
            // pieces: Vec::new(),
            points: vec![0; players.into()],
            turn: 0,
            board: vec![vec![Spot::new(); width]; height],
        })
    }

    pub fn add_piece(
        &mut self,
        pieces: &Pieces,
        piece_id: usize,
        coordinate: Coordinate,
    ) -> Result<(), Error> {
        let spot = self.get_spot_mut(coordinate)?;
        spot.place(piece_id)
            .map_err(|err| Error::from_spot_error(err, coordinate))?;
        pieces
            .get(piece_id)
            .unwrap()
            .1
            .add_attacks(self, piece_id, coordinate);
        let spot = self.get_spot(coordinate)?;
        for attacker_id in spot.all_attackers().clone() {
            let attacker = pieces.get(attacker_id).expect("could not get attacker");
            if attacker.1.blockable() {
                self.remove_attacks(attacker_id);
                if let Some(attacker_position) = self.find(attacker_id) {
                    attacker.1.add_attacks(self, attacker_id, attacker_position);
                }
            }
        }
        Ok(())
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

    pub fn get<'a>(
        &'a self,
        pieces: &'a Pieces,
        coordinate: Coordinate,
    ) -> Result<Option<&(u8, Box<dyn Piece>)>, Error> {
        let (Ok(x), Ok(y)) = (usize::try_from(coordinate.0), usize::try_from(coordinate.1)) else {
            return Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        };

        match self.board.get(y).map(|rank| rank.get(x)) {
            Some(Some(spot)) => Ok(spot.get(pieces)),
            _ => Err(Error::CoordinateNotOnBoard(
                coordinate,
                self.width,
                self.height,
            )),
        }
    }

    pub fn get_mut<'a>(
        &'a self,
        pieces: &'a mut Pieces,
        coordinate: Coordinate,
    ) -> Result<Option<&mut (u8, Box<dyn Piece>)>, Error> {
        let (Ok(x), Ok(y)) = (usize::try_from(coordinate.0), usize::try_from(coordinate.1)) else {
            return Err(Error::CoordinateNotOnBoard(coordinate, self.width, self.height))
        };

        match self.board.get(y).map(|rank| rank.get(x)) {
            Some(Some(spot)) => Ok(spot.get_mut(pieces)),
            _ => Err(Error::CoordinateNotOnBoard(
                coordinate,
                self.width,
                self.height,
            )),
        }
    }

    pub fn get_spot(&self, coordinate: Coordinate) -> Result<&Spot, Error> {
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

    pub fn get_spot_mut(&mut self, coordinate: Coordinate) -> Result<&mut Spot, Error> {
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

    #[must_use]
    pub fn find(&self, piece_id: usize) -> Option<Coordinate> {
        for (y, rank) in self.board.iter().enumerate() {
            for (x, spot) in rank.iter().enumerate() {
                match spot.get_id() {
                    Some(id) if *id == piece_id => {
                        return Some(Coordinate(
                            i16::try_from(x).unwrap(),
                            i16::try_from(y).unwrap(),
                        ))
                    }
                    _ => (),
                }
            }
        }
        None
    }

    pub fn attack(&mut self, coordinate: Coordinate, piece_id: usize) -> Result<(), Error> {
        self.get_spot_mut(coordinate)?.attack(piece_id);
        Ok(())
    }

    pub fn remove_attacks(&mut self, piece_id: usize) {
        for rank in &mut self.board {
            for spot in rank {
                spot.unattack(piece_id);
            }
        }
    }

    pub fn is_valid_move(&self, pieces: &Pieces, r#move: Move) -> Result<bool, Error> {
        if r#move.player >= self.players {
            return Err(Error::PlayerDoesNotExist(r#move.player));
        }

        let to = r#move.from.add(&r#move.delta, self);

        let Some(piece) = self.get(pieces, r#move.from)? else {
            return Err(Error::NoPieceAtSpot(r#move.from));
        };

        if piece.0 != r#move.player {
            return Ok(false);
        }

        let target = self.get(pieces, to)?;
        if let Some(target_piece) = target {
            if target_piece.0 == r#move.player || !target_piece.1.takeable() {
                return Ok(false);
            }
        }

        Ok(piece.1.is_valid_move(self, pieces, target, &r#move, &to))
    }

    pub fn make_move(&mut self, pieces: &mut Pieces, r#move: Move) -> Result<(), Error> {
        let to = r#move.from.add(&r#move.delta, self);

        if !self.is_valid_move(pieces, r#move)? {
            return Err(Error::InvalidMove(r#move));
        }

        let Some(piece_id) = self.get_spot_mut(r#move.from)?.take() else {
            return Err(Error::NoPieceAtSpot(r#move.from));
        };
        let mut piece = mem::replace(
            pieces.get_mut(piece_id)?,
            (r#move.player, Box::new(piece::Dummy)),
        );

        piece.1.mid_move(self, pieces, &r#move, &to);
        piece.1.increment_moves();

        let mut to_spot = self.get_spot_mut(to)?.replace(piece_id);
        if let Some(taken_id) = to_spot {
            let taken = pieces.get(taken_id)?;
            *self
                .points
                .get_mut(usize::from(r#move.player))
                .expect("could not get player points whilst making move") +=
                u16::from(taken.1.capture_points());
        }
        to_spot.replace(piece_id);

        *pieces.get_mut(piece_id)? = piece;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    CoordinateNotOnBoard(Coordinate, i16, i16),
    SpotError(spot::Error, Coordinate),
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
            Self::SpotError(err, coordinate) => write!(f, "Spot {coordinate}: {err}"),
            Self::NoPieceAtSpot(coordinate) => write!(f, "No piece at {coordinate}"),
            Self::PieceDoesNotExist(piece_id) => write!(f, "The piece {piece_id} does not exist!"),
            Self::PlayerDoesNotExist(player_id) => {
                write!(f, "The player {player_id} does not exist!")
            }
            Self::InvalidMove(r#move) => write!(f, "Move {move} is not valid"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::SpotError(err, _) = self {
            Some(err)
        } else {
            None
        }
    }
}

impl Error {
    #[must_use]
    pub const fn from_spot_error(value: spot::Error, coordinate: Coordinate) -> Self {
        Self::SpotError(value, coordinate)
    }
}
