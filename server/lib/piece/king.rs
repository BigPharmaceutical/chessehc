use crate::{
    board::{Board, Error::CoordinateNotOnBoard},
    logic::{Coordinate, CoordinateDelta, Move},
};

use super::{Piece, Pieces};

#[derive(Debug)]
/// King piece
pub struct King(u16);

impl King {
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }
}

impl Piece for King {
    fn takeable(&self) -> bool {
        false
    }

    fn blockable(&self) -> bool {
        false
    }

    fn moves(&self) -> u16 {
        self.0
    }

    fn capture_points(&self) -> u8 {
        0
    }

    fn add_attacks(&self, board: &mut Board, piece_id: usize, from: Coordinate) {
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
            let position = from.add(&CoordinateDelta(dir.0, dir.1), board);

            let result = board.attack(position, piece_id);
            match result {
                Err(CoordinateNotOnBoard(..)) => break,
                Err(_) => result.expect("failed to add attack"),
                _ => (),
            }
        }
    }

    fn is_valid_move(
        &self,
        board: &Board,
        pieces: &Pieces,
        _target: Option<&(u8, Box<dyn Piece>)>,
        r#move: &Move,
        to: &Coordinate,
    ) -> bool {
        // Check for attacks
        if board
            .get_spot(*to)
            .expect("invalid resulting coordinate")
            .is_being_attacked(pieces, r#move.player)
        {
            return false;
        }

        if r#move.delta.0.abs() < 2
            && r#move.delta.1.abs() < 2
            && !(r#move.delta.0 == 0 && r#move.delta.1 == 0)
        {
            return true;
        }

        // Castling
        if self.0 == 0 && r#move.delta.1 == 0 && r#move.delta.0.abs() == 2 {
            // Get the first piece in that direction
            // Lookahead dx
            for la_dx in 1.. {
                let position = r#move
                    .from
                    .add(&CoordinateDelta(la_dx * r#move.delta.0.signum(), 0), board);

                let result = board.get(pieces, position);
                match result {
                    Ok(Some(c_piece)) => {
                        if c_piece.0 == r#move.player
                            && c_piece.1.castleable()
                            && c_piece.1.moves() == 0
                        {
                            return true;
                        }
                        break;
                    }
                    Ok(None) => (),
                    Err(CoordinateNotOnBoard(..)) => break,
                    Err(_) => {
                        result.expect("could not get spot for castling");
                    }
                }
            }
        }

        false
    }

    fn increment_moves(&mut self) {
        self.0 += 1;
    }

    fn mid_move(
        &mut self,
        board: &mut Board,
        pieces: &mut Pieces,
        r#move: &Move,
        to: &Coordinate,
    ) -> (u8, Option<Box<dyn Piece>>) {
        // Castling logic
        if self.0 == 0 && r#move.delta.1 == 0 && r#move.delta.0.abs() == 2 {
            // Get the first piece in that direction
            // Lookahead dx
            for la_dx in 1.. {
                let position = r#move
                    .from
                    .add(&CoordinateDelta(la_dx * r#move.delta.0.signum(), 0), board);

                let result = board.get_spot_mut(position);
                let castle_id = match result {
                    Ok(spot) => {
                        if if let Some(c_piece) = spot.get(pieces) {
                            if c_piece.0 == r#move.player
                                && c_piece.1.castleable()
                                && c_piece.1.moves() == 0
                            {
                                // Castling
                                true
                            } else {
                                break;
                            }
                        } else {
                            false
                        } {
                            spot.take().unwrap()
                        } else {
                            continue;
                        }
                    }
                    Err(CoordinateNotOnBoard(..)) => break,
                    Err(_) => {
                        result.expect("could not get spot for castling");
                        break;
                    }
                };

                board.remove_attacks(castle_id);

                let castle_position = to.add(&CoordinateDelta(-r#move.delta.0.signum(), 0), board);
                board
                    .get_spot_mut(castle_position)
                    .unwrap()
                    .place(castle_id)
                    .unwrap();

                let castle = pieces
                    .get_mut(castle_id)
                    .expect("could not get castle piece");

                castle.1.increment_moves();
                castle.1.add_attacks(board, castle_id, castle_position);
            }
        }

        (0, None)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        board::Board,
        logic::{Coordinate, CoordinateDelta, Move},
        piece::{Pieces, Rook},
    };

    use super::King;

    #[test]
    fn attacking() {
        let king_1_position = Coordinate(2, 2);

        let rook_1_position = Coordinate(3, 3);

        let mut pieces = Pieces::new_with_capacity(2);
        let mut board = Board::new(2, 5, 5).expect("failed to create board");

        let king_1_id = pieces.push(0, Box::new(King::new()));

        let rook_1_id = pieces.push(1, Box::new(Rook::new()));

        board
            .add_piece(&pieces, king_1_id, king_1_position)
            .expect("failed to add king");

        board
            .add_piece(&pieces, rook_1_id, rook_1_position)
            .expect("failed to add rook");

        let tests = [
            [false; 5],
            [false, true, true, true, false],
            [false, true, false, true, false],
            [false, true, true, true, false],
            [false; 5],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = board
                    .get_spot(Coordinate(
                        i16::try_from(x).unwrap(),
                        i16::try_from(y).unwrap(),
                    ))
                    .unwrap()
                    .is_being_attacked(&pieces, 1);
                assert!(
                    result == expected,
                    "test failed: {king_1_position} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moving() {
        let king_1_position = Coordinate(2, 2);

        let rook_1_position = Coordinate(3, 3);

        let mut pieces = Pieces::new_with_capacity(2);
        let mut board = Board::new(2, 5, 5).expect("failed to create board");

        let king_1_id = pieces.push(0, Box::new(King::new()));

        let rook_1_id = pieces.push(1, Box::new(Rook::new()));

        board
            .add_piece(&pieces, king_1_id, king_1_position)
            .expect("failed to add king");

        board
            .add_piece(&pieces, rook_1_id, rook_1_position)
            .expect("failed to add rook");

        let tests = [
            [false; 5],
            [false, true, true, false, false],
            [false, true, false, false, false],
            [false, false, false, true, false],
            [false; 5],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = board
                    .is_valid_move(
                        &pieces,
                        Move {
                            player: 0,
                            from: king_1_position,
                            delta: CoordinateDelta(
                                i8::try_from(x).unwrap() - i8::try_from(king_1_position.0).unwrap(),
                                i8::try_from(y).unwrap() - i8::try_from(king_1_position.1).unwrap(),
                            ),
                            data: 0,
                        },
                    )
                    .expect("failed to validate move");
                assert!(
                    result == expected,
                    "test failed: {king_1_position} -> ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn castling() {
        let king_1_position = Coordinate(2, 2);

        let rook_1_position = Coordinate(5, 2);

        let mut pieces = Pieces::new_with_capacity(2);
        let mut board = Board::new(2, 6, 5).expect("failed to create board");

        let king_1_id = pieces.push(0, Box::new(King::new()));

        let rook_1_id = pieces.push(0, Box::new(Rook::new()));

        board
            .add_piece(&pieces, king_1_id, king_1_position)
            .expect("failed to add king");

        board
            .add_piece(&pieces, rook_1_id, rook_1_position)
            .expect("failed to add rook");

        let tests = [
            [false; 6],
            [false, true, true, true, false, false],
            [false, true, false, true, true, false],
            [false, true, true, true, false, false],
            [false; 6],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = board
                    .is_valid_move(
                        &pieces,
                        Move {
                            player: 0,
                            from: king_1_position,
                            delta: CoordinateDelta(
                                i8::try_from(x).unwrap() - i8::try_from(king_1_position.0).unwrap(),
                                i8::try_from(y).unwrap() - i8::try_from(king_1_position.1).unwrap(),
                            ),
                            data: 0,
                        },
                    )
                    .expect("failed to validate move");
                assert!(
                    result == expected,
                    "test failed: {king_1_position} -> ({x}, {y}), {result} ({expected})"
                );
            }
        }

        board
            .make_move(
                &mut pieces,
                Move {
                    player: 0,
                    from: king_1_position,
                    delta: CoordinateDelta(2, 0),
                    data: 0,
                },
            )
            .expect("failed to make move");

        let expected_board = [
            [None; 6],
            [None; 6],
            [None, None, None, Some(rook_1_id), Some(king_1_id), None],
            [None; 6],
            [None; 6],
        ];

        for (y, rank) in expected_board.iter().enumerate() {
            for (x, expected) in rank.iter().enumerate() {
                let coordinate = Coordinate(i16::try_from(x).unwrap(), i16::try_from(y).unwrap());
                let spot = board.get_spot(coordinate).expect("could not get spot");
                let result = match (spot.get_id(), expected) {
                    (Some(a), Some(b)) if a == b => true,
                    (None, None) => true,
                    _ => false,
                };

                assert!(
                    result,
                    "piece test failed: {coordinate}: {spot:?} ({expected:?})"
                );
            }
        }
    }
}
