use chessehc::{
    self,
    logic::{Coordinate, CoordinateDelta, Move},
    piece::{Bishop, Pawn, Queen},
};

fn main() {
    println!("THIS IS CURRENTLY A TEST FILE FOR THE LIBRARY!");

    let mut board = chessehc::board::Board::new(3, 4, 5).unwrap();

    board
        .add_piece(0, Box::new(Pawn::new(1)), Coordinate(1, 1))
        .unwrap();
    board
        .add_piece(1, Box::new(Bishop::new()), Coordinate(2, 1))
        .unwrap();
    board
        .add_piece(2, Box::new(Pawn::new(-1)), Coordinate(2, 3))
        .unwrap();
    board
        .add_piece(0, Box::new(Queen::new()), Coordinate(3, 0))
        .unwrap();

    for (test, expected) in &[
        (Coordinate(3, 2), true),
        (Coordinate(3, 0), true),
        (Coordinate(1, 2), true),
        (Coordinate(1, 0), true),
        (Coordinate(3, 3), false),
        (Coordinate(0, 0), false),
        (Coordinate(1, 4), false),
    ] {
        let a = board
            .get_piece(1)
            .unwrap()
            .1
            .is_attacking(&board, &Coordinate(2, 1), test);
        println!(
            "test: {test} -> {a} ({expected}) {}",
            if a == *expected { '✅' } else { '❌' }
        );
    }

    let move1 = Move {
        player: 0,
        from: Coordinate(1, 1),
        delta: CoordinateDelta(0, 2),
        data: 0,
    };

    let move2 = Move {
        player: 1,
        from: Coordinate(2, 1),
        delta: CoordinateDelta(-1, 1),
        data: 0,
    };

    let move3 = Move {
        player: 2,
        from: Coordinate(2, 3),
        delta: CoordinateDelta(-1, -1),
        data: 0,
    };

    let move1v = board.is_valid_move(move1).unwrap();
    println!("Move 1: {}", move1v);
    board.make_move(move1).unwrap();

    let move2v = board.is_valid_move(move2).unwrap();
    println!("Move 2: {}", move2v);
    board.make_move(move2).unwrap();

    let move3v = board.is_valid_move(move3).unwrap();
    println!("Move 3: {}", move3v);
    board.make_move(move3).unwrap();

    for y in 0..5 {
        for x in 0..4 {
            print!("{:?}\t", board.get(Coordinate(x, y)).unwrap());
        }
        println!();
    }
}
