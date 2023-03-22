use crate::{
    board::Board,
    coordinate::{Coordinate, CoordinateDelta},
    delta::Delta,
    r#move::Move,
};

use super::{Error, StandardCompatiblePiece};

#[derive(Clone, Debug)]
pub struct Knight(u8);

impl Knight {
    pub fn new(player: u8) -> Box<dyn StandardCompatiblePiece> {
        Box::new(Self(player))
    }
}

impl StandardCompatiblePiece for Knight {
    fn type_id(&self) -> u8 {
        3
    }

    fn capture_points(&self) -> Option<u16> {
        Some(3)
    }

    fn blockable(&self) -> bool {
        false
    }

    fn player(&self) -> u8 {
        self.0
    }

    fn attacking(
        &self,
        board: &Board<Box<dyn StandardCompatiblePiece>>,
        from: &Coordinate,
    ) -> Result<Vec<Coordinate>, super::Error> {
        let mut attacks = Vec::with_capacity(8);

        for dir in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
            if let Some(position) = from + (&CoordinateDelta(dir.0 * 2, dir.1), board) {
                attacks.push(position);
            }
            if let Some(position) = from + (&CoordinateDelta(dir.0, dir.1 * 2), board) {
                attacks.push(position);
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
        let mut moves = Vec::with_capacity(8);

        for dir in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
            if let Some(position) = from + (&CoordinateDelta(dir.0 * 2, dir.1), board) {
                if if let Some(piece) = board
                    .get(&position)
                    .map_err(|err| Error::BoardError(Box::new(err)))?
                    .get()
                {
                    piece.player() != self.0 && piece.capture_points().is_some()
                } else {
                    true
                } {
                    moves.push((position, 0));
                }
            }
            if let Some(position) = from + (&CoordinateDelta(dir.0, dir.1 * 2), board) {
                if if let Some(piece) = board
                    .get(&position)
                    .map_err(|err| Error::BoardError(Box::new(err)))?
                    .get()
                {
                    piece.player() != self.0 && piece.capture_points().is_some()
                } else {
                    true
                } {
                    moves.push((position, 0));
                }
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
    ) -> Result<(Vec<Delta<Box<dyn StandardCompatiblePiece>>>, u16), Error> {
        Ok((Vec::with_capacity(0), 0))
    }

    fn clone(&self) -> Box<dyn StandardCompatiblePiece> {
        Box::new(Clone::clone(self))
    }
}

#[cfg(test)]
mod test {
    use crate::{coordinate::Coordinate, game::Game};

    use super::Knight;

    #[test]
    fn attacking() {
        const KNIGHT_1_POSITION: Coordinate = Coordinate(2, 2);

        const KNIGHT_2_POSITION: Coordinate = Coordinate(1, 4);

        let mut game = Game::new(2, 5, 5);

        game.add_piece(Knight::new(0), &KNIGHT_1_POSITION);
        game.add_piece(Knight::new(1), &KNIGHT_2_POSITION);

        let tests = [
            [false, true, false, true, false],
            [true, false, false, false, true],
            [false; 5],
            [true, false, false, false, true],
            [false, true, false, true, false],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = game
                    .board()
                    .is_being_attacked(&Coordinate(x, y), 1)
                    .unwrap();

                assert!(
                    result == expected,
                    "test failed: {KNIGHT_1_POSITION} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moves() {
        const KNIGHT_1_POSITION: Coordinate = Coordinate(2, 2);

        const KNIGHT_2_POSITION: Coordinate = Coordinate(1, 4);

        let mut game = Game::new(2, 5, 5);

        game.add_piece(Knight::new(0), &KNIGHT_1_POSITION);
        game.add_piece(Knight::new(1), &KNIGHT_2_POSITION);

        let tests = [
            [false, true, false, true, false],
            [true, false, false, false, true],
            [false; 5],
            [true, false, false, false, true],
            [false, true, false, true, false],
        ];

        game.generate_valid_moves()
            .expect("failed to generate moves");
        let valid_moves = game.valid_moves();

        for (from, _, _) in valid_moves {
            assert!(
                from == &KNIGHT_1_POSITION,
                "test failed: {from} != {KNIGHT_1_POSITION}"
            );
        }

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let position = Coordinate(x, y);

                let result = valid_moves.iter().find(|(_, to, _)| to == &position);

                assert!(
                    matches!(result, Some(_)) == expected,
                    "test failed: {KNIGHT_1_POSITION} -x ({x}, {y}), {} ({expected})",
                    !expected
                );
            }
        }
    }
}

