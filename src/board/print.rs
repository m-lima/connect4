use board::Player;
use board::Board;

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

        write!(fmt, "-------------------------\n")?;
        write!(fmt, "  1  2  3  4  5  6  7  8")
    }
}
