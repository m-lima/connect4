#[derive(Debug)]
pub enum Error {
    OutOfBounds,
    ColumnFull,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::OutOfBounds => write!(fmt, "out of bounds"),
            Error::ColumnFull => write!(fmt, "column full"),
        }
    }
}

pub struct Game {
    board: Board,
}

pub fn new() -> Game {
    Game {
        board: Board::new(),
    }
}

impl Game {
    pub fn play(&self, player: Player, x: u8) -> Result<(Self, u8), Error> {
        if x >= Board::SIZE as u8 {
            return Err(Error::OutOfBounds);
        }

        let position = self.board.fall_position(x as i8);
        if position.y < 0 {
            return Err(Error::ColumnFull);
        }

        let game = Game {
            board: self.board.place(player, &position),
        };
        let score = game.score(player, &position);
        Ok((game, score))
    }

    fn score(&self, player: Player, position: &Position) -> u8 {
        std::cmp::max(
            self.direction_score(player, position, &Direction::S),
            std::cmp::max(
                self.direction_score(player, position, &Direction::E),
                std::cmp::max(
                    self.direction_score(player, position, &Direction::NE),
                    self.direction_score(player, position, &Direction::SE),
                ),
            ),
        )
    }

    fn direction_score(&self, player: Player, position: &Position, direction: &Direction) -> u8 {
        if self.board.is_player(player, &position) {
            let reverse = direction.reverse();
            1 + self.compound_direction_score(player, position + &direction, &direction)
                + self.compound_direction_score(player, position + &reverse, &reverse)
        } else {
            0
        }
    }

    fn compound_direction_score(
        &self,
        player: Player,
        position: Position,
        direction: &Direction,
    ) -> u8 {
        if self.board.is_player(player, &position) {
            self.compound_direction_score(player, &position + direction, direction) + 1
        } else {
            0
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.board)
    }
}

struct Board {
    cells: [[Cell; Board::SIZE]; Board::SIZE],
}

impl Board {
    const SIZE: usize = 7;

    fn new() -> Self {
        Board {
            cells: [[Cell::Empty; Board::SIZE]; Board::SIZE],
        }
    }

    fn place(&self, player: Player, position: &Position) -> Self {
        let mut cells = self.cells.clone();
        cells[position.y as usize][position.x as usize] = Cell::Player(player);
        Self { cells }
    }

    fn cell(&self, position: &Position) -> Cell {
        if position.x < 0
            || position.y < 0
            || position.x >= Self::SIZE as i8
            || position.y >= Self::SIZE as i8
        {
            Cell::OutOfBounds
        } else {
            self.cells[position.y as usize][position.x as usize]
        }
    }

    fn is_player(&self, player: Player, position: &Position) -> bool {
        self.cell(&position) == Cell::Player(player)
    }

    fn fall_position(&self, x: i8) -> Position {
        self.fall_position_height(Position { x, y: -1 })
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

impl std::fmt::Display for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(fmt, "|{}", cell)?;
            }
            write!(fmt, "|\n")?;
        }

        for _ in 0..Self::SIZE {
            write!(fmt, "---")?;
        }
        write!(fmt, "-\n")?;

        for i in 0..Self::SIZE {
            write!(fmt, " {:2}", i + 1)?;
        }
        write!(fmt, "\n")
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    OutOfBounds,
    Player(Player),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(fmt, "  "),
            Cell::OutOfBounds => write!(fmt, ""),
            Cell::Player(p) => write!(fmt, "{}", p),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
    White,
    Black,
}

impl std::fmt::Display for Player {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::White => write!(fmt, "▓▓"),
            Player::Black => write!(fmt, "░░"),
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
            x: self.x * -1,
            y: self.y * -1,
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
        fn count() {
            let mut game = new();

            game.board.cells[6][2] = Cell::Player(Player::Black);
            game.board.cells[5][2] = Cell::Player(Player::Black);
            game.board.cells[4][2] = Cell::Player(Player::Black);
            game.board.cells[3][2] = Cell::Player(Player::White);

            game.board.cells[0][0] = Cell::Player(Player::White);
            game.board.cells[1][0] = Cell::Player(Player::White);

            game.board.cells[6][6] = Cell::Player(Player::Black);
            game.board.cells[5][5] = Cell::Player(Player::Black);
            game.board.cells[4][4] = Cell::Player(Player::Black);
            game.board.cells[4][5] = Cell::Player(Player::Black);

            assert_eq!(game.score(Player::Black, &Position { x: 2, y: 5 }), 3);
            assert_eq!(game.score(Player::White, &Position { x: 0, y: 1 }), 2);
            assert_eq!(game.score(Player::Black, &Position { x: 5, y: 5 }), 3);
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
        fn is_player() {
            let mut board = Board::new();
            board.cells[3][2] = Cell::Player(Player::Black);

            assert!(!board.is_player(Player::Black, &Position { x: 8, y: 8 }));
            assert!(!board.is_player(Player::Black, &Position { x: 0, y: 0 }));
            assert!(!board.is_player(Player::White, &Position { x: 2, y: 3 }));
            assert!(board.is_player(Player::Black, &Position { x: 2, y: 3 }));
        }

        #[test]
        fn fall_position() {
            let mut board = Board::new();
            board.cells[3][2] = Cell::Player(Player::Black);
            board.cells[6][4] = Cell::Player(Player::White);

            assert_eq!(board.fall_position(0), Position { x: 0, y: 6 });
            assert_eq!(board.fall_position(2), Position { x: 2, y: 2 });
            assert_eq!(board.fall_position(4), Position { x: 4, y: 5 });
        }
    }

    mod position {
        use super::super::*;

        #[test]
        fn translation() {
            let position = Position { x: 2, y: 3 };
            let direction = Direction { x: -9, y: 1 };
            assert_eq!(&position + &direction, Position { x: -7, y: 4 });
        }

        #[test]
        fn reversal() {
            let direction = Direction { x: -1, y: 1 };
            assert_eq!(direction.reverse(), Direction::NE);
        }
    }
}
