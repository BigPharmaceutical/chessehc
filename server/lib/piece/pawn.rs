use crate::{
    board::Board,
    logic::{Coordinate, CoordinateDelta, Move},
};

use super::Piece;

#[derive(Debug)]
/// Pawn piece
/// (moves, move number for first move if moved two, direction)
pub struct Pawn(u16, Option<(u16, Coordinate)>, i8);

impl Pawn {
    #[must_use]
    pub const fn new(rank: i8) -> Self {
        Self(0, None, rank)
    }
}

impl Piece for Pawn {
    fn capture_points(&self) -> u8 {
        1
    }

    fn moves(&self) -> u16 {
        self.0
    }

    fn pawn_first_move(&self) -> Option<(u16, Coordinate)> {
        self.1
    }

    fn is_attacking(&self, board: &Board, from: &Coordinate, to: &Coordinate) -> bool {
        let dx = to.0 - from.0;
        if dx.abs() != 1 {
            return false;
        }

        let dy = if to.1 - from.1 == board.height() - 1 {
            -1
        } else {
            (to.1 - from.1).rem_euclid(board.height())
        };

        // Check the distance and direction of the attack
        if dy.abs() != 1 || dy.is_positive() != self.2.is_positive() {
            return false;
        }

        true
    }

    fn is_valid_move(
        &self,
        target: Option<&(u8, Box<dyn Piece>)>,
        board: &Board,
        r#move: &Move,
        to: &Coordinate,
    ) -> bool {
        // Check the direction of the move
        if r#move.delta.1.is_positive() != self.2.is_positive() {
            return false;
        }

        // If the move is to another piece, make sure it is +-1, +-1
        if target.is_some() {
            if r#move.delta.0.abs() == 1 && r#move.delta.1.abs() == 1 {
                return true;
            }
            return false;
        }

        // If the move is 0, +-1, then it is valid
        if r#move.delta.0 == 0 && r#move.delta.1.abs() == 1 {
            return true;
        }

        // If this is the first move and it is 0, +-2,  then it is valid
        if self.0 == 0 && r#move.delta.0 == 0 && r#move.delta.1.abs() == 2 {
            return board
                .get(
                    r#move
                        .from
                        .add(&CoordinateDelta(r#move.delta.0 / 2, 0), board),
                )
                .expect("could not get spot between two valid spots")
                .is_some();
        }

        // En passant
        if r#move.delta.0.abs() == 1 && r#move.delta.1.abs() == 1 {
            for delta in [CoordinateDelta(0, 1), CoordinateDelta(0, -1)] {
                let ep_target = to.add(&delta, board);
                if let Some(ep_piece) = board
                    .get(ep_target)
                    .expect("could not get spot for En Passant")
                {
                    // If that piece is on the same side, ignore it
                    if ep_piece.0 == r#move.player {
                        continue;
                    }

                    // If the piece is a pawn, who just made their first move
                    if let Some(pawn_move) = ep_piece.1.pawn_first_move() {
                        // and their turn was within the last round,
                        // then this pawn can perform En Passant
                        if board.turn() - pawn_move.0 < u16::from(board.players())
                            && pawn_move.1 == *to
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn mid_move(
        &mut self,
        board: &mut Board,
        r#move: &Move,
        to: &Coordinate,
    ) -> (u8, Option<Box<dyn Piece>>) {
        // If the pawn moved two places forward on its first move, set self.1 to the turn
        if self.0 == 0 && r#move.delta.0 == 0 && r#move.delta.1.abs() == 2 {
            self.1 = Some((
                board.turn(),
                r#move
                    .from
                    .add(&CoordinateDelta(0, r#move.delta.1 / 2), board),
            ));
        }

        self.0 += 1;

        // En passant
        if r#move.delta.0.abs() == 1 && r#move.delta.1.abs() == 1 {
            let mut points = 0;
            let turn = board.turn();
            let players = u16::from(board.players());

            for delta in [CoordinateDelta(0, 1), CoordinateDelta(0, -1)] {
                let ep_target = to.add(&delta, board);
                let ep_spot = board
                    .get_id(ep_target)
                    .expect("could not get spot for En Passant");

                if let Some(ep_piece_id) = ep_spot {
                    let ep_piece = board
                        .get_piece(*ep_piece_id)
                        .expect("could not pet piece for En Passant");
                    // If that piece is on the same side, ignore it
                    if ep_piece.0 == r#move.player {
                        continue;
                    }

                    // If the piece is a pawn, who just made their first move
                    if let Some(pawn_move) = ep_piece.1.pawn_first_move() {
                        // and their turn was within the last round,
                        // then this pawn can perform En Passant
                        if turn - pawn_move.0 < players && pawn_move.1 == *to {
                            let ep_spot = board
                                .get_id_mut(ep_target)
                                .expect("could not get spot mutably for En Passant");

                            let taken_id = ep_spot
                                .take()
                                .expect("tried to take a spot without a piece in En Passant");
                            points += board
                                .get_piece(taken_id)
                                .expect("error whilst getting piece taken in En Passant")
                                .1
                                .capture_points();
                        }
                    }
                }
            }

            return (points, None);
        }

        (0, None)
    }
}
