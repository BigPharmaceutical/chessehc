use crate::{
    board::Board,
    coordinate::{Coordinate, CoordinateDelta},
    delta::Delta,
    r#move::Move,
};

use super::StandardCompatiblePiece;

#[derive(Clone, Debug)]
pub struct Queen(u8, bool);

impl Queen {
    #[must_use]
    pub fn new(player: u8) -> Box<dyn StandardCompatiblePiece> {
        Box::new(Self(player, false))
    }
}

impl StandardCompatiblePiece for Queen {
    fn type_id(&self) -> u8 {
        2
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

        for dir in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
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

        for dir in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
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

#[cfg(test)]
mod test {
    use crate::{coordinate::Coordinate, game::Game};

    use super::Queen;

    #[test]
    fn attacking() {
        const QUEEN_1_POSITION: Coordinate = Coordinate(2, 2);

        const QUEEN_2_POSITION: Coordinate = Coordinate(1, 1);
        const QUEEN_3_POSITION: Coordinate = Coordinate(0, 4);
        const QUEEN_4_POSITION: Coordinate = Coordinate(2, 3);
        const QUEEN_5_POSITION: Coordinate = Coordinate(0, 2);

        let mut game = Game::new(2, 5, 5);

        game.add_piece(Queen::new(0), &QUEEN_1_POSITION)
            .expect("failed to add first queen to board");
        game.add_piece(Queen::new(1), &QUEEN_2_POSITION)
            .expect("failed to add second queen to board");
        game.add_piece(Queen::new(1), &QUEEN_3_POSITION)
            .expect("failed to add third queen to board");
        game.add_piece(Queen::new(1), &QUEEN_4_POSITION)
            .expect("failed to add fourth queen to board");
        game.add_piece(Queen::new(1), &QUEEN_5_POSITION)
            .expect("failed to add fifth queen to board");

        let tests = [
            [false, false, true, false, true],
            [false, true, true, true, false],
            [true, true, false, true, true],
            [false, true, true, true, false],
            [true, false, false, false, true],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = game
                    .board()
                    .is_being_attacked(&Coordinate(x, y), 1)
                    .unwrap();

                assert!(
                    result == expected,
                    "test failed: {QUEEN_1_POSITION} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moves() {
        const QUEEN_1_POSITION: Coordinate = Coordinate(2, 2);

        const QUEEN_2_POSITION: Coordinate = Coordinate(1, 1);
        const QUEEN_3_POSITION: Coordinate = Coordinate(0, 4);
        const QUEEN_4_POSITION: Coordinate = Coordinate(2, 3);
        const QUEEN_5_POSITION: Coordinate = Coordinate(0, 2);

        let mut game = Game::new(2, 5, 5);

        game.add_piece(Queen::new(0), &QUEEN_1_POSITION)
            .expect("failed to add first queen to board");
        game.add_piece(Queen::new(1), &QUEEN_2_POSITION)
            .expect("failed to add second queen to board");
        game.add_piece(Queen::new(1), &QUEEN_3_POSITION)
            .expect("failed to add third queen to board");
        game.add_piece(Queen::new(1), &QUEEN_4_POSITION)
            .expect("failed to add fourth queen to board");
        game.add_piece(Queen::new(1), &QUEEN_5_POSITION)
            .expect("failed to add fifth queen to board");

        let tests = [
            [false, false, true, false, true],
            [false, true, true, true, false],
            [true, true, false, true, true],
            [false, true, true, true, false],
            [true, false, false, false, true],
        ];

        game.generate_valid_moves()
            .expect("failed to generate moves");
        let valid_moves = game.valid_moves();

        for (from, _, _) in valid_moves {
            assert!(
                from == &QUEEN_1_POSITION,
                "test failed: {from} != {QUEEN_1_POSITION}"
            );
        }

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let position = Coordinate(x, y);

                let result = valid_moves.iter().find(|(_, to, _)| to == &position);

                assert!(
                    matches!(result, Some(_)) == expected,
                    "test failed: {QUEEN_1_POSITION} -x ({x}, {y}), {} ({expected})",
                    !expected
                );
            }
        }
    }
}
