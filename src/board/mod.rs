mod checker;
mod print;

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

pub struct Vector<T> {
    pub column: T,
    pub row: T,
}

impl Vector<usize> {
    fn decrement_row(&self) -> Vector<usize> {
        Vector {
            row: self.row - 1,
            column: self.column,
        }
    }
}

pub struct Board {
    cells: [[Player; Board::SIZE]; Board::SIZE],
}

impl Board {
    pub const SIZE: usize = 8;

    pub fn new() -> Board {
        Board {
            cells: [[Player::None; Board::SIZE]; Board::SIZE],
        }
    }

    fn _place(&mut self, player: Player, position: Vector<usize>) -> Option<Vector<usize>> {
        if self.cells[position.row][position.column] == Player::None {
            self.cells[position.row][position.column] = player;
            Some(position)
        } else if position.row != 0 {
            self._place(player, position.decrement_row())
        } else {
            None
        }
    }

    pub fn place(
        &mut self,
        player: Player,
        column: usize,
    ) -> Result<Vector<usize>, PlacementError> {
        if column >= Board::SIZE {
            return Err(PlacementError::NotAColumn);
        }

        self._place(
            player,
            Vector {
                row: Board::SIZE - 1,
                column,
            },
        )
        .ok_or_else(|| PlacementError::ColumnFull)
    }

    pub fn check_victory(&self, last_place: Vector<usize>) -> bool {
        let player = self.cells[last_place.row][last_place.column];

        checker::check_victory(&self, checker::Direction::Horizontal, &last_place, player)
            || checker::check_victory(&self, checker::Direction::Vertical, &last_place, player)
            || checker::check_victory(&self, checker::Direction::UpDown, &last_place, player)
            || checker::check_victory(&self, checker::Direction::DownUp, &last_place, player)
    }

    pub fn full_column(&self, column: usize) -> bool {
        self.cells[0][column] != Player::None
    }
}
