use crate::{
    board::Board,
    coordinate::{Coordinate, CoordinateDelta},
    delta::Delta,
    r#move::Move,
};

use super::{StandardCompatiblePiece, StandardCompatiblePieceSet};

#[derive(Clone, Debug)]
pub struct Rook(u8, bool);

impl Rook {
    #[must_use]
    pub fn new(player: u8) -> StandardCompatiblePieceSet {
        Box::new(Self(player, false))
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
        board: &Board<StandardCompatiblePieceSet>,
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
        board: &Board<StandardCompatiblePieceSet>,
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
        _board: &Board<StandardCompatiblePieceSet>,
        _move: &Move,
        _turn: u16,
        _n_players: u8,
    ) -> Result<(Vec<Delta<StandardCompatiblePieceSet>>, u16), super::Error> {
        self.1 = true;

        Ok((Vec::with_capacity(0), 0))
    }

    fn clone(&self) -> StandardCompatiblePieceSet {
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

    use super::Rook;

    #[test]
    fn attacking() {
        const ROOK_1_POSITION: Coordinate = Coordinate(2, 2);

        const ROOK_2_POSITION: Coordinate = Coordinate(2, 3);
        const ROOK_3_POSITION: Coordinate = Coordinate(0, 2);

        let mut game = Game::new(2, 5, 5);

        game.add_piece(Rook::new(0), &ROOK_1_POSITION)
            .expect("failed to add first rook to board");
        game.add_piece(Rook::new(1), &ROOK_2_POSITION)
            .expect("failed to add second rook to board");
        game.add_piece(Rook::new(1), &ROOK_3_POSITION)
            .expect("failed to add third rook to board");

        let tests = [
            [false, false, true, false, false],
            [false, false, true, false, false],
            [true, true, false, true, true],
            [false, false, true, false, false],
            [false; 5],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = game
                    .board()
                    .is_being_attacked(&Coordinate(x, y), 1)
                    .unwrap();

                assert!(
                    result == expected,
                    "test failed: {ROOK_1_POSITION} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moves() {
        const ROOK_1_POSITION: Coordinate = Coordinate(2, 2);

        const ROOK_2_POSITION: Coordinate = Coordinate(2, 3);
        const ROOK_3_POSITION: Coordinate = Coordinate(0, 2);

        let mut game = Game::new(2, 5, 5);

        game.add_piece(Rook::new(0), &ROOK_1_POSITION)
            .expect("failed to add first rook to board");
        game.add_piece(Rook::new(1), &ROOK_2_POSITION)
            .expect("failed to add second rook to board");
        game.add_piece(Rook::new(1), &ROOK_3_POSITION)
            .expect("failed to add third rook to board");

        let tests = [
            [false, false, true, false, false],
            [false, false, true, false, false],
            [true, true, false, true, true],
            [false, false, true, false, false],
            [false; 5],
        ];

        game.generate_valid_moves()
            .expect("failed to generate moves");
        let valid_moves = game.valid_moves();

        for (from, _, _) in valid_moves {
            assert!(
                from == &ROOK_1_POSITION,
                "test failed: {from} != {ROOK_1_POSITION}"
            );
        }

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let position = Coordinate(x, y);

                let result = valid_moves.iter().find(|(_, to, _)| to == &position);

                assert!(
                    matches!(result, Some(_)) == expected,
                    "test failed: {ROOK_1_POSITION} -x ({x}, {y}), {} ({expected})",
                    !expected
                );
            }
        }
    }
}
