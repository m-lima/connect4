#[derive(Debug, PartialEq)]
pub enum Result {
    Ok(u8),
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

pub trait Player {
    fn play(&self, game: &super::game::Game) -> Result;
    fn token(&self) -> super::game::Token;
}

pub struct Human {
    token: super::game::Token,
}

pub fn new_human(token: super::game::Token) -> Human {
    Human { token }
}

impl Player for Human {
    fn play(&self, _: &super::game::Game) -> Result {
        {
            use std::io::Write;
            print!("Select the column for {}: ", self.token);
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
                Ok(i) => Result::Ok(i - 1),
                Err(e) => Result::Error(e.to_string()),
            },
        }
    }

    fn token(&self) -> super::game::Token {
        self.token
    }
}

pub struct Ai {
    token: super::game::Token,
}

pub fn new_ai(token: super::game::Token) -> Ai {
    Ai { token }
}

impl Ai {
    const DEPTH: u8 = 7;

    fn shuffle_columns() -> Vec<u8> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let mut columns = (0..super::game::Game::SIZE).collect::<Vec<u8>>();
        columns.shuffle(&mut rng);
        columns
    }

    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::filter_map
    )]
    fn best_move(&self, game: &super::game::Game) -> u8 {
        let columns = Self::shuffle_columns();
        columns
            .into_iter()
            .map(|x| (x, game.place(self.token, x)))
            .filter(|r| r.1.is_ok())
            .map(|r| {
                let g = r.1.unwrap();
//                let score = i32::from(g.last_score());
//                (r.0, score + Self::dig(&g, Self::DEPTH, self.token.flip(), -1))
                if g.last_score() == 1 {
                    ( r.0, 7_i64.pow(Self::DEPTH as u32))
                } else if Self::DEPTH > 0 {
                    ( r.0, Self::dig(&g, Self::DEPTH - 1, self.token.flip(), -1) )
                } else {
                    ( r.0, 0_i64)
                }
            })
//            .map(|r| {
//                println!("Score for {}: {}", r.0 + 1, r.1);
//                r
//            })
            .fold(
                (0, i64::min_value()),
                |acc, s| {
                    if s.1 > acc.1 {
                        s
                    } else {
                        acc
                    }
                },
            )
            .0
    }

    #[allow(clippy::filter_map)]
    fn dig(game: &super::game::Game, depth: u8, token: super::game::Token, factor: i64) -> i64 {
        let score = if depth > 0 {
            (0..super::game::Game::SIZE)
                .map(|x| game.place(token, x))
                .filter_map(std::result::Result::ok)
                .map(|g| {
//                    let score = factor * i32::from(g.last_score());
//                    score + Self::dig(&g, depth - 1, token.flip(), -factor)
                    if g.last_score() == 1 {
                        factor * 7_i64.pow(depth as u32)
                    } else {
                        Self::dig(&g, depth - 1, token.flip(), -factor)
                    }
                })
                .sum::<i64>()
        } else {
            (0..super::game::Game::SIZE)
                .map(|x| game.plan(token, x))
                .filter_map(std::result::Result::ok)
                .map(i64::from)
                .sum::<i64>()
                * factor * depth as i64
        };
//        println!("{: >3$}{}: {}", "Score for depth", depth, score, depth as usize);
        score
    }
}

impl Player for Ai {
    fn play(&self, game: &super::game::Game) -> Result {
        Result::Ok(self.best_move(&game))
    }

    fn token(&self) -> super::game::Token {
        self.token
    }
}
