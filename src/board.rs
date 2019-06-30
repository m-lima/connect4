#[derive(Debug, Copy, Clone)]
pub enum Player {
    NONE,
    ONE,
    TWO,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::NONE => write!(f, " "),
            Player::ONE => write!(f, "X"),
            Player::TWO => write!(f, "O"),
        }
    }
}

struct Row {
    cells: [Player; 8],
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in self.cells.iter() {
            write!(f, "|{}", v)?;
        }
        write!(f, "|")
    }
}

impl Row {
    fn empty() -> Row{Row{ cells: [Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE, Player::NONE] }}
}

pub struct Board {
    rows: [Row; 8],
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for  v in self.rows.iter() {
            write!(f, "{}\n", v)?;
        }

        write!(f, "-----------------")
    }
}

impl Board {
    pub fn new() -> Board{Board{ rows: [Row::empty(), Row::empty(), Row::empty(), Row::empty(), Row::empty(), Row::empty(), Row::empty(), Row::empty()] }}

    pub fn cell(&self, column: u8, row: u8) -> Player {
        self.rows[row as usize].cells[column as usize]
    }
}