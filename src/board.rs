#[derive(Debug, Copy, Clone)]
enum Player {
    NONE,
    ONE,
    TWO,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::NONE => write!(f, "  "),
            Player::ONE => write!(f, "▓▓"),
            Player::TWO => write!(f, "░░"),
        }
    }
}

pub struct Board {
    cells: [[Player; 8]; 8],
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(f, "|{}", cell)?;
            }
            write!(f, "|\n")?;
        }

        write!(f, "-------------------------")
    }
}

impl Board {
    pub fn new() -> Board {
        Board{ cells: [[Player::NONE; 8]; 8] }
    }
}