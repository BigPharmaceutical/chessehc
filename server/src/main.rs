use chessehc::{
    self,
    logic::{Coordinate, CoordinateDelta, Move},
    piece::Pawn,
};

fn main() {
    let mut board = chessehc::board::Board::new(2, 4, 5).unwrap();

    board
        .add_piece(0, Box::new(Pawn::new(1)), Coordinate(1, 1))
        .unwrap();
    board
        .add_piece(0, Box::new(Pawn::new(-1)), Coordinate(2, 3))
        .unwrap();

    let move1 = Move {
        player: 0,
        from: Coordinate(1, 1),
        delta: CoordinateDelta(0, 2),
        data: 0,
    };

    let move2 = Move {
        player: 1,
        from: Coordinate(2, 3),
        delta: CoordinateDelta(-1, -1),
        data: 0,
    };

    let move1v = board.is_valid_move(move1).unwrap();
    println!("{}", move1v);
    board.make_move(move1).unwrap();

    println!(
        "{:?}",
        board.get(Coordinate(1, 3)).unwrap().get(&board).unwrap().1
    );
}
