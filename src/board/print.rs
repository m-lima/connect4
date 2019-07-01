use board::Board;
use board::Player;

impl std::fmt::Display for Player {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::None => write!(fmt, "  "),
            Player::One => write!(fmt, "▓▓"),
            Player::Two => write!(fmt, "░░"),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(fmt, "|{}", cell)?;
            }
            write!(fmt, "|\n")?;
        }

        for i in 0..Board::SIZE {
            write!(fmt, "---")?;
        }
        write!(fmt, "-\n")?;

        for i in 0..Board::SIZE {
            write!(fmt, " {:2}", i + 1)?;
        }
        write!(fmt, "\n")
    }
}
