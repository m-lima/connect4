pub struct Ai {
    token: super::Token,
    depth: u8,
    verbose: bool,
}

struct AiPlay<T> {
    col: u8,
    board: super::Board,
    value: T,
}

enum AiResult {
    Threaded(std::thread::JoinHandle<AiPlay<i64>>),
    Static(AiPlay<i64>),
}

impl AiResult {
    fn resolve(self) -> Option<AiPlay<i64>> {
        match self {
            Self::Threaded(r) => r.join().ok(),
            Self::Static(r) => Some(r),
        }
    }
}

// struct GamePlan<'a> {
//     board: &'a super::Board,
//     plays: Vec<Play>,
// }

// TODO: Add tests
impl Ai {
    pub fn new(token: super::Token, depth: u8, verbose: bool) -> Self {
        Self {
            token,
            depth,
            verbose,
        }
    }

    pub fn play(&self, board: &super::Board) -> u8 {
        let columns = Self::shuffle_columns(board.size());
        columns
            .into_iter()
            .filter_map(|col| self.calculate_score(board, col))
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(AiResult::resolve)
            .inspect(|play| {
                if self.verbose {
                    println!("Score for {}: {}", play.col + 1, play.value);
                }
            })
            .fold(
                AiPlay {
                    col: rand::random::<u8>() % board.size(),
                    board: super::Board::new(0),
                    value: i64::min_value(),
                },
                Self::max_score,
            )
            .col
    }

    fn shuffle_columns(size: u8) -> Vec<u8> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let mut columns = (0..size).collect::<Vec<_>>();
        columns.shuffle(&mut rng);
        columns
    }

    fn calculate_score(&self, board: &super::Board, col: u8) -> Option<AiResult> {
        let mut board = board.clone();
        match super::Game::place(&mut board, self.token, col) {
            Ok(game) => {
                if super::State::Victory == game.state() {
                    Some(AiResult::Static(AiPlay {
                        col,
                        board,
                        value: 7_i64.pow(u32::from(self.depth)),
                    }))
                } else if self.depth > 0 {
                    Some(AiResult::Threaded(std::thread::spawn(move || AiPlay {
                        col,
                        board,
                        value: Self::dig(self.depth - 1, !self.token, -1),
                    })))
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn dig(depth: u8, token: super::Token, factor: i64) -> i64 {
        if depth > 0 {
            #[allow(clippy::filter_map)]
            (0..game.size())
                .map(|col| game.place(token, col))
                .filter_map(std::result::Result::ok)
                .map(|game| {
                    if super::State::Victory == game.state() {
                        factor * 7_i64.pow(u32::from(depth))
                    } else {
                        Self::dig(depth - 1, !token, -factor)
                    }
                })
                .sum::<i64>()
        } else {
            (0..game.size())
                .map(|col| game.plan(token, col))
                .filter_map(std::result::Result::ok)
                .filter(|state| super::State::Victory == *state)
                .count() as i64
                * factor
                * i64::from(depth)
        }
    }

    fn max_score(left: AiPlay<i64>, right: AiPlay<i64>) -> AiPlay<i64> {
        if left.value > right.value {
            left
        } else {
            right
        }
    }
}
