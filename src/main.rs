#[warn(clippy::all)]
mod board_state;

use board_state::BoardState;
use std::{
    f32::{INFINITY, NEG_INFINITY},
    io,
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    println!("Go first? [Y/n] ");
    io::stdin().read_line(&mut buffer)?;
    let mut first_skip = match &buffer[..] {
        "\n" => false,
        "y\n" => false,
        "Y\n" => false,
        _ => true,
    };

    let mut board = BoardState::new();
    loop {
        if !first_skip {
            println!("{}\nYour turn", board);

            buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let player_move = parse_input(&buffer);

            board = board.make_move(player_move);
            if board.winner.is_some() {
                break;
            }
        }

        println!("{}\nOpponent turn", board);

        let pc_move = determine_move(&board);
        println!("[{},{}]", pc_move.0, pc_move.1);
        board = board.make_move(pc_move);
        if board.winner.is_some() {
            break;
        }

        first_skip = false;
    }

    println!("{}\nWinner is: {}", board, board.winner.unwrap());

    Ok(())
}

fn determine_move(board: &BoardState) -> (usize, usize) {
    let moves = board.get_possible_moves();
    let mut best_move: Option<(usize, usize)> = None;
    let mut best_score = NEG_INFINITY;

    for position in moves {
        let curr_board = board.make_move(position);
        let move_score = alpha_beta(&curr_board, NEG_INFINITY, INFINITY, false);
        if move_score > best_score {
            best_score = move_score;
            best_move = Some(position);
        }
    }

    best_move.unwrap()
}

fn alpha_beta(state: &BoardState, alpha: f32, beta: f32, is_maximizing: bool) -> f32 {
    if state.winner.is_some() {
        let empty_cells = state.empty_cell_count() as f32;
        let coef: f32 = match state.winner.unwrap() {
            '-' => 0.0,
            _ => match is_maximizing {
                true => -1.0,
                false => 1.0,
            },
        };
        return coef * (1.0 + empty_cells);
    }

    let mut best_score: f32;
    let moves = state.get_possible_moves();
    if is_maximizing {
        best_score = NEG_INFINITY;
        for position in moves {
            let new_state = state.make_move(position);
            best_score = best_score.max(alpha_beta(&new_state, alpha, beta, false));
            let alpha = alpha.max(best_score);
            if alpha >= beta {
                break;
            }
        }
    } else {
        best_score = INFINITY;
        for position in moves {
            let new_state = state.make_move(position);
            best_score = best_score.min(alpha_beta(&new_state, alpha, beta, true));
            let beta = beta.min(best_score);
            if alpha >= beta {
                break;
            }
        }
    }

    best_score
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
