use crate::{
    board::Board,
    logic::{CoordinateDelta, Move, Coordinate},
};

use super::Piece;

/// Pawn piece
/// (moves, move number for first move if moved two, direction)
pub struct Pawn(u16, Option<u16>, i8, Vec<Coordinate>);

impl Piece for Pawn {
    fn capture_points(&self) -> u8 {
        1
    }

    fn blockable(&self) -> bool {
        false
    }

    fn moves(&self) -> u16 {
        self.0
    }

    fn pawn_first_move(&self) -> Option<u16> {
        self.1
    }
    
        fn add_attacks(&mut self, board: &mut Board, piece_id: usize, from: Coordinate) {
            for delta in [CoordinateDelta(-1, self.2), CoordinateDelta(1, self.2)] {
                let to = from.add(&delta, board);
    
                board.attack(to, piece_id).ok();
            }
        }

    fn remove_attacks(&mut self, board: &mut Board, piece_id: usize, from: Coordinate) {
        for delta in [CoordinateDelta(-1, self.2), CoordinateDelta(1, self.2)] {
            let to = from.add(&delta, board);

            board.unattack(to, piece_id).ok();
        }
    }

    fn is_valid_move(
        &self,
        target: &Option<(u8, Box<dyn Piece>)>,
        board: &Board,
        r#move: &Move,
    ) -> bool {
        // Check the direction of the move
        if r#move.delta.1.is_positive() != self.2.is_positive() {
            return false;
        }

        if target.is_some() {
            if r#move.delta.0 == 1 && r#move.delta.1 == 1 {
                return true;
            }
        } else {
            if self.0 == 0 && r#move.delta.0 == 0 && r#move.delta.1.abs() == 2 {
                return board
                    .get(
                        r#move
                            .from
                            .add(&CoordinateDelta(r#move.delta.0 / 2, 0), board),
                    )
                    .expect("could not get place between two valid places")
                    .is_occupied();
            }

            if r#move.delta.0 == 0 && r#move.delta.1 == 1 {
                return true;
            }
        }

        todo!("En Passant");

        false
    }

    fn mid_move(&mut self, board: &mut Board, r#move: &Move) -> Option<Box<dyn Piece>> {
        self.0 += 1;

        // If the pawn moved two places forward on its first move, set self.1 to the turn
        if self.0 == 0 && r#move.delta.0 == 0 && r#move.delta.1 == 2 {
            self.1 = Some(board.turn());
        }

        todo!("En Passant");

        None
    }
}
