#[warn(clippy::all)]
mod board_state;

use board_state::BoardState;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer: String;
    let mut board = BoardState::new();

    while board.winner.is_none() {
        println!("{}\nYour turn", board);

        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let player_move = parse_input(&buffer);

        board = board.make_move(player_move);

        println!("{}\nOpponent turn", board);

        let pc_move = find_move(&board);
        board = board.make_move(pc_move);
    }

    println!("{}\nWinner is: {}", board, board.winner.unwrap());

    Ok(())
}

fn find_move(board: &BoardState) -> (usize, usize) {
    todo!()
}

fn extract_digit(string: &String, pos: usize) -> u32 {
    string
        .chars()
        .nth(pos)
        .expect("Invalid input")
        .to_digit(10)
        .expect("NaN")
}

fn parse_input(input: &String) -> (usize, usize) {
    (
        extract_digit(&input, 1) as usize,
        extract_digit(&input, 3) as usize,
    )
}
