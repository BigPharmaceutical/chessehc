use std::{error, fmt::Display};

use crate::piece::{Piece, Pieces};

#[derive(Clone, Default, Debug)]
pub struct Spot {
    piece: Option<usize>,
    attackers: Vec<usize>,
}

impl Spot {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            piece: None,
            attackers: Vec::new(),
        }
    }
}

impl Spot {
    #[must_use]
    pub const fn is_occupied(&self) -> bool {
        self.piece.is_some()
    }

    #[must_use]
    pub fn get<'a>(&'a self, pieces: &'a Pieces) -> Option<&(u8, Box<dyn Piece>)> {
        Some(
            pieces
                .get(self.piece?)
                .expect("could not get piece from spot"),
        )
    }

    pub fn get_mut<'a>(&'a self, pieces: &'a mut Pieces) -> Option<&mut (u8, Box<dyn Piece>)> {
        Some(
            pieces
                .get_mut(self.piece?)
                .expect("could not get piece from spot"),
        )
    }

    #[must_use]
    pub const fn get_id(&self) -> &Option<usize> {
        &self.piece
    }

    pub fn get_id_mut(&mut self) -> &mut Option<usize> {
        &mut self.piece
    }

    #[must_use]
    pub fn attackers(&self, player: u8, pieces: &Pieces) -> Vec<&usize> {
        self.attackers
            .iter()
            .filter(|&&attacker| {
                pieces
                    .get(attacker)
                    .expect("failed to get an attacking piece")
                    .0
                    != player
            })
            .collect()
    }

    #[must_use]
    pub const fn all_attackers(&self) -> &Vec<usize> {
        &self.attackers
    }

    #[must_use]
    pub fn is_being_attacked(&self, pieces: &Pieces, player: u8) -> bool {
        for &attacker in &self.attackers {
            if pieces
                .get(attacker)
                .expect("failed to get an attacking piece")
                .0
                != player
            {
                return true;
            }
        }

        false
    }

    pub fn take(&mut self) -> Option<usize> {
        self.piece.take()
    }

    pub fn place(&mut self, piece_id: usize) -> Result<(), Error> {
        if self.piece.is_some() {
            return Err(Error::SpotOccupied);
        }

        self.piece = Some(piece_id);
        Ok(())
    }

    pub fn replace(&mut self, piece_id: usize) -> Option<usize> {
        self.piece.replace(piece_id)
    }

    pub fn attack(&mut self, piece: usize) {
        // // Binary search version, will most likely be slower due
        // // to small number of expected elements
        // if let Err(i) = self.attacking.binary_search(&piece) {
        //     self.attacking.insert(i, piece);
        // }

        if self.attackers.contains(&piece) {
            return;
        }

        self.attackers.push(piece);
    }

    pub fn unattack(&mut self, piece_id: usize) {
        // if let Ok(i) = self.attacking.binary_search(&piece) {
        //     self.attacking.remove(i);
        // }

        if let Some(i) = self
            .attackers
            .iter()
            .position(|&attacker| attacker == piece_id)
        {
            self.attackers.remove(i);
        }
    }
}

#[derive(Debug)]
pub enum Error {
    SpotOccupied,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpotOccupied => write!(f, "Spot already occupied!"),
        }
    }
}

impl error::Error for Error {}
