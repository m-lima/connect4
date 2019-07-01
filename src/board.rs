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

enum Direction {
    Vertical,
    Horizontal,
    UpDown,
    DownUp,
}

pub struct Board {
    //pub struct Board<const SIZE: usize> {
    cells: [[Player; Board::SIZE as usize]; Board::SIZE as usize],
}

pub struct Vector {
    column: i8,
    row: i8,
}

impl Vector {
    fn shift(&self, direction: &Vector) -> Vector {
        Vector {
            column: self.column + direction.column,
            row: self.row + direction.row,
        }
    }

    fn invert(&self) -> Vector {
        Vector {
            column: self.column * -1,
            row: self.row * -1,
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

    fn _is_same_player(&self, current: &Vector, player: Player) -> bool {
        current.column >= 0
            && current.column < Board::SIZE as i8
            && current.row >= 0
            && current.row < Board::SIZE as i8
            && self.cells[current.row as usize][current.column as usize] == player
    }

    fn _measure_sequence(
        &self,
        direction: &Vector,
        current: Vector,
        player: Player,
        counter: u8,
    ) -> u8 {
        if counter == 4 {
            4
        } else {
            if self._is_same_player(&current, player) {
                self._measure_sequence(&direction, current.shift(direction), player, counter + 1)
            } else {
                counter
            }
        }
    }

    fn _check_victory(&self, direction: Direction, current: &Vector, player: Player) -> bool {
        let vector = match direction {
            Direction::Vertical => Vector { row: -1, column: 0 },
            Direction::Horizontal => Vector { row: 0, column: -1 },
            Direction::UpDown => Vector {
                row: -1,
                column: -1,
            },
            Direction::DownUp => Vector { row: -1, column: 1 },
        };
        let backwards_vector = vector.invert();
        self._measure_sequence(
            &vector,
            current.shift(&vector),
            player,
            self._measure_sequence(
                &backwards_vector,
                current.shift(&backwards_vector),
                player,
                1,
            ),
        ) >= 4
    }

    pub fn check_victory(&self, last_place: Vector) -> bool {
        let player = self.cells[last_place.row as usize][last_place.column as usize];

        self._check_victory(Direction::Horizontal, &last_place, player)
            || self._check_victory(Direction::Vertical, &last_place, player)
            || self._check_victory(Direction::UpDown, &last_place, player)
            || self._check_victory(Direction::DownUp, &last_place, player)
    }
}
