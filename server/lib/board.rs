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

    #[must_use]
    pub fn raw_mut(&mut self) -> &mut Vec<Vec<Spot<Set>>> {
        &mut self.0
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

    /// Applies a delta to the board
    ///
    /// # Errors
    /// - [`Error<Set>::CoordinateNotOnBoard`] - A coordinate in the `delta` is not on the board
    /// - [`Error<Set>::NoPieceAtSpot`] - A coordinate in the `delta` which should contain a piece does not
    /// - [`Error<Set>::SpotOccupied`] - A coordinate in the `delta` which should not contain a piece does
    /// - [`Error<Set>::PieceError`] - Error from a piece
    pub fn apply_delta(
        &mut self,
        delta: Delta<Set>,
    ) -> Result<PartialDelta<Set::PieceId>, Error<Set>> {
        Ok(match delta {
            Delta::Move(from, to) => {
                // Take the piece at `from`
                let Some(piece) = self.get_mut(&from)?.take() else {
                    return Err(Error::NoPieceAtSpot(from));
                };

                // Remove the attacks from the piece
                self.remove_attacks(&piece, &from)?;

                // Remove the attacks for the pieces blocked by the piece
                let from_blocks = self.get(&from)?.blocking_spots();
                for block in &from_blocks {
                    let blocked_piece = self
                        .get(block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(*block))?;
                    self.remove_attacks(&blocked_piece, block)?;
                }

                // Add the attacks from the piece
                self.add_attacks(&piece, &to)?;

                // Remove the attacks for the pieces now blocked by the piece
                let to_blocks = self.get(&to)?.blocking_spots();
                for block in &to_blocks {
                    let blocked_piece = self
                        .get(block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(*block))?;
                    self.remove_attacks(&blocked_piece, block)?;
                }

                // Place the piece into its new spot
                let target = self.get_mut(&to)?.replace(piece);
                if let Some(taken) = target {
                    return Err(Error::SpotOccupied(to, Some(taken)));
                };

                // Add the attacks for the pieces that were blocked by the piece
                for block in from_blocks {
                    let blocked_piece = self
                        .get(&block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(block))?;
                    self.add_attacks(&blocked_piece, &block)?;
                }

                // Add the attacks for the pieces now blocked by the piece
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
                // Take the piece
                let Some(taken) = self.get_mut(&position)?.take() else {
                    return Err(Error::NoPieceAtSpot(position));
                };

                // Remove the piece's attacks
                self.remove_attacks(&taken, &position)?;

                // Re-add the attacks for the pieces that were blocked by the piece
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
                // Get the new piece's data
                let id = new_piece.type_id();
                let player = new_piece.player();

                // Remove the attacks for the pieces blocked by the piece
                let blocks = self.get(&position)?.blocking_spots();
                for block in &blocks {
                    let blocked_piece = self
                        .get(block)?
                        .get()
                        .clone()
                        .ok_or_else(|| Error::NoPieceAtSpot(*block))?;
                    self.remove_attacks(&blocked_piece, block)?;
                }

                // If there is a piece in the spot, take it and remove its attacks
                if let Some(taken) = self.get_mut(&position)?.take() {
                    self.remove_attacks(&taken, &position)?;
                }

                // Add the attacks for the new piece
                self.add_attacks(&new_piece, &position)?;
                // Place the piece
                self.get_mut(&position)?.replace(new_piece);

                // Add the attacks for the pieces that were blocked
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
        // Get the piece to be moved
        let mut piece = self
            .get_mut(&r#move.from)?
            .take()
            .ok_or(Error::NoPieceAtSpot(r#move.from))?;

        // Remove the piece's attacks
        for coordinate in piece
            .attacking(self, &r#move.from)
            .map_err(|err| Error::PieceError(err))?
        {
            if let Ok(spot) = self.get_mut(&coordinate) {
                spot.unattack(&r#move.from);
            }
        }

        // Re-add the attacks for the pieces that were blocked by the piece
        let from_blocks = self.get(&r#move.from)?.blocking_spots();
        for block in from_blocks {
            let blocked_piece = self
                .get(&block)?
                .get()
                .clone()
                .ok_or_else(|| Error::NoPieceAtSpot(block))?;
            self.add_attacks(&blocked_piece, &block)?;
        }

        let mut partial_deltas = vec![PartialDelta::Move(r#move.from, r#move.to)];

        // Get the piece's mid-move deltas
        let (deltas, mut points) = piece
            .mid_move(self, r#move, turn, n_players)
            .map_err(|err| Error::PieceError(err))?;

        // Add the piece's new attacks
        for coordinate in piece
            .attacking(self, &r#move.to)
            .map_err(|err| Error::PieceError(err))?
        {
            if let Ok(spot) = self.get_mut(&coordinate) {
                spot.attack(r#move.player, r#move.to, piece.blockable());
            }
        }

        // Remove the attacks for the pieces that will be blocked by the piece
        let to_blocks = self.get(&r#move.to)?.blocking_spots();
        for block in &to_blocks {
            let blocked_piece = self
                .get(block)?
                .get()
                .clone()
                .ok_or_else(|| Error::NoPieceAtSpot(*block))?;
            self.remove_attacks(&blocked_piece, block)?;
        }

        // Place the piece and get the taken piece
        let target = self.get_mut(&r#move.to)?.replace(piece);
        if let Some(taken) = target {
            // Stop taking own piece
            if taken.player() == r#move.player {
                return Err(Error::PieceOwnedByWrongPlayer(r#move.to, taken.player()));
            }

            // Add capture points and stop taking a non-capturable piece
            if let Some(capture_points) = taken.capture_points() {
                points += capture_points;
            } else {
                return Err(Error::PieceNotCapturable(r#move.to));
            }
        }

        // Add the attacks for the pieces that are now blocked by the piece
        for block in to_blocks {
            let blocked_piece = self
                .get(&block)?
                .get()
                .clone()
                .ok_or_else(|| Error::NoPieceAtSpot(block))?;
            self.add_attacks(&blocked_piece, &block)?;
        }

        // Perform the mid-move deltas
        for delta in deltas {
            partial_deltas.push(self.apply_delta(delta)?);
        }

        Ok((partial_deltas, points))
    }

    /// Remove a player from the board
    /// 
    /// # Errors
    /// - Error from `apply_delta`
    /// 
    /// # Panics
    /// Will panic if it cannot get a valid spot on the board
    pub fn remove_player(
        &mut self,
        player: u8,
    ) -> Result<Vec<PartialDelta<Set::PieceId>>, Error<Set>> {
        let mut partial_deltas = Vec::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let coordinate = Coordinate(x, y);

                let Some(piece) = self
                    .get(&coordinate)
                    .unwrap_or_else(|_| panic!("could not get {coordinate} within board of {} x {}", self.width(), self.height()))
                    .get() else { continue };
                if piece.player() != player {
                    continue;
                }

                // Remove the player's piece
                partial_deltas.push(self.apply_delta(Delta::Delete(Coordinate(x, y)))?);
            }
        }

        Ok(partial_deltas)
    }

    #[must_use]
    pub fn export(&self) -> (usize, usize, Vec<(u8, Set::PieceId)>) {
        (
            self.width(),
            self.height(),
            self.0
                .iter()
                .flat_map(|rank| {
                    rank.iter().map(|spot| {
                        spot.get()
                            .as_ref()
                            .map_or((0, Set::NONE_ID), |piece| (piece.player(), piece.type_id()))
                    })
                })
                .collect(),
        )
    }
}
