pub struct Board where {
    cells: [[Cell; Board::SIZE]; Board::SIZE],
}

impl Board {
    pub const SIZE: usize = 7;
}

pub enum Cell {
    Empty,
    OutOfBounds,
    Player(Player),
}

pub enum Player {
    White,
    Black,
}

pub struct Position {
    x: i8,
    y: i8,
}

impl std::ops::Add<Direction> for &Position {
    type Output = Position;

    fn add(self, direction: Direction) -> Position {
        Position{ x: self.x + direction.x, y: self.y + direction.y }
    }
}

pub enum Orientation {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl std::ops::Mul<i8> for &Orientation {
    type Output = Direction;

    fn mul(self, amount: i8) -> Direction {
        &Direction::from(self) * amount
    }
}

pub struct Direction {
    x: i8,
    y: i8,
}

impl std::ops::Mul<i8> for &Direction {
    type Output = Direction;

    fn mul(self, amount: i8) -> Direction {
        Direction{ x: self.x * amount, y: self.y * amount}
    }
}

impl std::convert::From<&Orientation> for Direction {
    fn from(orientation: &Orientation) -> Self {
        match orientation {
            Orientation::N => Self{ x: 0, y: -1},
            Orientation::NE => Self{ x: 1, y: -1},
            Orientation::E => Self{ x: 1, y: 0},
            Orientation::SE => Self{ x: 1, y: 1},
            Orientation::S => Self{ x: 0, y: 1},
            Orientation::SW => Self{ x: -1, y: 1},
            Orientation::W => Self{ x: -1, y: 0},
            Orientation::NW => Self{ x: -1, y: -1},
        }
    }
}

impl Position {
    pub fn get_cell<'a>(&self, board: &'a Board) -> &'a Cell {
        if self.x < 0 || self.y < 0 || self.x >= Board::SIZE as i8 || self.y >= Board::SIZE as i8 {
            &Cell::OutOfBounds
        } else {
            &board.cells[self.x as usize][self.y as usize]
        }
    }

    pub fn next(&self, orientation: &Orientation) -> Self {
        self.translate(orientation, 1)
    }

    pub fn translate(&self, orientation: &Orientation, amount: i8) -> Self {
        let movement = orientation * amount;
        self + movement
    }
}