use std::fmt;

pub struct BoardState {
    state: Vec<Vec<char>>,
    last_player: char,
    pub winner: Option<char>,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            state: vec![vec![' '; 3]; 3],
            last_player: ' ',
            winner: None,
        }
    }

    pub fn make_move(&self, position: (usize, usize)) -> Self {
        let player = match self.last_player {
            'o' => 'x',
            'x' => 'o',
            _ => 'x',
        };
        let mut new_state = self.state.clone();
        new_state[position.0 - 1][position.1 - 1] = player;

        Self {
            winner: BoardState::check(&new_state, player),
            last_player: player,
            state: new_state,
        }
    }

    pub fn get_possible_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        for (i, row) in self.state.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == ' ' {
                    moves.push((i + 1, j + 1));
                }
            }
        }

        moves
    }

    fn check(state: &Vec<Vec<char>>, player: char) -> Option<char> {
        let by_row = state
            .iter()
            .any(|row| row.iter().all(|&cell| cell == player));
        if by_row {
            return Some(player);
        }

        let mut by_col = true;
        for col in 0..state.len() {
            for row in 0..state.len() {
                if state[row][col] != player {
                    by_col = false;
                    break;
                }
            }
            if by_col {
                return Some(player);
            }
        }

        let mut by_diagonal = true;
        for i in 0..state.len() {
            if state[i][i] != player {
                by_diagonal = false;
                break;
            }
        }
        if by_diagonal {
            return Some(player);
        }

        by_diagonal = true;
        for i in 0..state.len() {
            let row_index = state.len() - 1 - i;
            if state[row_index][i] != player {
                by_diagonal = false;
                break;
            }
        }
        if by_diagonal {
            return Some(player);
        }

        // Check if draw
        if state.iter().flatten().all(|&cell| cell != ' ') {
            return Some(' ');
        }

        None
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self
            .state
            .iter()
            .map(|row| {
                row.iter()
                    .fold(String::new(), |acc, &c| format!("{} {}", acc, c))
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", state)
    }
}
