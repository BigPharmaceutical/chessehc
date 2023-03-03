/// Pieces was extracted from Board to try to fix some issues with borrows,
/// but this will probably be moved back into Board soon.
use crate::board::Error;

use super::Piece;

pub struct Pieces(Vec<(u8, Box<dyn Piece>)>);

impl Pieces {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn get(&self, piece_id: usize) -> Result<&(u8, Box<dyn Piece>), Error> {
        self.0
            .get(piece_id)
            .ok_or(Error::PieceDoesNotExist(piece_id))
    }

    pub fn get_mut(&mut self, piece_id: usize) -> Result<&mut (u8, Box<dyn Piece>), Error> {
        self.0
            .get_mut(piece_id)
            .ok_or(Error::PieceDoesNotExist(piece_id))
    }

    pub fn push(&mut self, player: u8, piece: Box<dyn Piece>) -> usize {
        let piece_id = self.0.len();
        self.0.push((player, piece));
        piece_id
    }
}

impl Default for Pieces {
    fn default() -> Self {
        Self::new()
    }
}
