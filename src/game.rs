pub struct Board {
    cells: [[Cell; Board::SIZE]; Board::SIZE],
}

impl Board {
    pub const SIZE: usize = 7;

    pub fn cell(&self, position: &Position) -> &Cell {
        if position.x < 0
            || position.y < 0
            || position.x >= Self::SIZE as i8
            || position.y >= Self::SIZE as i8
        {
            &Cell::OutOfBounds
        } else {
            &self.cells[position.x as usize][position.y as usize]
        }
    }

    pub fn is_player(&self, player: Player, position: &Position) -> bool {
        self.cell(&position) == &Cell::Player(player)
    }

    pub fn get_fall_position(&self, x: i8) -> Position {
        Position {
            x,
            y: self.get_fall_position_height(Position { x, y: 0 }),
        }
    }

    fn get_fall_position_height(&self, position: Position) -> i8 {
        if let Cell::Empty = self.cell(&position) {
            self.get_fall_position_height(position.translate(&Movement::from(Orientation::S)))
        } else {
            position.y
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Empty,
    OutOfBounds,
    Player(Player),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Player {
    White,
    Black,
}

pub struct Position {
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

#[derive(Copy, Clone, Eq, PartialEq)]
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

impl Movement {
    fn reverse(&self) -> Self {
        self * -1
    }
}

impl std::ops::Mul<i8> for Orientation {
    type Output = Movement;

    fn mul(self, amount: i8) -> Movement {
        &Movement::from(self) * amount
    }
}

pub struct Movement {
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

impl Position {
    fn translate(&self, movement: &Movement) -> Self {
        self + movement
    }
}

pub fn consecutive_count(player: Player, board: &Board, position: &Position) -> u8 {
    std::cmp::max(
        consecutive_count_per_orientation(player, board, position, Orientation::S),
        0,
    )
}

fn consecutive_count_per_orientation(
    player: Player,
    board: &Board,
    position: &Position,
    orientation: Orientation,
) -> u8 {
    if board.is_player(player, &position) {
        let movement = Movement::from(orientation);
        let reverse = movement.reverse();
        1 + compound_consecutive_count(player, board, position.translate(&movement), &movement)
            + compound_consecutive_count(player, board, position.translate(&reverse), &reverse)
    } else {
        0
    }
}

fn compound_consecutive_count(
    player: Player,
    board: &Board,
    position: Position,
    movement: &Movement,
) -> u8 {
    if board.is_player(player, &position) {
        compound_consecutive_count(player, board, position.translate(movement), movement)
    } else {
        0
    }
}
