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

pub fn new() -> Game {
    Game {
        board: [[Cell::Empty; Game::SIZE]; Game::SIZE],
    }
}

pub struct Game {
    board: [[Cell; Self::SIZE]; Self::SIZE],
}

impl Game {
    const SIZE: usize = 7;

    #[allow(clippy::cast_sign_loss)]
    pub fn play(&self, player: Player, x: i8) -> Result<(Self, u8), Error> {
        if x < 0 || x as usize >= Self::SIZE {
            return Err(Error::OutOfBounds);
        }

        let position = self.fall_position(x);
        if position.y < 0 {
            return Err(Error::ColumnFull);
        }

        let game = self.place(player, &position);
        let score = game.score(player, &position);
        Ok((game, score))
    }

    #[allow(clippy::cast_sign_loss)]
    fn place(&self, player: Player, position: &Position) -> Self {
        let mut board = self.board;
        board[position.y as usize][position.x as usize] = Cell::Player(player);
        Self { board }
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
        if self.is_player(player, &position) {
            let reverse = direction.reverse();
            1 + self.compound_direction_score(player, &(position + direction), &direction)
                + self.compound_direction_score(player, &(position + &reverse), &reverse)
        } else {
            0
        }
    }

    fn compound_direction_score(
        &self,
        player: Player,
        position: &Position,
        direction: &Direction,
    ) -> u8 {
        if self.is_player(player, position) {
            self.compound_direction_score(player, &(position + direction), direction) + 1
        } else {
            0
        }
    }

    #[allow(clippy::cast_sign_loss)]
    fn cell(&self, position: &Position) -> Cell {
        if position.x < 0
            || position.y < 0
            || position.x as usize >= Self::SIZE
            || position.y as usize >= Self::SIZE
        {
            Cell::OutOfBounds
        } else {
            self.board[position.y as usize][position.x as usize]
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

impl std::fmt::Display for Game {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for cell in row.iter() {
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
    Player(Player),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(fmt, "  "),
            Self::OutOfBounds => write!(fmt, ""),
            Self::Player(p) => write!(fmt, "{}", p),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
    White,
    Black,
}

impl std::fmt::Display for Player {
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
    #[allow(clippy::neg_multiply)]
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
        fn play() {
            let game = new();
            let (game, score) = game.play(Player::White, 3).unwrap();
            assert_eq!(score, 1);
            let (game, score) = game.play(Player::Black, 3).unwrap();
            assert_eq!(score, 1);
            let (game, score) = game.play(Player::White, 3).unwrap();
            assert_eq!(score, 1);
            let (game, score) = game.play(Player::Black, 0).unwrap();
            assert_eq!(score, 1);
            let (game, score) = game.play(Player::White, 1).unwrap();
            assert_eq!(score, 1);
            let (game, score) = game.play(Player::Black, 2).unwrap();
            assert_eq!(score, 2);
            let (_, score) = game.play(Player::White, 2).unwrap();
            assert_eq!(score, 3);
        }

        #[test]
        fn play_errors() {
            let game = new();

            let (game, _) = game.play(Player::White, 3).unwrap();
            let (game, _) = game.play(Player::White, 3).unwrap();
            let (game, _) = game.play(Player::White, 3).unwrap();
            let (game, _) = game.play(Player::White, 3).unwrap();
            let (game, _) = game.play(Player::White, 3).unwrap();
            let (game, _) = game.play(Player::White, 3).unwrap();
            let (game, _) = game.play(Player::White, 3).unwrap();

            assert_eq!(
                game.play(Player::Black, 3).err().unwrap(),
                Error::ColumnFull
            );
            assert_eq!(
                game.play(Player::Black, 9).err().unwrap(),
                Error::OutOfBounds
            );
        }

        #[test]
        fn count() {
            let mut game = new();

            game.board[6][2] = Cell::Player(Player::Black);
            game.board[5][2] = Cell::Player(Player::Black);
            game.board[4][2] = Cell::Player(Player::Black);
            game.board[3][2] = Cell::Player(Player::White);

            game.board[0][0] = Cell::Player(Player::White);
            game.board[1][0] = Cell::Player(Player::White);

            game.board[6][6] = Cell::Player(Player::Black);
            game.board[5][5] = Cell::Player(Player::Black);
            game.board[4][4] = Cell::Player(Player::Black);
            game.board[4][5] = Cell::Player(Player::Black);

            assert_eq!(game.score(Player::Black, &Position { x: 2, y: 5 }), 3);
            assert_eq!(game.score(Player::White, &Position { x: 0, y: 1 }), 2);
            assert_eq!(game.score(Player::Black, &Position { x: 5, y: 5 }), 3);
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
        fn is_player() {
            let mut game = new();
            game.board[3][2] = Cell::Player(Player::Black);

            assert!(!game.is_player(Player::Black, &Position { x: 8, y: 8 }));
            assert!(!game.is_player(Player::Black, &Position { x: 0, y: 0 }));
            assert!(!game.is_player(Player::White, &Position { x: 2, y: 3 }));
            assert!(game.is_player(Player::Black, &Position { x: 2, y: 3 }));
        }

        #[test]
        fn fall_position() {
            let mut game = new();
            game.board[3][2] = Cell::Player(Player::Black);
            game.board[6][4] = Cell::Player(Player::White);

            assert_eq!(game.fall_position(0), Position { x: 0, y: 6 });
            assert_eq!(game.fall_position(2), Position { x: 2, y: 2 });
            assert_eq!(game.fall_position(4), Position { x: 4, y: 5 });
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
