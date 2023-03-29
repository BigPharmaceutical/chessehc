use crate::{
    board::Board,
    coordinate::Coordinate,
    delta::{Delta, PartialDelta},
    error::Error,
    piece_set::PieceSet,
    r#move::{Move, PartialMove},
};

pub struct Game<Set: PieceSet> {
    players: Vec<(bool, u16)>,
    board: Board<Set>,
    turn: (u16, u8),
    valid_moves: Vec<PartialMove>,
}

pub type AttemptedMove<Set, PieceId> = Option<(Board<Set>, Vec<PartialDelta<PieceId>>, u16)>;

impl<Set: PieceSet> Game<Set> {
    #[must_use]
    pub fn new(n_players: u8, width: u16, height: u16) -> Self {
        Self {
            players: vec![(true, 0); n_players as usize],
            board: Board::new(width, height),
            turn: (0, 0),
            valid_moves: Vec::with_capacity(0),
        }
    }

    /// Add a piece to the board
    /// 
    /// # Errors
    /// - [`Error<Set>::CoordinateNotOnBoard`] - `position` is not on the board
    /// - [`Error<Set>::SpotOccupied`] - A piece is already in the `position`
    /// - [`Error<Set>::PieceError`] - Error from a piece
    pub fn add_piece(
        &mut self,
        piece: Set,
        position: &Coordinate,
    ) -> Result<PartialDelta<Set::PieceId>, Error<Set>> {
        let spot = self.board.get(position)?;
        if spot.is_occupied() {
            return Err(Error::SpotOccupied(*position, Some(piece)));
        }

        let delta = Delta::Replace(*position, piece);
        let partial_delta = self.board.perform_delta(delta)?;

        self.generate_valid_moves()?;
        Ok(partial_delta)
    }

    /// Generate the valid moves for the current player
    /// 
    /// # Errors
    /// - [`Error<Set>::PieceError`] - Error from a piece
    pub fn generate_valid_moves(&mut self) -> Result<(), Error<Set>> {
        self.valid_moves = Vec::new();

        let n_players_in_play = u8::try_from(
            self.players
                .iter()
                .filter(|&&(is_in_play, _)| is_in_play)
                .count(),
        )
        .expect("exceeded maximum number of players in play");

        for (y, rank) in self.board.raw().iter().enumerate() {
            for (x, spot) in rank.iter().enumerate() {
                let Some(piece) = spot.get() else { continue };
                if piece.player() != self.turn.1 {
                    continue;
                }

                let from = Coordinate(x, y);

                for (to, data) in piece
                    .valid_moves(&self.board, &from, self.turn.0, n_players_in_play)
                    .map_err(|err| Error::PieceError(err))?
                {
                    let r#move = Move {
                        from,
                        to,
                        data,
                        player: piece.player(),
                    };

                    if self.attempt_move(&r#move)?.is_none() {
                        continue;
                    };

                    self.valid_moves.push((Coordinate(x, y), to, data));
                }
            }
        }
        Ok(())
    }

    #[must_use]
    /// Get the valid moves for the current player
    pub const fn valid_moves(&self) -> &Vec<PartialMove> {
        &self.valid_moves
    }

    /// Attempt a move on the board
    /// 
    /// Returns an option with the board and partial moves if the move can be made
    /// 
    /// # Errors
    /// - [`Error<Set>::CoordinateNotOnBoard`] - A coordinate in `move` is not on the board
    /// - [`Error<Set>::PieceError`] - Error from a piece
    pub fn attempt_move(
        &self,
        r#move: &Move,
    ) -> Result<AttemptedMove<Set, Set::PieceId>, Error<Set>> {
        if r#move.player != self.turn.1 {
            return Ok(None);
        }

        let n_players_in_play = u8::try_from(
            self.players
                .iter()
                .filter(|&&(is_in_play, _)| is_in_play)
                .count(),
        )
        .expect("exceeded maximum number of players in play");

        let mut new_state = self.board.clone();
        let (partial_deltas, points) =
            new_state.make_move(r#move, self.turn.0, n_players_in_play)?;

        if new_state.is_player_in_check(r#move.player)? {
            return Ok(None);
        }

        Ok(Some((new_state, partial_deltas, points)))
    }

    /// Increment turn and select the next player
    pub fn increment_turn(&mut self) {
        let players_n =
            u8::try_from(self.players.len()).expect("exceeded maximum number of players in game");

        // Update turn number
        self.turn.0 += 1;

        // Update which player's turn it is
        for i in 0..players_n {
            let j = (self.turn.1 + i).rem_euclid(players_n);
            if let Some(&(is_in_game, _)) = self.players.get(j as usize) {
                if is_in_game {
                    self.turn.1 = j;
                    break;
                }
            }
        }
    }

    /// Start the next turn by incrementing turn,
    /// checking for check and generating the valid moves
    /// 
    /// # Errors
    /// - [`Error<Set>::PieceError`] - An error from a piece
    pub fn start_turn(&mut self) -> Result<Vec<PartialDelta<Set::PieceId>>, Error<Set>> {
        self.increment_turn();

        // Check if the current player is in check
        let player_in_check = self.board.is_player_in_check(self.turn.1)?;

        // Update the current player's valid moves
        self.generate_valid_moves()?;

        let mut partial_deltas = Vec::new();

        if self.valid_moves.is_empty() {
            let deletion_moves = self.board.remove_player(self.turn.1);
            if deletion_moves.is_empty() {
                return Ok(Vec::with_capacity(0));
            }
            partial_deltas = deletion_moves;

            partial_deltas.push(if player_in_check {
                PartialDelta::Checkmate(self.turn.1)
            } else {
                PartialDelta::Stalemate(self.turn.1)
            });

            if let Some((is_in_game, _)) = self.players.get_mut(self.turn.1 as usize) {
                *is_in_game = false;
            }

            partial_deltas.extend(self.start_turn()?);
        }

        Ok(partial_deltas)
    }

    /// Make a move
    /// 
    /// # Errors
    /// - [`Error<Set>::CoordinateNotOnBoard`] - A coordinate in `move` is not on the board
    /// - [`Error<Set>::PieceError`] - An error from a piece
    pub fn make_move(
        &mut self,
        r#move: &Move,
    ) -> Result<Vec<PartialDelta<Set::PieceId>>, Error<Set>> {
        let Some((new_state, partial_deltas, points)) = self.attempt_move(r#move)? else {
            return Err(Error::InvalidMove(*r#move));
        };
        self.board = new_state;

        let (_, score) = self
            .players
            .get_mut(r#move.player as usize)
            .expect("exceeded maximum number of players in game");
        *score += points;

        Ok(partial_deltas)
    }

    #[must_use]
    /// Get the board
    pub const fn board(&self) -> &Board<Set> {
        &self.board
    }

    /// Remove a player from the game
    pub fn remove_player(&mut self, player: u8) -> Vec<PartialDelta<Set::PieceId>> {
        self.increment_turn();
        self.turn.0 -= 1;

        self.board.remove_player(player)
    }
}
