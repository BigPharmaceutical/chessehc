use crate::{
    board::Board,
    coordinate::{Coordinate, CoordinateDelta},
    delta::Delta,
    r#move::Move,
};

use super::{Bishop, Error, Knight, Queen, Rook, StandardCompatiblePiece};

#[derive(Clone, Debug)]
pub struct Pawn {
    player: u8,
    has_moved: bool,
    first_double_move: Option<(Coordinate, u16)>,
    direction: i8,
    upgrade_rank: usize,
}

impl Pawn {
    pub fn new(player: u8, direction: i8, upgrade_rank: usize) -> Box<dyn StandardCompatiblePiece> {
        Box::new(Self {
            player,
            has_moved: false,
            first_double_move: None,
            direction,
            upgrade_rank,
        })
    }
}

const UPGRADE_NUMBERS: [u8; 4] = [2, 3, 4, 5];

impl StandardCompatiblePiece for Pawn {
    fn type_id(&self) -> u8 {
        1
    }

    fn capture_points(&self) -> Option<u16> {
        Some(1)
    }

    fn blockable(&self) -> bool {
        false
    }

    fn player(&self) -> u8 {
        self.player
    }

    fn attacking(
        &self,
        board: &Board<Box<dyn StandardCompatiblePiece>>,
        from: &Coordinate,
    ) -> Result<Vec<Coordinate>, Error> {
        let mut attacks = Vec::with_capacity(2);

        for dx in [-1, 1] {
            let Some(position) = from + (&CoordinateDelta(dx, self.direction.into()), board) else { continue };
            attacks.push(position);
        }

        Ok(attacks)
    }

    fn valid_moves(
        &self,
        board: &Board<Box<dyn StandardCompatiblePiece>>,
        from: &Coordinate,
        turn: u16,
        n_players: u8,
    ) -> Result<Vec<(Coordinate, u8)>, Error> {
        let mut moves = Vec::new();

        // Standard
        if let Some(position) = from + (&CoordinateDelta(0, self.direction as isize), board) {
            if let Ok(spot) = board.get(&position) {
                if !spot.is_occupied() {
                    moves.push((position, 0));
                }
            }
        }

        // First move
        if !self.has_moved {
            if let (Some(position), Some(intermediate)) = (
                from + (&CoordinateDelta(0, self.direction as isize * 2), board),
                from + (&CoordinateDelta(0, self.direction as isize), board),
            ) {
                if let (Ok(spot), Ok(intermediate)) =
                    (board.get(&position), board.get(&intermediate))
                {
                    if !spot.is_occupied() && !intermediate.is_occupied() {
                        moves.push((position, 0));
                    }
                }
            }
        }

        // Attack
        'attack_positions: for dx in [-1, 1] {
            let Some(position) = from + (&CoordinateDelta(dx, self.direction as isize), board) else { continue };

            let Ok(Some(piece)) = board.get(&position).map(|spot| spot.get()) else {
                // En Passant
                for ldy in [-1, 1] {
                    let Some(ep_position) = &position + (&CoordinateDelta(0, ldy), board) else { continue };
                    let Ok(Some(ep_piece)) = board.get(&ep_position).map(|spot| spot.get()) else { continue };

                    if ep_piece.player() != self.player && ep_piece.can_en_passant(&position, turn, n_players) {
                        moves.push((position, 0));
                        continue 'attack_positions;
                    }
                }

                continue;
            };

            if piece.player() == self.player || piece.capture_points().is_none() {
                continue;
            }

            moves.push((position, 0));
        }

        Ok(moves
            .iter()
            .flat_map(|r#move| {
                if r#move.0 .1 == self.upgrade_rank {
                    UPGRADE_NUMBERS
                        .iter()
                        .map(|&upgrade| (r#move.0, upgrade))
                        .collect()
                } else {
                    vec![*r#move]
                }
            })
            .collect())
    }

    fn mid_move(
        &mut self,
        board: &Board<Box<dyn StandardCompatiblePiece>>,
        r#move: &Move,
        turn: u16,
        n_players_in_play: u8,
    ) -> Result<(Vec<Delta<Box<dyn StandardCompatiblePiece>>>, u16), Error> {
        let delta = CoordinateDelta(
            isize::try_from(r#move.to.0).expect("move too large")
                - isize::try_from(r#move.from.0).expect("move too large"),
            {
                let mut dy = isize::try_from(r#move.to.1)
                    .map_err(|err| Error::PositionOrDeltaTooLarge(r#move.to.1, err))?
                    - isize::try_from(r#move.from.1)
                        .map_err(|err| Error::PositionOrDeltaTooLarge(r#move.from.1, err))?;
                if i8::try_from(dy.signum()).unwrap() != self.direction {
                    dy += isize::from(self.direction)
                        * isize::try_from(board.height())
                            .map_err(|err| Error::PositionOrDeltaTooLarge(r#move.from.1, err))?;
                }
                dy
            },
        );

        if delta.1 == 2 && !self.has_moved {
            self.first_double_move = Some((
                (&r#move.from + (&CoordinateDelta(0, self.direction.into()), board))
                    .expect("failed to add 1 to y in coordinate"),
                turn,
            ))
        }

        let mut deltas = Vec::new();
        let mut points = 0;

        // En Passant
        for ldy in [-1, 1] {
            let Some(ep_position) =
                &r#move.to + (&CoordinateDelta(0, self.direction as isize + ldy), board) else { continue };
            let Ok(Some(ep_piece)) = board.get(&ep_position).map(|spot| spot.get()) else { continue };

            let Some(ep_points) = ep_piece.capture_points() else { continue };
            if ep_piece.player() != self.player
                && ep_piece.can_en_passant(&r#move.to, turn, n_players_in_play)
            {
                deltas.push(Delta::Delete(ep_position));
                points += ep_points;
            }
        }

        self.has_moved = true;

        if r#move.to.1 == self.upgrade_rank {
            deltas.push(Delta::Replace(
                r#move.to,
                match r#move.data {
                    2 => Bishop::new(self.player),
                    3 => Knight::new(self.player),
                    4 => Rook::new(self.player),
                    5 => Queen::new(self.player),
                    _ => return Err(Error::InvalidPieceId(r#move.data)),
                },
            ));
        }

        Ok((deltas, points))
    }

    fn clone(&self) -> Box<dyn StandardCompatiblePiece> {
        Box::new(Clone::clone(self))
    }

    fn can_en_passant(&self, intermediate: &Coordinate, turn: u16, n_players: u8) -> bool {
        if let Some((ep_intermediate, ep_turn)) = self.first_double_move {
            &ep_intermediate == intermediate && turn - ep_turn < n_players.into()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{coordinate::Coordinate, game::Game};

    use super::Pawn;

    #[test]
    fn attacking() {
        const PAWN_POSITION: Coordinate = Coordinate(2, 1);

        let mut game = Game::new(1, 5, 4);
        game.add_piece(Pawn::new(0, 1, 20), &PAWN_POSITION).unwrap();

        let tests = [
            [false; 5],
            [false; 5],
            [false, true, false, true, false],
            [false; 5],
        ];

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let result = game
                    .board()
                    .is_being_attacked(&Coordinate(x, y), 1)
                    .unwrap();

                assert!(
                    result == expected,
                    "test failed: {PAWN_POSITION} -x ({x}, {y}), {result} ({expected})"
                );
            }
        }
    }

    #[test]
    fn moves() {
        const PAWN_POSITION: Coordinate = Coordinate(2, 1);

        let mut game = Game::new(1, 5, 4);
        game.add_piece(Pawn::new(0, 1, 20), &PAWN_POSITION).unwrap();

        let tests = [
            [false; 5],
            [false; 5],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ];

        game.generate_valid_moves().unwrap();
        let valid_moves = game.valid_moves();

        for (from, _, _) in valid_moves {
            assert!(
                from == &PAWN_POSITION,
                "test failed: {from} != {PAWN_POSITION}"
            );
        }

        for (y, rank) in tests.iter().enumerate() {
            for (x, &expected) in rank.iter().enumerate() {
                let position = Coordinate(x, y);

                let result = valid_moves.iter().find(|(_, to, _)| to == &position);

                assert!(
                    matches!(result, Some(_)) == expected,
                    "test failed: {PAWN_POSITION} -x ({x}, {y}), {} ({expected})",
                    !expected
                );
            }
        }
    }
}
