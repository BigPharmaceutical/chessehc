use std::{ops::Add, os::windows};

use crate::board::Board;

#[derive(Clone, Copy, Debug)]
pub struct Coordinate(pub usize, pub usize);

#[derive(Clone, Copy, Debug)]
pub struct CoordinateDelta(pub i8, pub i8);

impl Coordinate {
    pub fn add(&self, rhs: &CoordinateDelta, board: &Board) -> Self {
        Self(
            (self.0 as isize + rhs.0 as isize).rem_euclid(board.width() as isize) as usize,
            (self.1 as isize + rhs.1 as isize).rem_euclid(board.height() as isize) as usize,
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub player: u8,
    pub from: Coordinate,
    pub delta: CoordinateDelta,
    pub data: u8,
}
