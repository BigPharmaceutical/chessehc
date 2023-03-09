use crate::{
    board::Board,
    coordinate::{Coordinate, CoordinateDelta},
    delta::Delta,
    r#move::Move,
};

use super::StandardCompatiblePiece;

#[derive(Clone, Debug)]
pub struct Rook(u8, bool);

impl Rook {
    pub fn new(player: u8) -> Box<dyn StandardCompatiblePiece> {
        Box::new(Rook(player, false))
    }
}

impl StandardCompatiblePiece for Rook {
    fn type_id(&self) -> u8 {
        4
    }

    fn capture_points(&self) -> Option<u16> {
        Some(5)
    }

    fn blockable(&self) -> bool {
        true
    }

    fn player(&self) -> u8 {
        self.0
    }

    fn attacking(
        &self,
        board: &Board<Box<dyn StandardCompatiblePiece>>,
        from: &Coordinate,
    ) -> Result<Vec<Coordinate>, super::Error> {
        let mut attacks = Vec::new();

        for dir in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
            for d in 1..isize::try_from(board.height()).expect("board height exceeded isize") {
                let Some(position) = from + (&CoordinateDelta(dir.0 * d, dir.1 * d), board) else { break };
                let Ok(spot) = board.get(&position) else { break };

                attacks.push(position);
                if spot.get().is_some() {
                    break;
                };
            }
        }

        Ok(attacks)
    }

    fn valid_moves(
        &self,
        board: &Board<Box<dyn StandardCompatiblePiece>>,
        from: &Coordinate,
        _turn: u16,
        _n_players: u8,
    ) -> Result<Vec<(Coordinate, u8)>, super::Error> {
        let mut moves = Vec::new();

        for dir in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
            for d in 1..isize::try_from(board.height()).expect("board height exceeded isize") {
                let Some(position) = from + (&CoordinateDelta(dir.0 * d, dir.1 * d), board) else { break };
                let Ok(spot) = board.get(&position) else { break };

                let Some(piece) = spot.get() else {
                    moves.push((position, 0));
                    continue;
                };

                if piece.player() != self.0 && piece.capture_points().is_some() {
                    moves.push((position, 0));
                }

                break;
            }
        }

        Ok(moves)
    }

    fn mid_move(
        &mut self,
        _board: &Board<Box<dyn StandardCompatiblePiece>>,
        _move: &Move,
        _turn: u16,
        _n_players: u8,
    ) -> Result<(Vec<Delta<Box<dyn StandardCompatiblePiece>>>, u16), super::Error> {
        self.1 = true;

        Ok((Vec::with_capacity(0), 0))
    }

    fn clone(&self) -> Box<dyn StandardCompatiblePiece> {
        Box::new(Clone::clone(self))
    }

    fn can_castle(&self) -> bool {
        !self.1
    }

    fn mid_castle(&mut self) {
        self.1 = true;
    }
}
