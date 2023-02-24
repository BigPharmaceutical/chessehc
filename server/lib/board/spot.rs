use std::{error, fmt::Display};

use super::Board;

#[derive(Clone, Default)]
pub struct Spot {
    piece: Option<usize>,
    attackers: Vec<usize>,
}

impl Spot {
    pub const fn is_occupied(&self) -> bool {
        self.piece.is_some()
    }

    pub const fn get(&self) -> Option<usize> {
        self.piece
    }

    pub fn attacking(&self, player: u8, board: &Board) -> usize {
        self.attackers(player, board).len()
    }

    pub fn attackers(&self, player: u8, board: &Board) -> Vec<&usize> {
        self.attackers
            .iter()
            .filter(|&&attacker| {
                board
                    .get_piece(attacker)
                    .expect("failed to get an attacking piece")
                    .0
                    != player
            })
            .collect()
    }

    pub fn take(&mut self) -> Option<usize> {
        self.piece.take()
    }

    pub fn place(&mut self, piece: usize) -> Result<(), Error> {
        if self.piece.is_some() {
            return Err(Error::SpotOccupied);
        }

        self.piece = Some(piece);
        Ok(())
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

    pub fn unattack(&mut self, piece: usize) {
        // if let Ok(i) = self.attacking.binary_search(&piece) {
        //     self.attacking.remove(i);
        // }

        if let Some(i) = self
            .attackers
            .iter()
            .position(|&attacker| attacker == piece)
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
