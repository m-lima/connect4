#![deny(warnings)]
#![deny(clippy::pedantic)]
#![warn(rust_2018_idioms)]
#![allow(clippy::missing_errors_doc)]

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
    #[must_use]
    pub fn new(size: u8) -> Self {
        Self {
            board: Board::new(size),
            state: if size < 5 { State::Tie } else { State::Ongoing },
        }
    }

    pub fn place(&mut self, token: Token, x: u8) -> Result<(), Error> {
        let position = self.fall_position(x)?;
        *self.board.cell_mut(position)? = Cell::Token(token);
        self.update_state(token, position);
        Ok(())
    }

    // pub fn plan(&self, token: Token, x: u8) -> Result<State, Error> {
    //     let position = self.fall_position(x)?;
    //     Ok(Self::build_state(token, position, &self.board))
    // }

    #[must_use]
    pub const fn state(&self) -> State {
        self.state
    }

    #[must_use]
    pub const fn size(&self) -> u8 {
        self.board.size()
    }

    fn update_state(&mut self, token: Token, position: Position) {
        if Self::tie(position, &self.board) {
            self.state = State::Tie
        } else if Self::victory(token, position, &self.board) {
            self.state = State::Victory
        } else {
            self.state = State::Ongoing
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

    fn fall_position(&self, x: u8) -> Result<Position, Error> {
        if x >= self.size() {
            return Err(Error::OutOfBounds);
        }

        let position = self.board.count(
            Cell::Empty,
            Position {
                x: i16::from(x),
                y: 0,
            },
            Direction::S,
        );

        if position == 0 {
            Err(Error::ColumnFull)
        } else {
            Ok(Position {
                x: i16::from(x),
                y: i16::from(position) - 1,
            })
        }
    }
}

struct Board {
    cells: Vec<Cell>,
    size: u8,
}

impl Board {
    fn new(size: u8) -> Self {
        Self {
            cells: vec![Cell::Empty; usize::from(size * size)],
            size,
        }
    }

    fn cell(&self, position: Position) -> Cell {
        if out_of_bounds(i16::from(self.size), position) {
            Cell::OutOfBounds
        } else {
            self.cells[index(self.size, position)]
        }
    }

    fn cell_mut(&mut self, position: Position) -> Result<&mut Cell, Error> {
        if out_of_bounds(i16::from(self.size), position) {
            Err(Error::OutOfBounds)
        } else {
            Ok(&mut self.cells[index(self.size, position)])
        }
    }

    const fn size(&self) -> u8 {
        self.size
    }

    const fn iter(&self, position: Position, direction: Direction) -> BoardIterator<'_> {
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

fn out_of_bounds(size: i16, position: Position) -> bool {
    position.x < 0 || position.y < 0 || position.x >= size || position.y >= size
}

// Allowed because it is a private function and it is checked before being called
#[allow(clippy::cast_sign_loss)]
const fn index(size: u8, position: Position) -> usize {
    (position.x * size as i16 + position.y) as usize
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
        if Cell::OutOfBounds == cell {
            None
        } else {
            self.position += self.direction;
            Some(cell)
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

    fn add(self, direction: Direction) -> Position {
        Position {
            x: self.x + i16::from(direction.x),
            y: self.y + i16::from(direction.y),
        }
    }
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, direction: Direction) {
        self.x += i16::from(direction.x);
        self.y += i16::from(direction.y);
    }
}

impl Direction {
    const fn reverse(self) -> Self {
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
            game.place(Token::Black, 1).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::White, 2).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::Black, 3).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::White, 3).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::Black, 3).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::White, 3).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::Black, 4).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::White, 1).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::Black, 2).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::White, 2).unwrap();
            assert_eq!(game.state(), State::Ongoing);
            game.place(Token::White, 0).unwrap();
            assert_eq!(game.state(), State::Victory);
        }

        #[test]
        fn place_errors() {
            let mut game = Game::new(7);

            game.place(Token::White, 3).unwrap();
            game.place(Token::White, 3).unwrap();
            game.place(Token::White, 3).unwrap();
            game.place(Token::White, 3).unwrap();
            game.place(Token::White, 3).unwrap();
            game.place(Token::White, 3).unwrap();
            game.place(Token::White, 3).unwrap();

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
            let size = 7;
            let mut game = Game::new(size);

            game.board.cells[index(size, Position { x: 2, y: 6 })] = Cell::Token(Token::Black);
            game.board.cells[index(size, Position { x: 2, y: 5 })] = Cell::Token(Token::Black);
            game.board.cells[index(size, Position { x: 2, y: 4 })] = Cell::Token(Token::Black);
            game.board.cells[index(size, Position { x: 2, y: 3 })] = Cell::Token(Token::White);

            game.board.cells[index(size, Position { x: 0, y: 0 })] = Cell::Token(Token::White);
            game.board.cells[index(size, Position { x: 0, y: 1 })] = Cell::Token(Token::White);

            game.board.cells[index(size, Position { x: 6, y: 6 })] = Cell::Token(Token::Black);
            game.board.cells[index(size, Position { x: 5, y: 5 })] = Cell::Token(Token::Black);
            game.board.cells[index(size, Position { x: 4, y: 4 })] = Cell::Token(Token::Black);
            game.board.cells[index(size, Position { x: 5, y: 4 })] = Cell::Token(Token::Black);

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

            game.board.cells[index(size, Position { x: 3, y: 3 })] = Cell::Token(Token::Black);
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
            let size = 7;
            let mut game = Game::new(size);
            game.board.cells[index(size, Position { x: 2, y: 3 })] = Cell::Token(Token::Black);
            game.board.cells[index(size, Position { x: 4, y: 6 })] = Cell::Token(Token::White);

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
            let position = Position { x: 2, y: 3 };
            let direction = Direction { x: -9, y: 1 };
            assert_eq!(position + direction, Position { x: -7, y: 4 });
        }

        #[test]
        fn reversal() {
            let direction = Direction { x: -1, y: 1 };
            assert_eq!(direction.reverse(), Direction::NE);
        }
    }
}
