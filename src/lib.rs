#![deny(warnings)]
#![deny(clippy::pedantic)]
#![warn(rust_2018_idioms)]

mod ai;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Error {
    OutOfBounds,
    ColumnFull,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::OutOfBounds => write!(fmt, "out of bounds"),
            Self::ColumnFull => write!(fmt, "column full"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Victory,
    Tie,
    Ongoing,
}

pub struct Game {
    board: Board,
    state: State,
}

impl std::default::Default for Game {
    fn default() -> Self {
        Self::new(7)
    }
}

impl Game {
    pub fn new(size: u8) -> Self {
        Self {
            board: Board::new(size),
            state: if size < 5 { State::Tie } else { State::Ongoing },
        }
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn place(&self, token: Token, x: u8) -> Result<Self, Error> {
        let position = self.fall_position(x)?;
        Ok({
            let mut cells = self.board.cells.clone();
            cells[self.board.to_linear_index(position)] = Cell::Token(token);
            let board = Board {
                cells,
                size: self.size(),
            };
            let status = Self::build_status(token, position, &board);

            Self {
                board,
                state: status,
            }
        })
    }

    pub fn plan(&self, token: Token, x: u8) -> Result<State, Error> {
        let position = self.fall_position(x)?;
        Ok(Self::build_status(token, position, &self.board))
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub const fn size(&self) -> u8 {
        self.board.size()
    }

    fn build_status(token: Token, position: Position, board: &Board) -> State {
        if Self::tie(position, &board) {
            State::Tie
        } else if Self::victory(token, position, &board) {
            State::Victory
        } else {
            State::Ongoing
        }
    }

    fn tie(position: Position, board: &Board) -> bool {
        position.y == 0
            && board
                .iter(Position { x: 0, y: 0 }, Direction::E)
                .any(|c| c == Cell::Empty)
    }

    fn victory(token: Token, position: Position, board: &Board) -> bool {
        Self::direction_victory(token, position, &board, Direction::S)
            || Self::direction_victory(token, position, &board, Direction::E)
            || Self::direction_victory(token, position, &board, Direction::NE)
            || Self::direction_victory(token, position, &board, Direction::SE)
    }

    fn direction_victory(
        token: Token,
        position: Position,
        board: &Board,
        direction: Direction,
    ) -> bool {
        let reverse = direction.reverse();
        board.count(Cell::Token(token), position, direction)
            + board.count(Cell::Token(token), position, reverse)
            > 4
    }

    #[allow(clippy::cast_possible_wrap)]
    fn fall_position(&self, x: u8) -> Result<Position, Error> {
        if x >= self.size() {
            return Err(Error::OutOfBounds);
        }

        let position = self
            .board
            .count(Cell::Empty, Position { x: x as i16, y: 0 }, Direction::S);

        if position == 0 {
            Err(Error::ColumnFull)
        } else {
            Ok(Position {
                x: x as i16,
                y: position as i16,
            })
        }
    }
}

struct Board {
    cells: Vec<Cell>,
    size: u8,
}

impl std::default::Default for Board {
    fn default() -> Self {
        Self::new(7)
    }
}

impl Board {
    fn new(size: u8) -> Self {
        Self {
            cells: vec![Cell::Empty; usize::from(size * size)],
            size,
        }
    }

    #[allow(clippy::cast_sign_loss)]
    fn cell(&self, position: Position) -> Cell {
        if position.x < 0
            || position.y < 0
            || position.x as u8 >= self.size
            || position.y as u8 >= self.size
        {
            Cell::OutOfBounds
        } else {
            self.cells[self.to_linear_index(position)]
        }
    }

    const fn usize(&self) -> usize {
        self.size as usize
    }

    const fn size(&self) -> u8 {
        self.size
    }

    const fn to_linear_index(&self, position: Position) -> usize {
        (position.x * self.size() as i16 + position.y) as usize
    }

    fn iter(&self, position: Position, direction: Direction) -> BoardIterator<'_> {
        BoardIterator {
            position,
            direction,
            board: &self,
        }
    }

    fn count(&self, cell: Cell, position: Position, direction: Direction) -> u8 {
        let mut counter = 0;
        for c in self.iter(position, direction) {
            if c == cell {
                counter += 1;
            } else {
                break;
            }
        }
        counter
    }
}

struct BoardIterator<'a> {
    position: Position,
    direction: Direction,
    board: &'a Board,
}

impl std::iter::Iterator for BoardIterator<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let cell = self.board.cell(self.position);
        if Cell::OutOfBounds != cell {
            self.position += self.direction;
            Some(cell)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    OutOfBounds,
    Token(Token),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    White,
    Black,
}

impl std::ops::Not for Token {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    x: i16,
    y: i16,
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;

    #[must_use]
    fn add(self, direction: Direction) -> Position {
        Position {
            x: self.x + direction.x as i16,
            y: self.y + direction.y as i16,
        }
    }
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, direction: Direction) {
        self.x += direction.x as i16;
        self.y += direction.y as i16;
    }
}

impl Direction {
    #[must_use]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
            let mut game = Game::new(7);
            game = game.place(Token::Black, 1).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::White, 2).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::Black, 3).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::White, 3).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::Black, 3).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::White, 3).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::Black, 4).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::White, 1).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::Black, 2).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::White, 2).unwrap();
            assert_eq!(game.status(), State::Ongoing);
            game = game.place(Token::White, 0).unwrap();
            assert_eq!(game.status(), State::Victory);
        }

        #[test]
        fn place_errors() {
            let mut game = Game::new(7);

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
            let mut game = Game::new(7);

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
                Game::victory(Token::Black, Position { x: 2, y: 5 }, &game.board),
                false
            );
            assert_eq!(
                Game::victory(Token::White, Position { x: 0, y: 1 }, &game.board),
                false
            );
            assert_eq!(
                Game::victory(Token::Black, Position { x: 5, y: 5 }, &game.board),
                false
            );

            game.board.cells[3][3] = Cell::Token(Token::Black);
            assert_eq!(
                Game::victory(Token::White, Position { x: 5, y: 5 }, &game.board),
                false
            );
            assert_eq!(
                Game::victory(Token::Black, Position { x: 5, y: 5 }, &game.board),
                true
            );
        }

        #[test]
        fn fall_position() {
            let mut game = Game::new(7);
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
            let board = Board::new(7);
            assert_eq!(board.cell(Position { x: 8, y: 1 }), Cell::OutOfBounds);
            assert_eq!(board.cell(Position { x: 1, y: 8 }), Cell::OutOfBounds);
            assert_eq!(board.cell(Position { x: 8, y: 8 }), Cell::OutOfBounds);
            assert_ne!(board.cell(Position { x: 1, y: 1 }), Cell::OutOfBounds);
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
