use crate::coordinate::Coordinate;

pub enum Delta<Set> {
    /// Move must not delete a piece
    Move(Coordinate, Coordinate),
    Delete(Coordinate),
    Replace(Coordinate, Set),
}

pub enum PartialDelta<PieceId> {
    Move(Coordinate, Coordinate),
    Delete(Coordinate),
    Replace(Coordinate, PieceId, u8),
    Checkmate(u8),
    Stalemate(u8),
}
