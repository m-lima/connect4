use crate::board::Board;
use crate::board::Token;
use crate::game;
use crate::game::State;

#[derive(Debug)]
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
    // Allowed because tests are not multi-threaded
    #[allow(dead_code)]
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
                value: 2_i64.pow(u32::from(self.depth)),
            })),
            #[cfg(test)]
            Ok(State::Ongoing) if self.depth > 0 => Some(Result::Static(Play {
                col,
                value: (0..board.size())
                    .map(|col| {
                        score_for_column(board.clone(), col, self.depth - 1, !self.token, -1)
                    })
                    .sum(),
            })),
            #[cfg(not(test))]
            Ok(State::Ongoing) if self.depth > 0 => {
                let depth = self.depth;
                let token = self.token;
                Some(Result::Threaded(std::thread::spawn(move || Play {
                    col,
                    value: (0..board.size())
                        .map(|col| score_for_column(board.clone(), col, depth - 1, !token, -1))
                        .sum(),
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
        Ok(State::Victory) => 2_i64.pow(u32::from(depth)) * factor,
        Ok(State::Ongoing) if depth > 0 => (0..board.size())
            .map(|col| score_for_column(board.clone(), col, depth - 1, !token, -factor))
            .sum(),
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

#[cfg(test)]
mod test {
    macro_rules! place {
        ($board:ident; b; $($col:literal),*) => {
            $($crate::game::place(&mut $board, $crate::board::Token::Black, $col).unwrap();)*
        };
        ($board:ident; w; $($col:literal),*) => {
            $($crate::game::place(&mut $board, $crate::board::Token::White, $col).unwrap();)*
        };
    }

    use super::Ai;
    use crate::board::Board;
    use crate::board::Token;

    #[test]
    fn obvious_attack() {
        let mut board = Board::new(7);
        place!(board; b; 4, 5, 6);
        place!(board; w; 0, 0, 0);
        println!("{:#?}", board);

        let ai = Ai::new(Token::Black, 0, true);
        assert_eq!(ai.play(&board), 3);
    }

    #[test]
    fn obvious_defence() {
        let mut board = Board::new(7);
        place!(board; b; 4, 5, 5);
        place!(board; w; 0, 0, 0);
        println!("{:#?}", board);

        let ai = Ai::new(Token::Black, 1, true);
        assert_eq!(ai.play(&board), 0);
    }

    #[test]
    fn planned_attack() {
        let mut board = Board::new(7);
        place!(board; b; 0, 1);
        place!(board; w; 5, 5);
        println!("{:#?}", board);

        let ai = Ai::new(Token::Black, 5, true);
        let play = ai.play(&board);
        assert!(play == 3 || play == 4);
    }

    #[test]
    fn planned_defence() {
        let mut board = Board::new(7);
        place!(board; b; 6, 6);
        place!(board; w; 2, 3);
        println!("{:#?}", board);

        let ai = Ai::new(Token::Black, 5, true);
        let play = ai.play(&board);
        assert!(play == 1 || play == 4);
    }

    #[test]
    fn conflicting_attack() {
        let mut board = Board::new(7);
        place!(board; w; 2, 3, 3);
        place!(board; b; 3, 4, 5, 5);
        place!(board; w; 5, 5);
        println!("{:#?}", board);

        let ai = Ai::new(Token::Black, 1, true);
        assert_ne!(ai.play(&board), 4);
    }

    #[test]
    fn go_for_trap() {
        let mut board = Board::new(7);
        place!(board; b; 2, 3, 6, 6);
        place!(board; w; 2, 3, 2, 3);
        println!("{:#?}", board);

        let ai = Ai::new(Token::Black, 4, true);
        let play = ai.play(&board);
        assert!(play == 1 || play == 4);
    }
}
