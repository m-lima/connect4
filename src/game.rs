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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Victory,
    Tie,
    Ongoing,
}

trait Playable {
    type Output;
    fn place(&self, token: Token, x: u8) -> Result<Self::Output, Error>;
    fn plan(&self, token: Token, x: u8) -> Result<Self::Output, Error>;
    fn status(&self) -> Status;
    fn size(&self) -> u8;
}

trait Placeable {}

pub fn new() -> Game {
    Game {
        board: Board::new(),
        status: Status::Ongoing,
    }
}

pub struct Game {
    board: Board,
    status: Status,
}

//impl Playable for Game {
//
//}

// TODO: Make trait for tests
impl Game {
    #[allow(clippy::cast_sign_loss)]
    pub fn place(&self, token: Token, x: u8) -> Result<Self, Error> {
        let position = self.fall_position(x)?;
        Ok({
            let mut cells = self.board.cells;
            cells[position.y as usize][position.x as usize] = Cell::Token(token);
            let board = Board { cells };
            let status = Self::build_status(token, &position, &board);

            Self { board, status }
        })
    }

    pub fn plan(&self, token: Token, x: u8) -> Result<Status, Error> {
        let position = self.fall_position(x)?;
        Ok(Self::build_status(token, &position, &self.board))
    }

    pub fn status(&self) -> Status {
        self.status
    }

    fn build_status(token: Token, position: &Position, board: &Board) -> Status {
        if Self::tie(&position, &board) {
            Status::Tie
        } else if Self::victory(token, &position, &board) {
            Status::Victory
        } else {
            Status::Ongoing
        }
    }

    fn tie(position: &Position, board: &Board) -> bool {
        position.y == 0 && !board.cells[0].iter().any(|c| *c == Cell::Empty)
    }

    fn victory(token: Token, position: &Position, board: &Board) -> bool {
        Self::direction_score(token, position, &board, &Direction::S)
            || Self::direction_score(token, position, &board, &Direction::E)
            || Self::direction_score(token, position, &board, &Direction::NE)
            || Self::direction_score(token, position, &board, &Direction::SE)
    }

    fn direction_score(
        token: Token,
        position: &Position,
        board: &Board,
        direction: &Direction,
    ) -> bool {
        let reverse = &direction.reverse();
        (Self::compound_direction_score(token, position + direction, &board, direction)
            + Self::compound_direction_score(token, position + reverse, &board, reverse))
            >= 3
    }

    #[allow(clippy::needless_pass_by_value)]
    fn compound_direction_score(
        token: Token,
        position: Position,
        board: &Board,
        direction: &Direction,
    ) -> u8 {
        if board.is_token(token, &position) {
            Self::compound_direction_score(token, &position + direction, &board, direction) + 1
        } else {
            0
        }
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn fall_position(&self, x: u8) -> Result<Position, Error> {
        if x >= Board::SIZE {
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
        if Cell::Empty == self.board.cell(&floor) {
            self.fall_position_height(floor)
        } else {
            position
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board.cells {
            for cell in row {
                write!(fmt, "|{}", cell)?;
            }
            writeln!(fmt, "|")?;
        }

        for _ in 0..Board::SIZE {
            write!(fmt, "---")?;
        }
        writeln!(fmt, "-")?;

        for i in 0..Board::SIZE {
            write!(fmt, " {:2}", i + 1)?;
        }
        writeln!(fmt)
    }
}

pub struct Board {
    cells: [[Cell; Self::SIZE as usize]; Self::SIZE as usize],
}

impl Board {
    // TODO: Make dynamic
    pub const SIZE: u8 = 7;

    fn new() -> Self {
        Self {
            cells: [[Cell::Empty; Self::SIZE as usize]; Self::SIZE as usize],
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
            self.cells[position.y as usize][position.x as usize]
        }
    }

    fn is_token(&self, token: Token, position: &Position) -> bool {
        self.cell(&position) == Cell::Token(token)
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
            game = game.place(Token::Black, 1).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::White, 2).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::Black, 3).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::White, 3).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::Black, 3).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::White, 3).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::Black, 4).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::White, 1).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::Black, 2).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::White, 2).unwrap();
            assert_eq!(game.status(), Status::Ongoing);
            game = game.place(Token::White, 0).unwrap();
            assert_eq!(game.status(), Status::Victory);
        }

        #[test]
        fn place_errors() {
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
        fn victory() {
            let mut game = new();

            game.board.cells[6][2] = Cell::Token(Token::Black);
            game.board.cells[5][2] = Cell::Token(Token::Black);
            game.board.cells[4][2] = Cell::Token(Token::Black);
            game.board.cells[3][2] = Cell::Token(Token::White);

            game.board.cells[0][0] = Cell::Token(Token::White);
            game.board.cells[1][0] = Cell::Token(Token::White);

            game.board.cells[6][6] = Cell::Token(Token::Black);
            game.board.cells[5][5] = Cell::Token(Token::Black);
            game.board.cells[4][4] = Cell::Token(Token::Black);
            game.board.cells[4][5] = Cell::Token(Token::Black);

            assert_eq!(
                Game::victory(Token::Black, &Position { x: 2, y: 5 }, &game.board),
                false
            );
            assert_eq!(
                Game::victory(Token::White, &Position { x: 0, y: 1 }, &game.board),
                false
            );
            assert_eq!(
                Game::victory(Token::Black, &Position { x: 5, y: 5 }, &game.board),
                false
            );

            game.board.cells[3][3] = Cell::Token(Token::Black);
            assert_eq!(
                Game::victory(Token::White, &Position { x: 5, y: 5 }, &game.board),
                false
            );
            assert_eq!(
                Game::victory(Token::Black, &Position { x: 5, y: 5 }, &game.board),
                true
            );
        }

        #[test]
        fn fall_position() {
            let mut game = new();
            game.board.cells[3][2] = Cell::Token(Token::Black);
            game.board.cells[6][4] = Cell::Token(Token::White);

            assert_eq!(game.fall_position(0).unwrap(), Position { x: 0, y: 6 });
            assert_eq!(game.fall_position(2).unwrap(), Position { x: 2, y: 2 });
            assert_eq!(game.fall_position(4).unwrap(), Position { x: 4, y: 5 });
        }
    }

    mod board {
        use super::super::*;

        #[test]
        fn out_of_bounds() {
            let board = Board::new();
            assert_eq!(board.cell(&Position { x: 8, y: 1 }), Cell::OutOfBounds);
            assert_eq!(board.cell(&Position { x: 1, y: 8 }), Cell::OutOfBounds);
            assert_eq!(board.cell(&Position { x: 8, y: 8 }), Cell::OutOfBounds);
            assert_ne!(board.cell(&Position { x: 1, y: 1 }), Cell::OutOfBounds);
        }

        #[test]
        fn is_token() {
            let mut board = Board::new();
            board.cells[3][2] = Cell::Token(Token::Black);

            assert!(!board.is_token(Token::Black, &Position { x: 8, y: 8 }));
            assert!(!board.is_token(Token::Black, &Position { x: 0, y: 0 }));
            assert!(!board.is_token(Token::White, &Position { x: 2, y: 3 }));
            assert!(board.is_token(Token::Black, &Position { x: 2, y: 3 }));
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
