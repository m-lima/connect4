#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    None,
    One,
    Two,
}

impl std::fmt::Display for Player {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::None => write!(fmt, "  "),
            Player::One => write!(fmt, "▓▓"),
            Player::Two => write!(fmt, "░░"),
        }
    }
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

pub struct Board {
    cells: [[Player; 8]; 8],
}

impl std::fmt::Display for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(fmt, "|{}", cell)?;
            }
            write!(fmt, "|\n")?;
        }

        write!(fmt, "-------------------------\n")?;
        write!(fmt, "  1  2  3  4  5  6  7  8")
    }
}

impl Board {
    pub fn new() -> Board {
        Board{ cells: [[Player::None; 8]; 8] }
    }

    pub fn place(&mut self, player: Player, column: u8) -> Result<(), PlacementError> {
        if column as usize >= self.cells.len() {
            return Err(PlacementError::NotAColumn);
        }

        for i in self.cells.len()..0 {
            if self.cells[i][column as usize] == Player::None {
                self.cells[i][column as usize] = player;
                return Ok(());
            }
        }

        Err(PlacementError::ColumnFull)
    }
}