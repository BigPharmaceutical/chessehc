use crate::{board::Board, coordinate::Coordinate, delta::Delta, r#move::Move};
use std::{error, fmt::Debug};

pub trait PieceSet
where
    Self: Clone + Debug,
{
    type Error: error::Error;
    type PieceId;

    // Piece type constants
    fn type_id(&self) -> Self::PieceId;
    fn capture_points(&self) -> Option<u16>;
    fn blockable(&self) -> bool;

    // Piece constants
    fn player(&self) -> u8;

    // Probing
    fn is_in_check(
        &self,
        _board: &Board<Self>,
        _position: &Coordinate,
    ) -> Result<Option<bool>, Self::Error> {
        Ok(None)
    }
    fn valid_moves(
        &self,
        board: &Board<Self>,
        from: &Coordinate,
        turn: u16,
        n_players_in_game: u8,
    ) -> Result<Vec<(Coordinate, u8)>, Self::Error>;

    // Moving
    fn attacking(
        &self,
        board: &Board<Self>,
        from: &Coordinate,
    ) -> Result<Vec<Coordinate>, Self::Error>;
    fn mid_move(
        &mut self,
        board: &Board<Self>,
        r#move: &Move,
        turn: u16,
        n_players_in_game: u8,
    ) -> Result<(Vec<Delta<Self>>, u16), Self::Error>;
}
