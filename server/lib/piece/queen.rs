use crate::{
    board::{Board, Error::CoordinateNotOnBoard},
    logic::{Coordinate, CoordinateDelta, Move},
};

use super::{Piece, Pieces};

#[derive(Debug, Default)]
pub struct Queen(u16);

impl Queen {
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }
}

impl Piece for Queen {
    fn capture_points(&self) -> u8 {
        9
    }

    fn blockable(&self) -> bool {
        true
    }

    fn moves(&self) -> u16 {
        self.0
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
            let mut d: i8 = 0;
            loop {
                d = match d.checked_add(1) {
                    Some(value) => value,
                    None => break,
                };

                let position = from.add(&CoordinateDelta(d * dir.0, d * dir.1), board);

                // Wrapping case
                if position == from {
                    break;
                }

                let result = board.attack(position, piece_id);
                match result {
                    Err(CoordinateNotOnBoard(..)) => break,
                    Err(_) => result.expect("failed to add attack"),
                    _ => (),
                }

                if board
                    .get_spot(position)
                    .expect("failed to get spot")
                    .is_occupied()
                {
                    break;
                }
            }
        }
    }

    fn is_valid_move(
        &self,
        board: &Board,
        pieces: &Pieces,
        _target: Option<&(u8, Box<dyn Piece>)>,
        r#move: &Move,
        _to: &Coordinate,
    ) -> bool {
        // Check that the move is straight or diagonal
        if !(((r#move.delta.0 == 0) ^ (r#move.delta.1 == 0))
            || r#move.delta.0.abs() == r#move.delta.1.abs())
        {
            return false;
        }

        // Check that no piece is between the origin and target
        for d in 1..if r#move.delta.0 == 0 {
            r#move.delta.1
        } else {
            r#move.delta.0
        }
        .abs()
        {
            let position = r#move.from.add(
                &CoordinateDelta(d * r#move.delta.0.signum(), d * r#move.delta.1.signum()),
                board,
            );
            if board
                .get(pieces, position)
                .expect("could not get spot between two valid spots")
                .is_some()
            {
                return false;
            }
        }

        true
    }

    fn increment_moves(&mut self) {
        self.0 += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::{
        board::Board,
        logic::{Coordinate, CoordinateDelta, Move},
        piece::Pieces,
    };

    use super::Queen;

    #[test]
    fn attacking() {
        let queen_1_position = Coordinate(4, 4);
        let queen_2_position = Coordinate(1, 4);
        let queen_3_position = Coordinate(4, 3);
        let queen_4_position = Coordinate(4, 8);
        let queen_5_position = Coordinate(6, 2);
        let queen_6_position = Coordinate(7, 7);

        let mut pieces = Pieces::new_with_capacity(6);
        let mut board = Board::new(2, 9, 9).expect("failed to create board");

        let queen_1_id = pieces.push(0, Box::new(Queen::new()));
        let queen_2_id = pieces.push(1, Box::new(Queen::new()));
        let queen_3_id = pieces.push(1, Box::new(Queen::new()));
        let queen_4_id = pieces.push(1, Box::new(Queen::new()));
        let queen_5_id = pieces.push(1, Box::new(Queen::new()));
        let queen_6_id = pieces.push(1, Box::new(Queen::new()));

        board
            .add_piece(&pieces, queen_1_id, queen_1_position)
            .expect("failed to add queen");
        board
            .add_piece(&pieces, queen_2_id, queen_2_position)
            .expect("failed to add second queen");
        board
            .add_piece(&pieces, queen_3_id, queen_3_position)
            .expect("failed to add third queen");
        board
            .add_piece(&pieces, queen_4_id, queen_4_position)
            .expect("failed to add fourth queen");
        board
            .add_piece(&pieces, queen_5_id, queen_5_position)
            .expect("failed to add fifth queen");
        board
            .add_piece(&pieces, queen_6_id, queen_6_position)
            .expect("failed to add sixth queen");

        let tests = [
            [true, false, false, false, false, false, false, false, false],
            [false, true, false, false, false, false, false, false, false],
            [false, false, true, false, false, false, true, false, false],
            [false, false, false, true, true, true, false, false, false],
            [false, true, true, true, false, true, true, true, true],
            [false, false, false, true, true, true, false, false, false],
            [false, false, true, false, true, false, true, false, false],
            [false, true, false, false, true, false, false, true, false],
            [true, false, false, false, true, false, false, false, false],
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
                    "test failed: {queen_1_position} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moving() {
        let queen_1_position = Coordinate(4, 4);
        let queen_2_position = Coordinate(1, 4);
        let queen_3_position = Coordinate(4, 3);
        let queen_4_position = Coordinate(4, 8);
        let queen_5_position = Coordinate(6, 2);
        let queen_6_position = Coordinate(7, 7);

        let mut pieces = Pieces::new_with_capacity(6);
        let mut board = Board::new(2, 9, 9).expect("failed to create board");

        let queen_1_id = pieces.push(0, Box::new(Queen::new()));
        let queen_2_id = pieces.push(1, Box::new(Queen::new()));
        let queen_3_id = pieces.push(1, Box::new(Queen::new()));
        let queen_4_id = pieces.push(1, Box::new(Queen::new()));
        let queen_5_id = pieces.push(1, Box::new(Queen::new()));
        let queen_6_id = pieces.push(1, Box::new(Queen::new()));

        board
            .add_piece(&pieces, queen_1_id, queen_1_position)
            .expect("failed to add queen");
        board
            .add_piece(&pieces, queen_2_id, queen_2_position)
            .expect("failed to add second queen");
        board
            .add_piece(&pieces, queen_3_id, queen_3_position)
            .expect("failed to add third queen");
        board
            .add_piece(&pieces, queen_4_id, queen_4_position)
            .expect("failed to add fourth queen");
        board
            .add_piece(&pieces, queen_5_id, queen_5_position)
            .expect("failed to add fifth queen");
        board
            .add_piece(&pieces, queen_6_id, queen_6_position)
            .expect("failed to add sixth queen");

        let tests = [
            [true, false, false, false, false, false, false, false, false],
            [false, true, false, false, false, false, false, false, false],
            [false, false, true, false, false, false, true, false, false],
            [false, false, false, true, true, true, false, false, false],
            [false, true, true, true, false, true, true, true, true],
            [false, false, false, true, true, true, false, false, false],
            [false, false, true, false, true, false, true, false, false],
            [false, true, false, false, true, false, false, true, false],
            [true, false, false, false, true, false, false, false, false],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = board
                    .is_valid_move(
                        &pieces,
                        Move {
                            player: 0,
                            from: queen_1_position,
                            delta: CoordinateDelta(
                                i8::try_from(x).unwrap()
                                    - i8::try_from(queen_1_position.0).unwrap(),
                                i8::try_from(y).unwrap()
                                    - i8::try_from(queen_1_position.1).unwrap(),
                            ),
                            data: 0,
                        },
                    )
                    .expect("failed to validate move");
                assert!(
                    result == expected,
                    "test failed: {queen_1_position} -> ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }
}
