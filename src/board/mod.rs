mod print;
mod checker;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    None,
    One,
    Two,
}

pub enum PlacementError {
    NotAColumn,
    ColumnFull,
}

impl PlacementError {
    pub fn to_string(&self) -> &'static str {
        match self {
            PlacementError::NotAColumn => "not a column",
            PlacementError::ColumnFull => "column full",
        }
    }
}

pub struct Vector {
    column: i8,
    row: i8,
}

pub struct Board {
    cells: [[Player; Board::SIZE as usize]; Board::SIZE as usize],
}

impl Board {
    const SIZE: usize = 8;

    pub fn new() -> Board {
        Board {
            cells: [[Player::None; Board::SIZE]; Board::SIZE],
        }
    }

    pub fn place(&mut self, player: Player, column: i8) -> Result<Vector, PlacementError> {
        if column as usize >= Board::SIZE {
            return Err(PlacementError::NotAColumn);
        }

        for i in (0..Board::SIZE).rev() {
            if self.cells[i][column as usize] == Player::None {
                self.cells[i][column as usize] = player;
                return Ok(Vector {
                    column,
                    row: i as i8,
                });
            }
        }

        Err(PlacementError::ColumnFull)
    }

    pub fn check_victory(&self, last_place: Vector) -> bool {
        let player = self.cells[last_place.row as usize][last_place.column as usize];

        checker::check_victory(&self, checker::Direction::Horizontal, &last_place, player)
            || checker::check_victory(&self, checker::Direction::Vertical, &last_place, player)
            || checker::check_victory(&self, checker::Direction::UpDown, &last_place, player)
            || checker::check_victory(&self, checker::Direction::DownUp, &last_place, player)
    }
}