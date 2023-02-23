use crate::board::Coordinate;

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
    pub data: u8,
}
