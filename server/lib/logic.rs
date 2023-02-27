use std::fmt::Display;

use crate::board::Board;

#[derive(Clone, Copy, Debug)]
pub struct Coordinate(pub i16, pub i16);

#[derive(Clone, Copy, Debug)]
pub struct CoordinateDelta(pub i8, pub i8);

impl Coordinate {
    #[must_use]
    pub fn add(&self, rhs: &CoordinateDelta, board: &Board) -> Self {
        Self(
            self.0 + i16::from(rhs.0),
            (self.1 + i16::from(rhs.1)).rem_euclid(board.height()),
        )
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Display for CoordinateDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub player: u8,
    pub from: Coordinate,
    pub delta: CoordinateDelta,
    pub data: u8,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {} -{}-> _, {}]",
            self.player, self.from, self.delta, self.data
        )
    }
}
