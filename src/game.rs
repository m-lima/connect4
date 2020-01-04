pub struct Board where {
    cells: [[Cell; Board::SIZE]; Board::SIZE],
}

impl Board {
    pub const SIZE: usize = 7;

    pub fn cell(&self, position: &Position) -> &Cell {
        if position.x < 0 || position.y < 0 || position.x >= Self::SIZE as i8 || position.y >= Self::SIZE as i8 {
            &Cell::OutOfBounds
        } else {
            &self.cells[position.x as usize][position.y as usize]
        }
    }

    pub fn get_fall_position(&self, x: i8) -> Position {
        Position{ x, y: self.get_fall_position_height(Position{ x, y: 0 }) }
    }

    fn get_fall_position_height(&self, position: Position) -> i8 {
        if let Cell::Empty = self.cell(&position) {
            self.get_fall_position_height(position.next(Orientation::S))
        } else {
            position.y
        }
    }
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

impl std::ops::Add<&Direction> for &Position {
    type Output = Position;

    fn add(self, direction: &Direction) -> Position {
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

impl std::ops::Mul<i8> for Orientation {
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

impl std::convert::From<Orientation> for Direction {
    fn from(orientation: Orientation) -> Self {
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
    pub fn next(&self, orientation: Orientation) -> Self {
        self.translate(orientation, 1)
    }

    pub fn translate(&self, orientation: Orientation, amount: i8) -> Self {
        let movement = orientation * amount;
        self + &movement
    }
}

