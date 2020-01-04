struct Game {
    board: Board,
}

impl Game {
    fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }

    fn consecutive_count(&self, player: Player, position: &Position) -> u8 {
        std::cmp::max(
            self.consecutive_count_per_direction(player, position, &Direction::S),
            std::cmp::max(
                self.consecutive_count_per_direction(player, position, &Direction::E),
                std::cmp::max(
                    self.consecutive_count_per_direction(player, position, &Direction::NE),
                    self.consecutive_count_per_direction(player, position, &Direction::SE),
                ),
            ),
        )
    }

    fn consecutive_count_per_direction(
        &self,
        player: Player,
        position: &Position,
        direction: &Direction,
    ) -> u8 {
        if self.board.is_player(player, &position) {
            let reverse = direction.reverse();
            1 + self.compound_consecutive_count(player, position + &direction, &direction)
                + self.compound_consecutive_count(player, position + &reverse, &reverse)
        } else {
            0
        }
    }

    fn compound_consecutive_count(
        &self,
        player: Player,
        position: Position,
        direction: &Direction,
    ) -> u8 {
        if self.board.is_player(player, &position) {
            self.compound_consecutive_count(player, &position + direction, direction) + 1
        } else {
            0
        }
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

    fn cell(&self, position: &Position) -> Cell {
        if position.x < 0
            || position.y < 0
            || position.x >= Self::SIZE as i8
            || position.y >= Self::SIZE as i8
        {
            Cell::OutOfBounds
        } else {
            self.cells[position.x as usize][position.y as usize]
        }
    }

    fn is_player(&self, player: Player, position: &Position) -> bool {
        self.cell(&position) == Cell::Player(player)
    }

    fn get_fall_position(&self, x: i8) -> Position {
        self.get_fall_position_height(Position { x, y: -1 })
    }

    fn get_fall_position_height(&self, position: Position) -> Position {
        let floor = &position + &Direction::S;
        if Cell::Empty == self.cell(&floor) {
            self.get_fall_position_height(floor)
        } else {
            position
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    OutOfBounds,
    Player(Player),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Player {
    White,
    Black,
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

    const N: Self = Self { x: 0, y: -1 };
    const NE: Self = Self { x: 1, y: -1 };
    const E: Self = Self { x: 1, y: 0 };
    const SE: Self = Self { x: 1, y: 1 };
    const S: Self = Self { x: 0, y: 1 };
    const SW: Self = Self { x: -1, y: 1 };
    const W: Self = Self { x: -1, y: 0 };
    const NW: Self = Self { x: -1, y: -1 };
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
            let mut game = Game::new();

            game.board.cells[2][6] = Cell::Player(Player::Black);
            game.board.cells[2][5] = Cell::Player(Player::Black);
            game.board.cells[2][4] = Cell::Player(Player::Black);
            game.board.cells[2][3] = Cell::Player(Player::White);

            game.board.cells[0][0] = Cell::Player(Player::White);
            game.board.cells[0][1] = Cell::Player(Player::White);

            game.board.cells[6][6] = Cell::Player(Player::Black);
            game.board.cells[5][5] = Cell::Player(Player::Black);
            game.board.cells[4][4] = Cell::Player(Player::Black);
            game.board.cells[5][4] = Cell::Player(Player::Black);

            assert_eq!(
                game.consecutive_count(Player::Black, &Position { x: 2, y: 5 }),
                3
            );
            assert_eq!(
                game.consecutive_count(Player::White, &Position { x: 0, y: 1 }),
                2
            );
            assert_eq!(
                game.consecutive_count(Player::Black, &Position { x: 5, y: 5 }),
                3
            );
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
            board.cells[2][3] = Cell::Player(Player::Black);

            assert!(!board.is_player(Player::Black, &Position { x: 8, y: 8 }));
            assert!(!board.is_player(Player::Black, &Position { x: 0, y: 0 }));
            assert!(!board.is_player(Player::White, &Position { x: 2, y: 3 }));
            assert!(board.is_player(Player::Black, &Position { x: 2, y: 3 }));
        }

        #[test]
        fn fall_position() {
            let mut board = Board::new();
            board.cells[2][3] = Cell::Player(Player::Black);
            board.cells[4][6] = Cell::Player(Player::White);

            assert_eq!(board.get_fall_position(0), Position { x: 0, y: 6 });
            assert_eq!(board.get_fall_position(2), Position { x: 2, y: 2 });
            assert_eq!(board.get_fall_position(4), Position { x: 4, y: 5 });
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
