use crate::{
    board::{Board, Error},
    logic::{Coordinate, CoordinateDelta},
};

use super::{Piece, Pieces};

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

    fn blockable(&self) -> bool {
        false
    }

    fn moves(&self) -> u16 {
        self.0
    }

    fn add_attacks(&self, board: &mut Board, piece_id: usize, from: Coordinate) {
        for dir in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
            let position_1 = from.add(&CoordinateDelta(dir.0 * 2, dir.1), board);
            let result_1 = board.attack(position_1, piece_id);
            match &result_1 {
                Err(Error::CoordinateNotOnBoard(..)) => continue,
                Err(_) => result_1.unwrap(),
                _ => (),
            }

            let position_2 = from.add(&CoordinateDelta(dir.0, dir.1 * 2), board);
            let result_2 = board.attack(position_2, piece_id);
            match &result_2 {
                Err(Error::CoordinateNotOnBoard(..)) => continue,
                Err(_) => result_2.unwrap(),
                _ => (),
            }
            println!("{from} -x {position_1}, {position_2}");
        }
    }

    fn is_valid_move(
        &self,
        _board: &Board,
        _pieces: &Pieces,
        _target: Option<&(u8, Box<dyn Piece>)>,
        r#move: &crate::logic::Move,
        _to: &Coordinate,
    ) -> bool {
        // Check the distances of the move
        (r#move.delta.0.abs() == 1 && r#move.delta.1.abs() == 2)
            || (r#move.delta.0.abs() == 2 && r#move.delta.1.abs() == 1)
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

    use super::Knight;

    #[test]
    fn attacking() {
        let knight_1_position = Coordinate(3, 3);
        let knight_2_position = Coordinate(2, 1);
        let knight_3_position = Coordinate(5, 2);

        let mut pieces = Pieces::new_with_capacity(3);
        let mut board = Board::new(2, 7, 7).expect("failed to create board");

        let knight_1_id = pieces.push(0, Box::new(Knight::new()));
        let knight_2_id = pieces.push(1, Box::new(Knight::new()));
        let knight_3_id = pieces.push(1, Box::new(Knight::new()));

        board
            .add_piece(&pieces, knight_1_id, knight_1_position)
            .expect("failed to add knight");
        board
            .add_piece(&pieces, knight_2_id, knight_2_position)
            .expect("failed to add second knight");
        board
            .add_piece(&pieces, knight_3_id, knight_3_position)
            .expect("failed to add third knight");

        let _knight = pieces.get(knight_1_id).expect("failed to get knight");

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
                    .get_spot(Coordinate(
                        i16::try_from(x).unwrap(),
                        i16::try_from(y).unwrap(),
                    ))
                    .unwrap()
                    .is_being_attacked(&pieces, 1);
                assert!(
                    result == expected,
                    "test failed: {knight_1_position} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moving() {
        let knight_1_position = Coordinate(3, 3);
        let knight_2_position = Coordinate(2, 1);
        let knight_3_position = Coordinate(5, 2);

        let mut pieces = Pieces::new_with_capacity(3);
        let mut board = Board::new(2, 7, 7).expect("failed to create board");

        let knight_1_id = pieces.push(0, Box::new(Knight::new()));
        let knight_2_id = pieces.push(1, Box::new(Knight::new()));
        let knight_3_id = pieces.push(1, Box::new(Knight::new()));

        board
            .add_piece(&pieces, knight_1_id, knight_1_position)
            .expect("failed to add knight");
        board
            .add_piece(&pieces, knight_2_id, knight_2_position)
            .expect("failed to add second knight");
        board
            .add_piece(&pieces, knight_3_id, knight_3_position)
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
                    .is_valid_move(
                        &pieces,
                        Move {
                            player: 0,
                            from: knight_1_position,
                            delta: CoordinateDelta(
                                i8::try_from(x).unwrap()
                                    - i8::try_from(knight_1_position.0).unwrap(),
                                i8::try_from(y).unwrap()
                                    - i8::try_from(knight_1_position.1).unwrap(),
                            ),
                            data: 0,
                        },
                    )
                    .expect("failed to validate move");
                assert!(
                    result == expected,
                    "test failed: {knight_1_position} -> ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }
}
