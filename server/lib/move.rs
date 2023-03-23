use crate::coordinate::Coordinate;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
    pub data: u8,
    pub player: u8,
}

#[allow(clippy::module_name_repetitions)]
pub type PartialMove = (Coordinate, Coordinate, u8);

#[must_use]
pub fn partial_move_eq(lhs: &PartialMove, rhs: &PartialMove) -> bool {
    lhs.0 == rhs.0 && lhs.1 == rhs.1 && lhs.2 == rhs.2
}
