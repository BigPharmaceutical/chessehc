use crate::{
    board::Board,
    logic::{Coordinate, CoordinateDelta, Move},
};

use super::Piece;

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

    fn moves(&self) -> u16 {
        self.0
    }

    fn is_attacking(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        let dx = to.0 - from.0;
        let dy_a = to.1 - from.1;
        let dy_b = dy_a.signum() * (dy_a.abs() - board.height());

        // Assume it was going up then down
        'up_down_loop: for dy in [dy_a, dy_b] {
            // Check that the move is diagonal and non-zero
            if dx.abs() != dy.abs() || dx == 0 {
                continue;
            }

            // Check that no piece is between the origin and target
            for d in 1..dx.abs() {
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

    fn is_valid_move(
        &self,
        _target: Option<&(u8, Box<dyn Piece>)>,
        board: &Board,
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

    use super::Bishop;

    #[test]
    fn attacking() {
        let bishop_position = Coordinate(4, 4);

        let mut board = Board::new(2, 9, 9).expect("failed to create board");
        let bishop_id = board
            .add_piece(0, Box::new(Bishop::new()), bishop_position)
            .expect("failed to add bishop");
        board
            .add_piece(1, Box::new(Bishop::new()), Coordinate(3, 3))
            .expect("failed to add second bishop");
        board
            .add_piece(1, Box::new(Bishop::new()), Coordinate(7, 7))
            .expect("failed to add third bishop");

        let bishop = board.get_piece(bishop_id).expect("failed to get bishop");

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
                let result = bishop.1.is_attacking(
                    &board,
                    &bishop_position,
                    &Coordinate(i16::try_from(x).unwrap(), i16::try_from(y).unwrap()),
                );
                assert!(
                    result == expected,
                    "test failed: {bishop_position} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moving() {
        let bishop_position = Coordinate(4, 4);

        let mut board = Board::new(2, 9, 9).expect("failed to create board");
        board
            .add_piece(0, Box::new(Bishop::new()), bishop_position)
            .expect("failed to add bishop");
        board
            .add_piece(1, Box::new(Bishop::new()), Coordinate(3, 3))
            .expect("failed to add second bishop");
        board
            .add_piece(1, Box::new(Bishop::new()), Coordinate(7, 7))
            .expect("failed to add third bishop");

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
                    .is_valid_move(Move {
                        player: 0,
                        from: bishop_position,
                        delta: CoordinateDelta(
                            i8::try_from(x).unwrap() - i8::try_from(bishop_position.0).unwrap(),
                            i8::try_from(y).unwrap() - i8::try_from(bishop_position.1).unwrap(),
                        ),
                        data: 0,
                    })
                    .expect("failed to validate move");
                assert!(
                    result == expected,
                    "test failed: {bishop_position} -> ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }
}
