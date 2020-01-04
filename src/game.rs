struct Game {
    board: Board,
}

impl Game {
    fn consecutive_count(&self, player: Player, position: &Position) -> u8 {
        std::cmp::max(
            self.consecutive_count_per_orientation(player, position, Orientation::S),
            0,
        )
    }

    fn consecutive_count_per_orientation(
        &self,
        player: Player,
        position: &Position,
        orientation: Orientation,
    ) -> u8 {
        if self.board.is_player(player, &position) {
            let movement = Movement::from(orientation);
            let reverse = movement.reverse();
            1 + self.compound_consecutive_count(player, position + &movement, &movement)
                + self.compound_consecutive_count(player, position + &reverse, &reverse)
        } else {
            0
        }
    }

    fn compound_consecutive_count(
        &self,
        player: Player,
        position: Position,
        movement: &Movement,
    ) -> u8 {
        if self.board.is_player(player, &position) {
            self.compound_consecutive_count(player, &position + movement, movement)
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
        Position {
            x,
            y: self.get_fall_position_height(Position { x, y: 0 }),
        }
    }

    fn get_fall_position_height(&self, position: Position) -> i8 {
        if let Cell::Empty = self.cell(&position) {
            self.get_fall_position_height(&position + &Movement::from(Orientation::S))
        } else {
            position.y - 1
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

impl std::ops::Add<&Movement> for &Position {
    type Output = Position;

    fn add(self, movement: &Movement) -> Position {
        Position {
            x: self.x + movement.x,
            y: self.y + movement.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Orientation {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Movement {
    fn reverse(&self) -> Self {
        self * -1
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

impl std::ops::Mul<i8> for Orientation {
    type Output = Movement;

    fn mul(self, amount: i8) -> Movement {
        &Movement::from(self) * amount
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Movement {
    x: i8,
    y: i8,
}

impl std::ops::Mul<i8> for &Movement {
    type Output = Movement;

    fn mul(self, amount: i8) -> Movement {
        Movement {
            x: self.x * amount,
            y: self.y * amount,
        }
    }
}

impl std::convert::From<Orientation> for Movement {
    fn from(orientation: Orientation) -> Self {
        match orientation {
            Orientation::N => Self { x: 0, y: -1 },
            Orientation::NE => Self { x: 1, y: -1 },
            Orientation::E => Self { x: 1, y: 0 },
            Orientation::SE => Self { x: 1, y: 1 },
            Orientation::S => Self { x: 0, y: 1 },
            Orientation::SW => Self { x: -1, y: 1 },
            Orientation::W => Self { x: -1, y: 0 },
            Orientation::NW => Self { x: -1, y: -1 },
        }
    }
}

#[cfg(test)]
mod tests {
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
            let movement = Movement { x: -9, y: 1 };
            assert_eq!(&position + &movement, Position { x: -7, y: 4 });
        }

        #[test]
        fn reversal() {
            let movement = Movement { x: -9, y: 9 };
            assert_eq!(movement.reverse(), &Movement::from(Orientation::NE) * 9);
        }
    }
}
