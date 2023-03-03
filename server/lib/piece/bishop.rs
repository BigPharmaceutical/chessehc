use crate::{
    board::{Board, Error::CoordinateNotOnBoard},
    logic::{Coordinate, CoordinateDelta, Move},
};

use super::{Piece, Pieces};

#[derive(Debug, Default)]
pub struct Bishop(u16);

impl Bishop {
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }
}

impl Piece for Bishop {
    fn capture_points(&self) -> u8 {
        3
    }

    fn blockable(&self) -> bool {
        true
    }

    fn moves(&self) -> u16 {
        self.0
    }

    fn add_attacks(&self, board: &mut Board, piece_id: usize, from: Coordinate) {
        for dir in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
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
        // Check that the move is diagonal and non-zero
        if r#move.delta.0.abs() != r#move.delta.1.abs() || r#move.delta.0 == 0 {
            return false;
        }

        // Check that no piece is between the origin and target
        for d in 1..r#move.delta.0.abs() {
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

    use super::Bishop;

    #[test]
    fn attacking() {
        let bishop_1_position = Coordinate(4, 4);
        let bishop_2_position = Coordinate(3, 3);
        let bishop_3_position = Coordinate(7, 7);

        let mut pieces = Pieces::new_with_capacity(3);
        let mut board = Board::new(2, 9, 9).expect("failed to create board");

        let bishop_1_id = pieces.push(0, Box::new(Bishop::new()));
        let bishop_2_id = pieces.push(1, Box::new(Bishop::new()));
        let bishop_3_id = pieces.push(1, Box::new(Bishop::new()));

        board
            .add_piece(&pieces, bishop_1_id, bishop_1_position)
            .expect("failed to add bishop");
        board
            .add_piece(&pieces, bishop_2_id, bishop_2_position)
            .expect("failed to add bishop");
        board
            .add_piece(&pieces, bishop_3_id, bishop_3_position)
            .expect("failed to add bishop");

        let tests = [
            [false, false, false, false, false, false, false, false, true],
            [false, false, false, false, false, false, false, true, false],
            [false, false, false, false, false, false, true, false, false],
            [false, false, false, true, false, true, false, false, false],
            [false; 9],
            [false, false, false, true, false, true, false, false, false],
            [false, false, true, false, false, false, true, false, false],
            [false, true, false, false, false, false, false, true, false],
            [true, false, false, false, false, false, false, false, false],
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
                    "test failed: {bishop_1_position} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moving() {
        let bishop_1_position = Coordinate(4, 4);
        let bishop_2_position = Coordinate(3, 3);
        let bishop_3_position = Coordinate(7, 7);

        let mut pieces = Pieces::new_with_capacity(3);
        let mut board = Board::new(2, 9, 9).expect("failed to create board");

        let bishop_1_id = pieces.push(0, Box::new(Bishop::new()));
        let bishop_2_id = pieces.push(1, Box::new(Bishop::new()));
        let bishop_3_id = pieces.push(1, Box::new(Bishop::new()));

        board
            .add_piece(&pieces, bishop_1_id, bishop_1_position)
            .expect("failed to add bishop");
        board
            .add_piece(&pieces, bishop_2_id, bishop_2_position)
            .expect("failed to add bishop");
        board
            .add_piece(&pieces, bishop_3_id, bishop_3_position)
            .expect("failed to add bishop");

        let tests = [
            [false, false, false, false, false, false, false, false, true],
            [false, false, false, false, false, false, false, true, false],
            [false, false, false, false, false, false, true, false, false],
            [false, false, false, true, false, true, false, false, false],
            [false; 9],
            [false, false, false, true, false, true, false, false, false],
            [false, false, true, false, false, false, true, false, false],
            [false, true, false, false, false, false, false, true, false],
            [true, false, false, false, false, false, false, false, false],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = board
                    .is_valid_move(
                        &pieces,
                        Move {
                            player: 0,
                            from: bishop_1_position,
                            delta: CoordinateDelta(
                                i8::try_from(x).unwrap()
                                    - i8::try_from(bishop_1_position.0).unwrap(),
                                i8::try_from(y).unwrap()
                                    - i8::try_from(bishop_1_position.1).unwrap(),
                            ),
                            data: 0,
                        },
                    )
                    .expect("failed to validate move");
                assert!(
                    result == expected,
                    "test failed: {bishop_1_position} -> ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }
}
