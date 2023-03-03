use chessehc::{board::Board, logic::Coordinate, piece::Pieces};

fn main() {
    println!("Hello, world!");
}

fn print_board(board: &Board, pieces: &Pieces) {
    for y in 0..5 {
        for x in 0..4 {
            print!(
                "{}\t",
                match board.get(pieces, Coordinate(x, y)).unwrap() {
                    Some((player, piece)) => {
                        let mut a = format!("{piece:?}")[0..1].to_owned();
                        a.push_str(&player.to_string());
                        a
                    }
                    None => ".".to_owned(),
                }
            );
        }
        println!();
    }
    println!();
}
