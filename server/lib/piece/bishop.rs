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
        for dy in [dy_a, dy_b] {
            // Check that the move is diagonal and non-zero
            if dx.abs() != dy.abs() || dx == 0 {
                return false;
            }

            // Check that no piece is between the origin and target
            for d in if dx.is_positive() { 1..dx } else { (dx + 1)..0 } {
                let d = match i8::try_from(d) {
                    Ok(value) => value,
                    Err(_) => continue,
                };
                let position = from.add(&CoordinateDelta(d, d), board);
                if board
                    .get(position)
                    .expect("could not get spot between two valid spots")
                    .is_some()
                {
                    return false;
                }
            }
        }

        true
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
        for d in if r#move.delta.0.is_positive() {
            1..r#move.delta.0
        } else {
            (r#move.delta.0 + 1)..0
        } {
            let position = r#move.from.add(&CoordinateDelta(d, d), board);
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
}
