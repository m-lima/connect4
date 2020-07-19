use crate::board::Board;
use crate::board::Token;
use crate::game;
use crate::game::State;

pub struct Ai {
    token: Token,
    depth: u8,
    verbose: bool,
}

struct Play<T> {
    col: u8,
    value: T,
}

enum Result {
    Threaded(std::thread::JoinHandle<Play<i64>>),
    Static(Play<i64>),
}

impl Result {
    fn resolve(self) -> Option<Play<i64>> {
        match self {
            Self::Threaded(r) => r.join().ok(),
            Self::Static(r) => Some(r),
        }
    }
}

// TODO: Add tests
impl Ai {
    #[must_use]
    pub fn new(token: Token, depth: u8, verbose: bool) -> Self {
        Self {
            token,
            depth,
            verbose,
        }
    }

    #[must_use]
    pub fn play(&self, board: &Board) -> u8 {
        shuffle_columns(board.size())
            .into_iter()
            .filter_map(|col| self.to_threads(board.clone(), col))
            .filter_map(Result::resolve)
            .inspect(|play| {
                if self.verbose {
                    println!("Score for {}: {}", play.col + 1, play.value);
                }
            })
            .fold(
                Play {
                    col: rand::random::<u8>() % board.size(),
                    value: i64::min_value(),
                },
                max_score,
            )
            .col
    }

    fn to_threads(&self, mut board: Board, col: u8) -> Option<Result> {
        match game::place(&mut board, self.token, col) {
            Ok(State::Victory) => Some(Result::Static(Play {
                col,
                value: 7_i64.pow(u32::from(self.depth)),
            })),
            Ok(State::Ongoing) if self.depth > 0 => {
                let depth = self.depth;
                let token = self.token;
                Some(Result::Threaded(std::thread::spawn(move || Play {
                    col,
                    value: score_for_column(board.clone(), col, depth - 1, !token, -1),
                })))
            }
            _ => None,
        }
    }
}

fn shuffle_columns(size: u8) -> Vec<u8> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let mut columns = (0..size).collect::<Vec<_>>();
    columns.shuffle(&mut rng);
    columns
}

fn score_for_column(mut board: Board, col: u8, depth: u8, token: Token, factor: i64) -> i64 {
    match game::place(&mut board, token, col) {
        Ok(State::Victory) => 7_i64.pow(u32::from(depth)) * factor,
        Ok(State::Ongoing) if depth > 0 => (0..board.size())
            .map(|col| score_for_column(board.clone(), col, depth - 1, !token, -factor))
            .sum::<i64>(),
        _ => 0,
    }
}

fn max_score(left: Play<i64>, right: Play<i64>) -> Play<i64> {
    if left.value > right.value {
        left
    } else {
        right
    }
}
