use crate::{board::Board, logic::Coordinate};

use super::Piece;

#[derive(Debug)]
/// Knight piece
pub struct Knight(u16);

impl Knight {
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }
}

impl Piece for Knight {
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

        // Assume dy_a then dy_b
        for dy in [dy_a, dy_b] {
            // Check distances of the attack
            if (dx.abs() == 1 && dy.abs() == 2) || (dx.abs() == 2 && dy.abs() == 1) {
                return true;
            }
        }

        false
    }

    fn is_valid_move(
        &self,
        _target: Option<&(u8, Box<dyn Piece>)>,
        _board: &Board,
        r#move: &crate::logic::Move,
        _to: &Coordinate,
    ) -> bool {
        // Check the distances of the move
        (r#move.delta.0.abs() == 1 && r#move.delta.1.abs() == 2)
            || (r#move.delta.0.abs() == 2 && r#move.delta.1.abs() == 1)
    }

    fn mid_move(
        &mut self,
        _board: &mut Board,
        _move: &crate::logic::Move,
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

    use super::Knight;

    #[test]
    fn attacking() {
        let knight_position = Coordinate(3, 3);

        let mut board = Board::new(2, 7, 7).expect("failed to create board");
        let knight_id = board
            .add_piece(0, Box::new(Knight::new()), knight_position)
            .expect("failed to add knight");
        board
            .add_piece(1, Box::new(Knight::new()), Coordinate(2, 1))
            .expect("failed to add second knight");
        board
            .add_piece(1, Box::new(Knight::new()), Coordinate(5, 2))
            .expect("failed to add third knight");

        let knight = board.get_piece(knight_id).expect("failed to get knight");

        let tests = [
            [false; 7],
            [false, false, true, false, true, false, false],
            [false, true, false, false, false, true, false],
            [false; 7],
            [false, true, false, false, false, true, false],
            [false, false, true, false, true, false, false],
            [false; 7],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = knight.1.is_attacking(
                    &board,
                    &knight_position,
                    &Coordinate(i16::try_from(x).unwrap(), i16::try_from(y).unwrap()),
                );
                assert!(
                    result == expected,
                    "test failed: {knight_position} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moving() {
        let knight_position = Coordinate(3, 3);

        let mut board = Board::new(2, 7, 7).expect("failed to create board");
        board
            .add_piece(0, Box::new(Knight::new()), knight_position)
            .expect("failed to add knight");
        board
            .add_piece(1, Box::new(Knight::new()), Coordinate(2, 1))
            .expect("failed to add second knight");
        board
            .add_piece(1, Box::new(Knight::new()), Coordinate(5, 2))
            .expect("failed to add third knight");

        let tests = [
            [false; 7],
            [false, false, true, false, true, false, false],
            [false, true, false, false, false, true, false],
            [false; 7],
            [false, true, false, false, false, true, false],
            [false, false, true, false, true, false, false],
            [false; 7],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = board
                    .is_valid_move(Move {
                        player: 0,
                        from: knight_position,
                        delta: CoordinateDelta(
                            i8::try_from(x).unwrap() - i8::try_from(knight_position.0).unwrap(),
                            i8::try_from(y).unwrap() - i8::try_from(knight_position.1).unwrap(),
                        ),
                        data: 0,
                    })
                    .expect("failed to validate move");
                assert!(
                    result == expected,
                    "test failed: {knight_position} -> ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }
}
