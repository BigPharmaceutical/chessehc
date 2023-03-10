use crate::{
    coordinate::Coordinate,
    delta::{Delta, PartialDelta},
    error::Error,
    piece_set::PieceSet,
    r#move::Move,
    spot::Spot,
};

#[derive(Clone)]
pub struct Board<Set: PieceSet>(Vec<Vec<Spot<Set>>>);

impl<Set: PieceSet> Board<Set> {
    pub fn new(width: u16, height: u16) -> Board<Set> {
        Self(vec![vec![Spot::new(); width as usize]; height as usize])
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        match self.0.get(0) {
            Some(rank) => rank.len(),
            None => 0,
        }
    }

    pub fn get(&self, coordinate: &Coordinate) -> Result<&Spot<Set>, Error<Set>> {
        self.0
            .get(coordinate.1)
            .ok_or(Error::CoordinateNotOnBoard(
                *coordinate,
                self.width(),
                self.height(),
            ))?
            .get(coordinate.0)
            .ok_or(Error::CoordinateNotOnBoard(
                *coordinate,
                self.width(),
                self.height(),
            ))
    }

    pub fn get_mut(&mut self, coordinate: &Coordinate) -> Result<&mut Spot<Set>, Error<Set>> {
        let (width, height) = (self.width(), self.height());

        self.0
            .get_mut(coordinate.1)
            .ok_or(Error::CoordinateNotOnBoard(*coordinate, width, height))?
            .get_mut(coordinate.0)
            .ok_or(Error::CoordinateNotOnBoard(*coordinate, width, height))
    }

    pub fn raw(&self) -> &Vec<Vec<Spot<Set>>> {
        &self.0
    }

    pub fn is_being_attacked(
        &self,
        coordinate: &Coordinate,
        player: u8,
    ) -> Result<bool, Error<Set>> {
        self.get(coordinate)
            .map(|spot| spot.is_being_attacked(player))
    }

    pub fn is_player_in_check(&self, player: u8) -> Result<bool, Error<Set>> {
        // For each spot on the board
        for (y, rank) in self.0.iter().enumerate() {
            for (x, spot) in rank.iter().enumerate() {
                // If it is empty, continue
                let Some(piece) = spot.get() else { continue };
                // If the piece does not belong to the player, continue
                if piece.player() != player {
                    continue;
                }

                // Unless the piece can be in check, continue
                let Some(is_in_check) = piece.is_in_check(self, &Coordinate(x, y)).map_err(|err| Error::PieceError(err))? else { continue };
                // If the piece is in check, return true
                if is_in_check {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    pub fn players_in_check(&self) -> Result<Vec<u8>, Error<Set>> {
        let mut in_check = Vec::new();

        // For each spot on the board
        for (y, rank) in self.raw().iter().enumerate() {
            for (x, spot) in rank.iter().enumerate() {
                // If it is empty, continue
                let Some(piece) = spot.get() else { continue };
                // If the player is already in check, continue
                if in_check.contains(&piece.player()) {
                    continue;
                }

                // Unless the piece can be in check, continue
                let Some(is_in_check) = piece.is_in_check(self, &Coordinate(x, y)).map_err(|err| Error::PieceError(err))? else { continue };
                // If the piece is in check, add the player to the list
                if is_in_check {
                    in_check.push(piece.player());
                }
            }
        }

        Ok(in_check)
    }

    pub fn add_attacks(&mut self, piece: &Set, position: &Coordinate) -> Result<(), Error<Set>> {
        for attack in piece
            .attacking(self, position)
            .map_err(|err| Error::PieceError(err))?
        {
            self.get_mut(&attack)?.attack(piece.player(), *position);
        }

        Ok(())
    }

    pub fn remove_attacks(&mut self, piece: &Set, position: &Coordinate) -> Result<(), Error<Set>> {
        for attack in piece
            .attacking(self, position)
            .map_err(|err| Error::PieceError(err))?
        {
            self.get_mut(&attack)?.unattack(position);
        }

        Ok(())
    }

    pub fn perform_delta(
        &mut self,
        delta: Delta<Set>,
    ) -> Result<PartialDelta<Set::PieceId>, Error<Set>> {
        Ok(match delta {
            Delta::Move(from, to) => {
                let Some(piece) = self.get_mut(&from)?.take() else {
                    return Err(Error::NoPieceAtSpot(from));
                };

                self.remove_attacks(&piece, &from)?;
                self.add_attacks(&piece, &to)?;

                let target = self.get_mut(&to)?.replace(piece);
                if let Some(taken) = target {
                    return Err(Error::SpotOccupied(to, Some(taken)));
                };

                PartialDelta::<Set::PieceId>::Move(from, to)
            }
            Delta::Delete(position) => {
                let Some(taken) = self.get_mut(&position)?.take() else {
                    return Err(Error::NoPieceAtSpot(position));
                };

                self.remove_attacks(&taken, &position)?;

                PartialDelta::<Set::PieceId>::Delete(position)
            }
            Delta::Replace(position, new_piece) => {
                let id = new_piece.type_id();
                let player = new_piece.player();

                if let Some(taken) = self.get_mut(&position)?.take() {
                    self.remove_attacks(&taken, &position)?;
                }

                self.add_attacks(&new_piece, &position)?;

                self.get_mut(&position)?.replace(new_piece);

                PartialDelta::<Set::PieceId>::Replace(position, id, player)
            }
        })
    }

    /// Attempts to make the move, assuming it has already
    /// been validated and returns the points gained
    pub fn make_move(
        &mut self,
        r#move: &Move,
        turn: u16,
        n_players: u8,
    ) -> Result<(Vec<PartialDelta<Set::PieceId>>, u16), Error<Set>> {
        let mut piece = self
            .get_mut(&r#move.from)?
            .take()
            .ok_or(Error::NoPieceAtSpot(r#move.from))?;

        for coordinate in piece
            .attacking(self, &r#move.from)
            .map_err(|err| Error::PieceError(err))?
        {
            if let Ok(spot) = self.get_mut(&coordinate) {
                spot.unattack(&r#move.from);
            }
        }

        let mut partial_deltas = vec![PartialDelta::Move(r#move.from, r#move.to)];

        let (deltas, mut points) = piece
            .mid_move(self, r#move, turn, n_players)
            .map_err(|err| Error::PieceError(err))?;

        for coordinate in piece
            .attacking(self, &r#move.to)
            .map_err(|err| Error::PieceError(err))?
        {
            if let Ok(spot) = self.get_mut(&coordinate) {
                spot.attack(r#move.player, r#move.to);
            }
        }

        let target = self.get_mut(&r#move.to)?.replace(piece);
        if let Some(taken) = target {
            if taken.player() == r#move.player {
                return Err(Error::PieceOwnedByWrongPlayer(r#move.to, taken.player()));
            }

            if let Some(capture_points) = taken.capture_points() {
                points += capture_points;
            } else {
                return Err(Error::PieceNotCapturable(r#move.to));
            }
        }

        for delta in deltas {
            partial_deltas.push(self.perform_delta(delta)?);
        }

        Ok((partial_deltas, points))
    }

    pub fn remove_player(&mut self, player: u8) -> Vec<PartialDelta<Set::PieceId>> {
        let mut partial_moves = Vec::new();

        for (y, rank) in self.0.iter_mut().enumerate() {
            for (x, spot) in rank.iter_mut().enumerate() {
                let Some(piece) = spot.get_mut() else { continue };
                if piece.player() != player {
                    continue;
                }

                spot.take();
                partial_moves.push(PartialDelta::Delete(Coordinate(x, y)));
            }
        }

        partial_moves
    }
}
