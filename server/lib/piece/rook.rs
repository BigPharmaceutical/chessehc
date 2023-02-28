use crate::{
    board::Board,
    logic::{Coordinate, CoordinateDelta, Move},
};

use super::Piece;

#[derive(Debug, Default)]
pub struct Rook(u16);

impl Rook {
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }
}

impl Piece for Rook {
    fn capture_points(&self) -> u8 {
        5
    }

    fn moves(&self) -> u16 {
        self.0
    }

    fn is_attacking(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        let dx = to.0 - from.0;
        let dy_a = to.1 - from.1;
        let dy_b = dy_a.signum() * (dy_a.abs() - board.height());

        // Check that the move is straight
        if !((dx == 0) ^ (dy_a == 0)) {
            return false;
        }

        if dy_a == 0 {
            // If it would have to move horizontally:
            for d in 1..dx.abs() {
                let d = match i8::try_from(d * dx.signum()) {
                    Ok(value) => value,
                    Err(_) => continue,
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
            // If it would have to move vertically:
            // Assume it was going up then down
            'up_down_loop: for dy in [dy_a, dy_b] {
                // Check that no piece is between the origin and target
                for d in 1..dy.abs() {
                    let d = match i8::try_from(d * dy.signum()) {
                        Ok(value) => value,
                        Err(_) => continue,
                    };

                    let position = from.add(&CoordinateDelta(0, d), board);
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
        // Check that the move is straight
        if !((r#move.delta.0 == 0) ^ (r#move.delta.1 == 0)) {
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

    use super::Rook;

    #[test]
    fn attacking() {
        let rook_position = Coordinate(4, 4);

        let mut board = Board::new(2, 9, 9).expect("failed to create board");
        let rook_id = board
            .add_piece(0, Box::new(Rook::new()), rook_position)
            .expect("failed to add rook");
        board
            .add_piece(1, Box::new(Rook::new()), Coordinate(4, 3))
            .expect("failed to add second rook");
        board
            .add_piece(1, Box::new(Rook::new()), Coordinate(4, 7))
            .expect("failed to add third rook");

        let rook = board.get_piece(rook_id).expect("failed to get rook");

        let tests = [
            [false; 9],
            [false; 9],
            [false; 9],
            [false, false, false, false, true, false, false, false, false],
            [true, true, true, true, false, true, true, true, true],
            [false, false, false, false, true, false, false, false, false],
            [false, false, false, false, true, false, false, false, false],
            [false, false, false, false, true, false, false, false, false],
            [false; 9],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = rook.1.is_attacking(
                    &board,
                    &rook_position,
                    &Coordinate(i16::try_from(x).unwrap(), i16::try_from(y).unwrap()),
                );
                assert!(
                    result == expected,
                    "test failed: {rook_position} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moving() {
        let rook_position = Coordinate(4, 4);

        let mut board = Board::new(2, 9, 9).expect("failed to create board");
        board
            .add_piece(0, Box::new(Rook::new()), rook_position)
            .expect("failed to add rook");
        board
            .add_piece(1, Box::new(Rook::new()), Coordinate(4, 3))
            .expect("failed to add second rook");
        board
            .add_piece(1, Box::new(Rook::new()), Coordinate(4, 7))
            .expect("failed to add third rook");

        let tests = [
            [false; 9],
            [false; 9],
            [false; 9],
            [false, false, false, false, true, false, false, false, false],
            [true, true, true, true, false, true, true, true, true],
            [false, false, false, false, true, false, false, false, false],
            [false, false, false, false, true, false, false, false, false],
            [false, false, false, false, true, false, false, false, false],
            [false; 9],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = board
                    .is_valid_move(Move {
                        player: 0,
                        from: rook_position,
                        delta: CoordinateDelta(
                            i8::try_from(x).unwrap() - i8::try_from(rook_position.0).unwrap(),
                            i8::try_from(y).unwrap() - i8::try_from(rook_position.1).unwrap(),
                        ),
                        data: 0,
                    })
                    .expect("failed to validate move");
                assert!(
                    result == expected,
                    "test failed: {rook_position} -> ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }
}
