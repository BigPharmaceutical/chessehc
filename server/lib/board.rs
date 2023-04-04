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

pub type MovePartialDeltas<PieceId> = (Vec<PartialDelta<PieceId>>, u16);

impl<Set: PieceSet> Board<Set> {
    #[must_use]
    /// Create a new board
    pub fn new(width: u16, height: u16) -> Self {
        Self(vec![vec![Spot::new(); width as usize]; height as usize])
    }

    #[must_use]
    /// Get the board's height
    pub fn height(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    /// Get the board's width
    pub fn width(&self) -> usize {
        self.0.get(0).map_or(0, Vec::len)
    }

    /// Get a spot on the board
    ///
    /// # Errors
    /// - [`Error<Set>::CoordinateNotOnBoard`] - `coordinate` is not on the board
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

    /// Mutably get a spot on the board
    ///
    /// # Errors
    /// - [`Error<Set>::CoordinateNotOnBoard`] - `coordinate` is not on the board
    pub fn get_mut(&mut self, coordinate: &Coordinate) -> Result<&mut Spot<Set>, Error<Set>> {
        let (width, height) = (self.width(), self.height());

        self.0
            .get_mut(coordinate.1)
            .ok_or(Error::CoordinateNotOnBoard(*coordinate, width, height))?
            .get_mut(coordinate.0)
            .ok_or(Error::CoordinateNotOnBoard(*coordinate, width, height))
    }

    #[must_use]
    pub const fn raw(&self) -> &Vec<Vec<Spot<Set>>> {
        &self.0
    }

    /// Check if a piece at a spot is being attacked
    ///
    /// # Errors
    /// - [`Error<Set>::CoordinateNotOnBoard`] - `coordinate` is not on the board
    pub fn is_being_attacked(
        &self,
        coordinate: &Coordinate,
        player: u8,
    ) -> Result<bool, Error<Set>> {
        self.get(coordinate)
            .map(|spot| spot.is_being_attacked(player))
    }

    /// Check if a player is in check
    ///
    /// # Errors
    /// - [`Error<Set>::PieceError`] - An error from a piece
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

    /// Get the players in check
    ///
    /// # Errors
    /// - [`Error<Set>::PieceError`] - An error from a piece
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

    /// Add the attacks from a piece to the board
    ///
    /// # Errors
    /// - [`Error<Set>::PieceError`] - An error from `piece`
    /// - [`Error<Set>::CoordinateNotOnBoard`] - `piece` tried to attack a coordinate not on the board
    pub fn add_attacks(&mut self, piece: &Set, position: &Coordinate) -> Result<(), Error<Set>> {
        for attack in piece
            .attacking(self, position)
            .map_err(|err| Error::PieceError(err))?
        {
            self.get_mut(&attack)?
                .attack(piece.player(), *position, piece.blockable());
        }

        Ok(())
    }

    /// Remove the attacks from a piece on the board
    ///
    /// # Errors
    /// - [`Error<Set>::PieceError`] - An error from `piece`
    /// - [`Error<Set>::CoordinateNotOnBoard`] - `piece` tried to attack a coordinate not on the board
    pub fn remove_attacks(&mut self, piece: &Set, position: &Coordinate) -> Result<(), Error<Set>> {
        for attack in piece
            .attacking(self, position)
            .map_err(|err| Error::PieceError(err))?
        {
            self.get_mut(&attack)?.unattack(position);
        }

        Ok(())
    }

    /// Perform a delta on the board
    ///
    /// # Errors
    /// - [`Error<Set>::CoordinateNotOnBoard`] - A coordinate in the `delta` is not on the board
    /// - [`Error<Set>::NoPieceAtSpot`] - A coordinate in the `delta` which should contain a piece does not
    /// - [`Error<Set>::SpotOccupied`] - A coordinate in the `delta` which should not contain a piece does
    /// - [`Error<Set>::PieceError`] - Error from a piece
    pub fn perform_delta(
        &mut self,
        delta: Delta<Set>,
    ) -> Result<PartialDelta<Set::PieceId>, Error<Set>> {
        Ok(match delta {
            Delta::Move(from, to) => {
                let Some(piece) = self.get_mut(&from)?.take() else {
                    return Err(Error::NoPieceAtSpot(from));
                };

                let from_blocks = self.get(&from)?.blocking_spots();
                for block in &from_blocks {
                    let blocked_piece = self
                        .get(block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(*block))?;
                    self.remove_attacks(&blocked_piece, block)?;
                }

                self.remove_attacks(&piece, &from)?;
                self.add_attacks(&piece, &to)?;

                let to_blocks = self.get(&to)?.blocking_spots();
                for block in &to_blocks {
                    let blocked_piece = self
                        .get(block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(*block))?;
                    self.remove_attacks(&blocked_piece, block)?;
                }

                let target = self.get_mut(&to)?.replace(piece);
                if let Some(taken) = target {
                    return Err(Error::SpotOccupied(to, Some(taken)));
                };

                for block in from_blocks {
                    let blocked_piece = self
                        .get(&block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(block))?;
                    self.add_attacks(&blocked_piece, &block)?;
                }
                for block in to_blocks {
                    let blocked_piece = self
                        .get(&block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(block))?;
                    self.add_attacks(&blocked_piece, &block)?;
                }

                PartialDelta::<Set::PieceId>::Move(from, to)
            }
            Delta::Delete(position) => {
                let Some(taken) = self.get_mut(&position)?.take() else {
                    return Err(Error::NoPieceAtSpot(position));
                };

                self.remove_attacks(&taken, &position)?;

                let blocks = self.get(&position)?.blocking_spots();
                for block in blocks {
                    let blocked_piece = self
                        .get(&block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(block))?;
                    self.add_attacks(&blocked_piece, &block)?;
                }

                PartialDelta::<Set::PieceId>::Delete(position)
            }
            Delta::Replace(position, new_piece) => {
                let id = new_piece.type_id();
                let player = new_piece.player();

                let blocks = self.get(&position)?.blocking_spots();
                for block in &blocks {
                    let blocked_piece = self
                        .get(block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(*block))?;
                    self.remove_attacks(&blocked_piece, block)?;
                }

                if let Some(taken) = self.get_mut(&position)?.take() {
                    self.remove_attacks(&taken, &position)?;
                }

                self.add_attacks(&new_piece, &position)?;
                self.get_mut(&position)?.replace(new_piece);

                for block in blocks {
                    let blocked_piece = self
                        .get(&block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(block))?;
                    self.add_attacks(&blocked_piece, &block)?;
                }

                PartialDelta::<Set::PieceId>::Replace(position, id, player)
            }
        })
    }

    /// Attempts to make the move, assuming it has already
    /// been validated and returns the points gained
    ///
    /// # Errors
    /// - [`Error<Set>::NoPieceAtSpot`] - There is no piece at `move.to`
    /// - [`Error<Set>::PieceError`] - Error from a piece
    /// - [`Error<Set>::PieceOwnedByWrongPlayer`] - The current player does not own the piece to move
    /// - [`Error<Set>::PieceNotCapturable`] - The piece attempting to be captured is not capturable
    /// - [`Error<Set>::CoordinateNotOnBoard`] - A coordinate in a `delta` produced by the piece is not on the board
    /// - [`Error<Set>::SpotOccupied`] - A coordinate in a `delta` produced by the piece which should not contain a piece does
    pub fn make_move(
        &mut self,
        r#move: &Move,
        turn: u16,
        n_players: u8,
    ) -> Result<MovePartialDeltas<Set::PieceId>, Error<Set>> {
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
                spot.attack(r#move.player, r#move.to, piece.blockable());
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

    /// Remove a piece from the board
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
