use std::fmt;

pub struct BoardState {
    state: Vec<Vec<char>>,
    pub winner: Option<char>,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            state: vec![vec![' '; 3]; 3],
            winner: None,
        }
    }

    pub fn make_move(&self, position: (usize, usize)) -> Self {
        let mut new_state = self.state.clone();
        new_state[position.0 - 1][position.1 - 1] = 'x';

        Self {
            winner: BoardState::check(&new_state),
            state: new_state,
        }
    }

    fn check(state: &Vec<Vec<char>>) -> Option<char> {
        // TODO
        if state[0][0] == 'x' && state[0][1] == 'x' && state[0][2] == 'x' {
            return Some('X');
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
