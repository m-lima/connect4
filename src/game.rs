#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBounds,
    ColumnFull,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::OutOfBounds => write!(fmt, "out of bounds"),
            Self::ColumnFull => write!(fmt, "column full"),
        }
    }
}

pub enum Status {
    Won,
    Tie,
    Ongoing,
}

pub fn new() -> Game {
    Game {
        board: [[Cell::Empty; Game::SIZE as usize]; Game::SIZE as usize],
        last_score: 0,
    }
}

pub struct Game {
    board: [[Cell; Self::SIZE as usize]; Self::SIZE as usize],
    last_score: u8,
}

impl Game {
    pub const SIZE: u8 = 7;

    #[allow(clippy::cast_sign_loss)]
    pub fn place(&self, token: Token, x: u8) -> Result<Self, Error> {
        let position = self.fall_position(x)?;
        Ok({
            let mut board = self.board;
            board[position.y as usize][position.x as usize] = Cell::Token(token);

            Self {
                board,
                last_score: self.score(token, &position),
            }
        })
    }

    pub fn plan(&self, token: Token, x: u8) -> Result<u8, Error> {
        let position = self.fall_position(x)?;
        Ok(self.score(token, &position))
    }

    pub fn status(&self) -> Status {
        if self.last_score >= 1 << 3 {
            Status::Won
        } else if !self.board[0].iter().any(|c| *c == Cell::Empty) {
            Status::Tie
        } else {
            Status::Ongoing
        }
    }

    pub fn last_score(&self) -> u8 {
        self.last_score
    }

    fn score(&self, token: Token, position: &Position) -> u8 {
        *[
            self.direction_score(token, position, &Direction::S),
            self.direction_score(token, position, &Direction::E),
            self.direction_score(token, position, &Direction::NE),
            self.direction_score(token, position, &Direction::SE),
        ]
        .iter()
        .max()
        .unwrap_or(&0)
    }

    fn direction_score(&self, token: Token, position: &Position, direction: &Direction) -> u8 {
        let reverse = &direction.reverse();
        1 << (self.compound_direction_score(token, position + direction, direction)
            + self.compound_direction_score(token, position + reverse, reverse))
    }

    #[allow(clippy::needless_pass_by_value)]
    fn compound_direction_score(
        &self,
        token: Token,
        position: Position,
        direction: &Direction,
    ) -> u8 {
        if self.is_token(token, &position) {
            self.compound_direction_score(token, &position + direction, direction) + 1
        } else {
            0
        }
    }

    #[allow(clippy::cast_sign_loss)]
    fn cell(&self, position: &Position) -> Cell {
        if position.x < 0
            || position.y < 0
            || position.x as u8 >= Self::SIZE
            || position.y as u8 >= Self::SIZE
        {
            Cell::OutOfBounds
        } else {
            self.board[position.y as usize][position.x as usize]
        }
    }

    fn is_token(&self, token: Token, position: &Position) -> bool {
        self.cell(&position) == Cell::Token(token)
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn fall_position(&self, x: u8) -> Result<Position, Error> {
        if x >= Self::SIZE {
            return Err(Error::OutOfBounds);
        }

        let position = self.fall_position_height(Position { x: x as i8, y: -1 });

        if position.y < 0 {
            Err(Error::ColumnFull)
        } else {
            Ok(position)
        }
    }

    fn fall_position_height(&self, position: Position) -> Position {
        let floor = &position + &Direction::S;
        if Cell::Empty == self.cell(&floor) {
            self.fall_position_height(floor)
        } else {
            position
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for cell in row {
                write!(fmt, "|{}", cell)?;
            }
            writeln!(fmt, "|")?;
        }

        for _ in 0..Self::SIZE {
            write!(fmt, "---")?;
        }
        writeln!(fmt, "-")?;

        for i in 0..Self::SIZE {
            write!(fmt, " {:2}", i + 1)?;
        }
        writeln!(fmt)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    OutOfBounds,
    Token(Token),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(fmt, "  "),
            Self::OutOfBounds => write!(fmt, ""),
            Self::Token(p) => write!(fmt, "{}", p),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    White,
    Black,
}

impl Token {
    pub fn flip(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(fmt, "\u{2593}\u{2593}"),
            Self::Black => write!(fmt, "\u{2591}\u{2591}"),
            // "█"
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Position {
    x: i8,
    y: i8,
}

impl std::ops::Add<&Direction> for &Position {
    type Output = Position;

    #[must_use]
    fn add(self, direction: &Direction) -> Position {
        Position {
            x: self.x + direction.x,
            y: self.y + direction.y,
        }
    }
}

impl Direction {
    fn reverse(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    const NE: Self = Self { x: 1, y: -1 };
    const E: Self = Self { x: 1, y: 0 };
    const SE: Self = Self { x: 1, y: 1 };
    const S: Self = Self { x: 0, y: 1 };
}

#[derive(Debug, Eq, PartialEq)]
struct Direction {
    x: i8,
    y: i8,
}

#[cfg(test)]
mod tests {
    mod game {
        use super::super::*;

        #[test]
        fn place() {
            let mut game = new();
            game = game.place(Token::White, 3).unwrap();
            assert_eq!(game.last_score, 1);
            game = game.place(Token::Black, 3).unwrap();
            assert_eq!(game.last_score, 1);
            game = game.place(Token::White, 3).unwrap();
            assert_eq!(game.last_score, 1);
            game = game.place(Token::Black, 0).unwrap();
            assert_eq!(game.last_score, 1);
            game = game.place(Token::White, 1).unwrap();
            assert_eq!(game.last_score, 1);
            game = game.place(Token::Black, 2).unwrap();
            assert_eq!(game.last_score, 2);
            game = game.place(Token::White, 2).unwrap();
            assert_eq!(game.last_score, 4);
        }

        #[test]
        fn play_errors() {
            let mut game = new();

            game = game.place(Token::White, 3).unwrap();
            game = game.place(Token::White, 3).unwrap();
            game = game.place(Token::White, 3).unwrap();
            game = game.place(Token::White, 3).unwrap();
            game = game.place(Token::White, 3).unwrap();
            game = game.place(Token::White, 3).unwrap();
            game = game.place(Token::White, 3).unwrap();

            assert_eq!(
                game.place(Token::Black, 3).err().unwrap(),
                Error::ColumnFull
            );
            assert_eq!(
                game.place(Token::Black, 9).err().unwrap(),
                Error::OutOfBounds
            );
        }

        #[test]
        fn score() {
            let mut game = new();

            game.board[6][2] = Cell::Token(Token::Black);
            game.board[5][2] = Cell::Token(Token::Black);
            game.board[4][2] = Cell::Token(Token::Black);
            game.board[3][2] = Cell::Token(Token::White);

            game.board[0][0] = Cell::Token(Token::White);
            game.board[1][0] = Cell::Token(Token::White);

            game.board[6][6] = Cell::Token(Token::Black);
            game.board[5][5] = Cell::Token(Token::Black);
            game.board[4][4] = Cell::Token(Token::Black);
            game.board[4][5] = Cell::Token(Token::Black);

            assert_eq!(game.score(Token::Black, &Position { x: 2, y: 5 }), 4);
            assert_eq!(game.score(Token::White, &Position { x: 0, y: 1 }), 2);
            assert_eq!(game.score(Token::Black, &Position { x: 5, y: 5 }), 4);
        }

        #[test]
        fn out_of_bounds() {
            let game = new();
            assert_eq!(game.cell(&Position { x: 8, y: 1 }), Cell::OutOfBounds);
            assert_eq!(game.cell(&Position { x: 1, y: 8 }), Cell::OutOfBounds);
            assert_eq!(game.cell(&Position { x: 8, y: 8 }), Cell::OutOfBounds);
            assert_ne!(game.cell(&Position { x: 1, y: 1 }), Cell::OutOfBounds);
        }

        #[test]
        fn is_token() {
            let mut game = new();
            game.board[3][2] = Cell::Token(Token::Black);

            assert!(!game.is_token(Token::Black, &Position { x: 8, y: 8 }));
            assert!(!game.is_token(Token::Black, &Position { x: 0, y: 0 }));
            assert!(!game.is_token(Token::White, &Position { x: 2, y: 3 }));
            assert!(game.is_token(Token::Black, &Position { x: 2, y: 3 }));
        }

        #[test]
        fn fall_position() {
            let mut game = new();
            game.board[3][2] = Cell::Token(Token::Black);
            game.board[6][4] = Cell::Token(Token::White);

            assert_eq!(game.fall_position(0).unwrap(), Position { x: 0, y: 6 });
            assert_eq!(game.fall_position(2).unwrap(), Position { x: 2, y: 2 });
            assert_eq!(game.fall_position(4).unwrap(), Position { x: 4, y: 5 });
        }
    }

    mod position {
        use super::super::*;

        #[test]
        fn translation() {
            let position = &Position { x: 2, y: 3 };
            let direction = &Direction { x: -9, y: 1 };
            assert_eq!(position + direction, Position { x: -7, y: 4 });
        }

        #[test]
        fn reversal() {
            let direction = Direction { x: -1, y: 1 };
            assert_eq!(direction.reverse(), Direction::NE);
        }
    }
}
