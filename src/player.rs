#[derive(Debug, PartialEq)]
pub enum Result {
    Ok((u8, super::game::Token)),
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
    fn play<Game: super::game::Game + 'static>(&self, game: &Game) -> Result
    where
        Self: Sized;
    fn token(&self) -> super::game::Token;
}

pub struct Human {
    token: super::game::Token,
}

pub fn new_human(token: super::game::Token) -> Human {
    Human { token }
}

impl Player for Human {
    fn play<Game: super::game::Game + 'static>(&self, _: &Game) -> Result {
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
                Ok(i) => Result::Ok((i - 1, self.token)),
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
    depth: u8,
}

pub fn new_ai(token: super::game::Token, depth: u8) -> Ai {
    Ai { token, depth }
}

// TODO: Add tests
impl Ai {
    fn shuffle_columns() -> Vec<u8> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let mut columns = (0..super::game::Board::size()).collect::<Vec<u8>>();
        columns.shuffle(&mut rng);
        columns
    }

    // TODO: Create struct with CMP for (u8, i64)
    #[allow(clippy::filter_map)]
    fn best_move<Game: super::game::Game + 'static>(&self, game: &Game) -> u8 {
        enum Result {
            Threaded(std::thread::JoinHandle<(u8, i64)>),
            Static((u8, i64)),
        }

        let columns = Self::shuffle_columns();
        columns
            .into_iter()
            .map(|x| (x, game.place(self.token, x)))
            .filter(|result| result.1.is_ok())
            .map(|result| {
                let x = result.0;
                let game = result.1.unwrap();
                if super::game::Status::Victory == game.status() {
                    Result::Static((x, 7_i64.pow(u32::from(self.depth))))
                } else if self.depth > 0 {
                    let token = self.token;
                    let depth = self.depth;
                    Result::Threaded(std::thread::spawn(move || {
                        (x, Self::dig(&game, depth - 1, token.flip(), -1))
                    }))
                } else {
                    Result::Static((x, 0_i64))
                }
            })
            .collect::<Vec<Result>>()
            .into_iter()
            .map(|result| match result {
                Result::Threaded(r) => r.join(),
                Result::Static(r) => Ok(r),
            })
            .filter_map(std::result::Result::ok)
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
    fn dig<Game: super::game::Game>(
        game: &Game,
        depth: u8,
        token: super::game::Token,
        factor: i64,
    ) -> i64 {
        if depth > 0 {
            (0..super::game::Board::size())
                .map(|x| game.place(token, x))
                .filter_map(std::result::Result::ok)
                .map(|g| {
                    if super::game::Status::Victory == g.status() {
                        factor * 7_i64.pow(u32::from(depth))
                    } else {
                        Self::dig(&g, depth - 1, token.flip(), -factor)
                    }
                })
                .sum::<i64>()
        } else {
            (0..super::game::Board::size())
                .map(|x| game.plan(token, x))
                .filter_map(std::result::Result::ok)
                .map(|s| {
                    if let super::game::Status::Victory = s {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i64>()
                * factor
                * i64::from(depth)
        }
    }
}

impl Player for Ai {
    fn play<Game: super::game::Game + 'static>(&self, game: &Game) -> Result {
        Result::Ok((self.best_move(game), self.token))
    }

    fn token(&self) -> super::game::Token {
        self.token
    }
}
