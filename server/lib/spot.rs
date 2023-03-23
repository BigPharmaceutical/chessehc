use crate::coordinate::Coordinate;

#[derive(Clone, Default)]
pub struct Spot<Set> {
    piece: Option<Set>,
    attackers: Vec<(u8, Coordinate, bool)>,
}

impl<Set> Spot<Set> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            piece: None,
            attackers: Vec::new(),
        }
    }
}

impl<Set> Spot<Set> {
    #[must_use]
    pub const fn is_occupied(&self) -> bool {
        self.piece.is_some()
    }

    #[must_use]
    pub const fn get(&self) -> &Option<Set> {
        &self.piece
    }

    pub fn get_mut(&mut self) -> &mut Option<Set> {
        &mut self.piece
    }

    #[must_use]
    pub fn is_being_attacked(&self, player: u8) -> bool {
        for (attacker, _, _) in &self.attackers {
            if attacker != &player {
                return true;
            }
        }

        false
    }

    pub fn take(&mut self) -> Option<Set> {
        self.piece.take()
    }

    pub fn replace(&mut self, piece: Set) -> Option<Set> {
        self.piece.replace(piece)
    }

    pub fn attack(&mut self, attacker: u8, from: Coordinate, blockable: bool) {
        if let Some(attack) = self.attackers.iter_mut().find(|(_, f, _)| f == &from) {
            attack.0 = attacker;
            return;
        }

        self.attackers.push((attacker, from, blockable));
    }

    pub fn unattack(&mut self, from: &Coordinate) {
        if let Some(i) = self.attackers.iter().position(|(_, f, _)| f == from) {
            self.attackers.remove(i);
        }
    }

    pub fn blocking_spots(&self) -> Vec<Coordinate> {
        self.attackers
            .iter()
            .filter_map(
                |(_, coordinate, blocking)| if *blocking { Some(*coordinate) } else { None },
            )
            .collect()
    }
}
