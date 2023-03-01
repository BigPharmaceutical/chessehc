use crate::{
    board::Board,
    logic::{Coordinate, CoordinateDelta, Move},
};

use super::Piece;

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

    fn moves(&self) -> u16 {
        self.0
    }

    fn is_attacking(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        let dx = to.0 - from.0;
        let dy_a = to.1 - from.1;
        let dy_b = dy_a.signum() * (dy_a.abs() - board.height());

        if dy_a == 0 {
            // If it would have to move horizontally:
            // Check that the move is non-zero
            if dx == 0 {
                return false;
            }

            for d in 1..dx.abs() {
                let Ok(d) = i8::try_from(d * dx.signum()) else {
                    continue;
                };

                let position = from.add(&CoordinateDelta(d, 0), board);
                if board
                    .get(position)
                    .expect("could not get spot between two valid spots")
                    .is_some()
                {
                    return false;
                }
            }

            true
        } else {
            // Assume it was going up then down
            'up_down_loop: for dy in [dy_a, dy_b] {
                // Check that the move is vertical or diagonal
                if dx != 0 && dx.abs() != dy.abs() {
                    continue;
                }

                // Check that no piece is between the origin and target
                for d in 1..dy.abs() {
                    let Ok(d) = i8::try_from(d) else {
                        continue;
                    };

                    let position = from.add(
                        &CoordinateDelta(
                            d * i8::try_from(dx.signum()).unwrap(),
                            d * i8::try_from(dy.signum()).unwrap(),
                        ),
                        board,
                    );
                    if board
                        .get(position)
                        .expect("could not get spot between two valid spots")
                        .is_some()
                    {
                        continue 'up_down_loop;
                    }
                }

                return true;
            }
            false
        }
    }

    fn is_valid_move(
        &self,
        _target: Option<&(u8, Box<dyn Piece>)>,
        board: &Board,
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
                .get(position)
                .expect("could not get spot between two valid spots")
                .is_some()
            {
                return false;
            }
        }

        true
    }

    fn mid_move(
        &mut self,
        _board: &mut Board,
        _move: &Move,
        _to: &Coordinate,
    ) -> (u8, Option<Box<dyn Piece>>) {
        self.0 += 1;
        (0, None)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        board::Board,
        logic::{Coordinate, CoordinateDelta, Move},
    };

    use super::Queen;

    #[test]
    fn attacking() {
        let queen_position = Coordinate(4, 4);

        let mut board = Board::new(2, 9, 9).expect("failed to create board");
        let queen_id = board
            .add_piece(0, Box::new(Queen::new()), queen_position)
            .expect("failed to add queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(1, 4))
            .expect("failed to add second queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(4, 3))
            .expect("failed to add third queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(4, 8))
            .expect("failed to add third queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(6, 2))
            .expect("failed to add fourth queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(7, 7))
            .expect("failed to add fifth queen");

        let queen = board.get_piece(queen_id).expect("failed to get queen");

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
                let result = queen.1.is_attacking(
                    &board,
                    &queen_position,
                    &Coordinate(i16::try_from(x).unwrap(), i16::try_from(y).unwrap()),
                );
                assert!(
                    result == expected,
                    "test failed: {queen_position} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moving() {
        let queen_position = Coordinate(4, 4);

        let mut board = Board::new(2, 9, 9).expect("failed to create board");
        board
            .add_piece(0, Box::new(Queen::new()), queen_position)
            .expect("failed to add queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(1, 4))
            .expect("failed to add second queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(4, 3))
            .expect("failed to add third queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(4, 8))
            .expect("failed to add third queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(6, 2))
            .expect("failed to add fourth queen");
        board
            .add_piece(1, Box::new(Queen::new()), Coordinate(7, 7))
            .expect("failed to add fifth queen");

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
                    .is_valid_move(Move {
                        player: 0,
                        from: queen_position,
                        delta: CoordinateDelta(
                            i8::try_from(x).unwrap() - i8::try_from(queen_position.0).unwrap(),
                            i8::try_from(y).unwrap() - i8::try_from(queen_position.1).unwrap(),
                        ),
                        data: 0,
                    })
                    .expect("failed to validate move");
                assert!(
                    result == expected,
                    "test failed: {queen_position} -> ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }
}
