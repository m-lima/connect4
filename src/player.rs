#[derive(Debug, PartialEq)]
pub enum Result {
    Ok(u8, usize),
    Repeat,
    Quit,
    Error(String),
}

impl std::fmt::Display for Result {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if let Self::Error(message) = self {
            write!(fmt, "{}", message)
        } else {
            Ok(())
        }
    }
}

pub enum Player {
    Ai(Ai),
    Human,
    Tcp,
}

impl Player {
    pub fn play<Game: super::game::Game + 'static>(
        &self,
        game: &Game,
        token: super::game::Token,
    ) -> Result {
        match self {
            Self::Ai(ai) => Ai::play(ai, game, token),
            Self::Human => Human::play(token),
            Self::Tcp => unimplemented!(),
        }
    }
}

struct Human;

impl Human {
    fn play(token: super::game::Token) -> Result {
        {
            use std::io::Write;
            print!("Select the column for {}: ", token);
            let _ = std::io::stdout().flush();
        }

        let mut buffer = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut buffer) {
            return Result::Error(e.to_string());
        }

        buffer = buffer.trim().to_string();
        if buffer.is_empty() {
            return Result::Repeat;
        }

        match buffer.as_str() {
            "Q" | "q" => Result::Quit,
            _ => match buffer.parse::<u8>() {
                Ok(i) => Result::Ok(i - 1, 1),
                Err(e) => Result::Error(e.to_string()),
            },
        }
    }
}

pub struct Ai {
    depth: u8,
    verbose: bool,
}

struct AiPlay<T> {
    col: u8,
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

// TODO: Add tests
impl Ai {
    pub fn new(depth: u8, verbose: bool) -> Self {
        Self { depth, verbose }
    }

    fn play<Game: super::game::Game + 'static>(
        &self,
        game: &Game,
        token: super::game::Token,
    ) -> Result {
        Result::Ok(self.best_move(game, token), 0)
    }

    fn shuffle_columns(size: u8) -> Vec<u8> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let mut columns = (0..size).collect::<Vec<_>>();
        columns.shuffle(&mut rng);
        columns
    }

    fn calculate_score<Game: super::game::Game + 'static>(
        play: AiPlay<std::result::Result<Game, super::game::Error>>,
        token: super::game::Token,
        depth: u8,
    ) -> Option<AiResult> {
        match play.value {
            Ok(game) => {
                if super::game::Status::Victory == game.status() {
                    Some(AiResult::Static(AiPlay {
                        col: play.col,
                        value: 7_i64.pow(u32::from(depth)),
                    }))
                } else if depth > 0 {
                    let col = play.col;
                    Some(AiResult::Threaded(std::thread::spawn(move || AiPlay {
                        col,
                        value: Self::dig(&game, depth - 1, !token, -1),
                    })))
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn best_move<Game: super::game::Game + 'static>(
        &self,
        game: &Game,
        token: super::game::Token,
    ) -> u8 {
        let columns = Self::shuffle_columns(game.size());
        columns
            .into_iter()
            .map(|col| AiPlay {
                col,
                value: game.place(token, col),
            })
            .filter_map(|play| Self::calculate_score(play, token, self.depth))
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
                    col: rand::random::<u8>() % game.size(),
                    value: i64::min_value(),
                },
                Self::max_score,
            )
            .col
    }

    fn dig<Game: super::game::Game>(
        game: &Game,
        depth: u8,
        token: super::game::Token,
        factor: i64,
    ) -> i64 {
        if depth > 0 {
            #[allow(clippy::filter_map)]
            (0..game.size())
                .map(|col| game.place(token, col))
                .filter_map(std::result::Result::ok)
                .map(|game| {
                    if super::game::Status::Victory == game.status() {
                        factor * 7_i64.pow(u32::from(depth))
                    } else {
                        Self::dig(&game, depth - 1, !token, -factor)
                    }
                })
                .sum::<i64>()
        } else {
            (0..game.size())
                .map(|col| game.plan(token, col))
                .filter_map(std::result::Result::ok)
                .filter(|status| super::game::Status::Victory == *status)
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
