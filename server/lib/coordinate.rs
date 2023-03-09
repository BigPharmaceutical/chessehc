use std::{fmt::Display, ops::Add};

use crate::{board::Board, piece_set::PieceSet};

#[derive(Clone, Copy, Debug)]
pub struct Coordinate(pub usize, pub usize);

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

#[derive(Debug)]
pub struct CoordinateDelta(pub isize, pub isize);

impl Display for CoordinateDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "d({}, {})", self.0, self.1)
    }
}

// impl Coordinate {
//     pub fn add<Set: PieceSet>(&self, rhs: &CoordinateDelta, board: &Board<Set>) -> Option<Self> {
//         Some(Self(
//             match self.0.checked_add_signed(rhs.0)? {
//                 a if a < board.width() => a,
//                 _ => return None,
//             },
//             (self.0.wrapping_add_signed(rhs.1)).rem_euclid(board.height()),
//         ))
//     }
// }

impl<Set: PieceSet> Add<(&CoordinateDelta, &Board<Set>)> for &Coordinate {
    type Output = Option<Coordinate>;

    fn add(self, rhs: (&CoordinateDelta, &Board<Set>)) -> Self::Output {
        Some(Coordinate(
            match self.0.checked_add_signed(rhs.0 .0)? {
                a if a < rhs.1.width() => a,
                _ => return None,
            },
            usize::try_from(isize::try_from(self.1).ok()?.add(rhs.0 .1))
                .ok()?
                .rem_euclid(rhs.1.height()),
        ))
    }
}
