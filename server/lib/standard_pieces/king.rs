use crate::{
    board::Board,
    coordinate::{Coordinate, CoordinateDelta},
    delta::Delta,
    piece_set::PieceSet,
    r#move::Move,
};

use super::{Error, StandardCompatiblePiece};

#[derive(Clone, Debug)]
pub struct King(u8, bool);

impl King {
    #[must_use]
    pub fn new(player: u8) -> Box<dyn StandardCompatiblePiece> {
        Box::new(Self(player, false))
    }
}

impl StandardCompatiblePiece for King {
    fn type_id(&self) -> u8 {
        6
    }

    fn capture_points(&self) -> Option<u16> {
        None
    }

    fn blockable(&self) -> bool {
        false
    }

    fn player(&self) -> u8 {
        self.0
    }

    fn is_in_check(
        &self,
        board: &Board<Box<dyn StandardCompatiblePiece>>,
        position: &Coordinate,
    ) -> Result<Option<bool>, Error> {
        board
            .is_being_attacked(position, self.player())
            .map(Some)
            .map_err(|err| Error::BoardError(Box::new(err)))
    }

    fn attacking(
        &self,
        board: &Board<Box<dyn StandardCompatiblePiece>>,
        from: &Coordinate,
    ) -> Result<Vec<Coordinate>, Error> {
        let mut attacks = Vec::new();

        for y in -1..=1 {
            for x in -1..=1 {
                if y == 0 && x == 0 {
                    continue;
                }

                let Some(coordinate) = from + (&CoordinateDelta(x, y), board) else { continue };
                attacks.push(coordinate);
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
    ) -> Result<Vec<(Coordinate, u8)>, Error> {
        let mut moves = Vec::new();

        for y in -1..=1 {
            for x in -1..=1 {
                if y == 0 && x == 0 {
                    continue;
                }

                let Some(coordinate) = from + (&CoordinateDelta(x, y), board) else { continue };

                if board
                    .is_being_attacked(&coordinate, self.0)
                    .map_err(|err| Error::BoardError(Box::new(err)))?
                {
                    continue;
                }

                moves.push((coordinate, 0));
            }
        }

        // Castling
        if !self.is_in_check(board, from)?.expect("castle check quasi-state") {
            for dir in [-1, 1] { 
                let mut castle_distance = None;

                for d in 1..isize::try_from(board.height()).expect("board height exceeded isize") {
                    let Some(position) = from + (&CoordinateDelta(dir * d, 0), board) else { break };
                    let Ok(spot) = board.get(&position) else { break };

                    let Some(piece) = spot.get() else { continue };

                    if piece.player() != self.0 {
                        break;
                    }
                    if !piece.can_castle() {
                        break;
                    }

                    castle_distance = Some(d);
                }

                let Some(castle_distance) = castle_distance else { continue };
                if castle_distance > 2 {
                    let Some(position) = from + (&CoordinateDelta(dir * 2, 0), board) else { continue };
                    if self.is_in_check(board, &position)?.expect("castle after into check quasi") { continue };
                    moves.push((position, 0));
                }
            }
        }

        Ok(moves)
    }

    fn mid_move(
        &mut self,
        board: &Board<Box<dyn StandardCompatiblePiece>>,
        r#move: &Move,
        _turn: u16,
        _n_players: u8,
    ) -> Result<(Vec<Delta<Box<dyn StandardCompatiblePiece>>>, u16), Error> {
        let delta = CoordinateDelta(
            isize::try_from(r#move.to.0).expect("move too large")
                - isize::try_from(r#move.from.0).expect("move too large"),
            {
                let mut dy = isize::try_from(r#move.to.1)
                    .map_err(|err| Error::PositionOrDeltaTooLarge(r#move.to.1, err))?
                    - isize::try_from(r#move.from.1)
                        .map_err(|err| Error::PositionOrDeltaTooLarge(r#move.from.1, err))?;
                if dy.abs() > 1 {
                    dy += -dy.signum()
                        * isize::try_from(board.height())
                            .map_err(|err| Error::PositionOrDeltaTooLarge(r#move.from.1, err))?;
                }
                dy
            },
        );

        let mut deltas = Vec::new();

        // Castling
        if !self.1 && delta.1 == 0 && delta.0.abs() == 2 {
            // Get the first piece in that direction
            // Lookahead dx
            for la_dx in 1..isize::try_from(board.height()).expect("board height exceeded isize") {
                let Some(position) =
                    &r#move.from + (&CoordinateDelta(la_dx * delta.0.signum(), 0), board) else { break };
                let Ok(c_spot) = board.get(&position) else { break };

                let Some(piece) = c_spot.get() else { continue };

                if piece.player() != self.0 {
                    break;
                }
                if !piece.can_castle() {
                    break;
                }

                deltas.push(Delta::Move(
                    position,
                    (&r#move.to + (&CoordinateDelta(-delta.0.signum(), 0), board)).ok_or(
                        Error::IntermediatePositionNotOnBoard(
                            position,
                            CoordinateDelta(-delta.0.signum(), 0),
                        ),
                    )?,
                ));
            }
        }

        Ok((deltas, 0))
    }

    fn clone(&self) -> Box<dyn StandardCompatiblePiece> {
        Box::new(Clone::clone(self))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        coordinate::{Coordinate, CoordinateDelta},
        game::Game,
        piece_set::PieceSet,
        r#move::Move,
        standard_pieces::{king::King, Pawn, Rook},
    };

    #[test]
    fn attacking() {
        const KING_POSITION: Coordinate = Coordinate(2, 2);

        let mut game = Game::new(1, 5, 5);
        game.add_piece(King::new(0), &KING_POSITION).unwrap();

        let tests = [
            [false; 5],
            [false, true, true, true, false],
            [false, true, false, true, false],
            [false, true, true, true, false],
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
                    "test failed: {KING_POSITION} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moves() {
        const KING_POSITION: Coordinate = Coordinate(2, 2);

        let mut game = Game::new(1, 5, 5);
        game.add_piece(King::new(0), &KING_POSITION).unwrap();

        let tests = [
            [false; 5],
            [false, true, true, true, false],
            [false, true, false, true, false],
            [false, true, true, true, false],
            [false; 5],
        ];

        game.generate_valid_moves().unwrap();
        let valid_moves = game.valid_moves();

        for (from, _, _) in valid_moves {
            assert!(
                from == &KING_POSITION,
                "test failed: {from} != {KING_POSITION}"
            );
        }

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let position = Coordinate(x, y);

                let result = valid_moves.iter().find(|(_, to, _)| to == &position);

                assert!(
                    matches!(result, Some(_)) == expected,
                    "test failed: {KING_POSITION} -x ({x}, {y}), {} ({expected})",
                    !expected
                );
            }
        }
    }

    #[test]
    fn castle() {
        const KING_POSITION: Coordinate = Coordinate(2, 2);
        const PAWN_POSITION: Coordinate = Coordinate(4, 2);
        const ROOK_POSITION: Coordinate = Coordinate(5, 2);

        let is_castle_move = |(from, to, data): &(Coordinate, Coordinate, u8)| {
            from == &KING_POSITION && to == &PAWN_POSITION && *data == 0
        };

        let mut game = Game::new(1, 6, 5);
        game.add_piece(King::new(0), &KING_POSITION)
            .expect("failed to place king on board");
        game.add_piece(Pawn::new(0, 1, 20), &PAWN_POSITION)
            .expect("failed to place pawn on board");
        game.add_piece(Rook::new(0), &ROOK_POSITION)
            .expect("failed to place rook on board");

        game.generate_valid_moves().unwrap();
        let valid_moves = game.valid_moves();
        assert!(
            !valid_moves.iter().any(is_castle_move),
            "test failed: {KING_POSITION} -> {PAWN_POSITION}, true (false)"
        );

        game.make_move(&Move {
            from: PAWN_POSITION,
            to: (&PAWN_POSITION + (&CoordinateDelta(0, 1), game.board())).unwrap(),
            data: 0,
            player: 0,
        })
        .expect("failed to make pawn move");

        game.start_turn().expect("failed to start next move");

        let valid_moves = game.valid_moves();
        assert!(
            valid_moves.iter().any(is_castle_move),
            "test failed: {KING_POSITION} -> {PAWN_POSITION}, false (true)"
        );

        game.make_move(&Move {
            from: KING_POSITION,
            to: PAWN_POSITION,
            data: 0,
            player: 0,
        })
        .expect("failed to make castle move");

        let tests = [
            [None; 6],
            [None; 6],
            [None, None, None, Some(4), Some(6), None],
            [None, None, None, None, Some(1), None],
            [None; 6],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = game
                    .board()
                    .get(&Coordinate(x, y))
                    .unwrap()
                    .get()
                    .as_ref()
                    .map(PieceSet::type_id);

                assert!(
                    result == expected,
                    "test failed: {PAWN_POSITION}: {result:?} ({expected:?})"
                );
            }
        }
    }
}
