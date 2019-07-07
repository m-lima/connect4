#[cfg(test)]
mod tests;

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
    fn shift(&self, direction: &Vector<i8>) -> Option<Vector<usize>> {
        if (direction.row < 0 && self.row == 0) || (direction.column < 0 && direction.column == 0) {
            None
        } else {
            Some(Vector {
                column: (self.column as i8 + direction.column) as usize,
                row: (self.row as i8 + direction.row) as usize,
            })
        }
    }
}

impl Vector<i8> {
    fn invert(&self) -> Vector<i8> {
        Vector {
            column: self.column * -1,
            row: self.row * -1,
        }
    }
    fn from_direction(direction: Direction) -> Vector<i8> {
        match direction {
            Direction::Vertical => Vector { row: -1, column: 0 },
            Direction::Horizontal => Vector { row: 0, column: -1 },
            Direction::UpDown => Vector {
                row: -1,
                column: -1,
            },
            Direction::DownUp => Vector { row: -1, column: 1 },
        }
    }
}

enum Direction {
    Vertical,
    Horizontal,
    UpDown,
    DownUp,
}

pub struct Board<T> {
    size: usize,
    cells: Vec<Vec<T>>,
    available: Vec<usize>,
}

impl<T: Default + Copy + PartialEq> Board<T> {
    pub fn new(size: usize) -> Board<T> {
        Board {
            size,
            cells: vec![vec![T::default(); size]; size],
            available: (0..size).collect(),
        }
    }

    fn place_internal(&mut self, token: T, position: Vector<usize>) -> Vector<usize> {
        if self.cells[position.row][position.column] == T::default() {
            self.cells[position.row][position.column] = token;
            position
        } else {
            self.place_internal(token, position.decrement_row())
        }
    }

    pub fn place(&mut self, token: T, index: usize) -> Vector<usize> {
        let column = self.available[index];
        let size = self.size;
        let position = self.place_internal(
            token,
            Vector {
                row: size - 1,
                column,
            },
        );
        if position.row == 0 {
            self.available.remove(index);
        }
        position
    }

    fn measure_sequence(
        &self,
        direction: &Vector<i8>,
        current: Option<Vector<usize>>,
        token: T,
        counter: u8,
    ) -> u8 {
        current
            .filter(|_| counter < 4)
            .filter(|c| {
                c.column < self.size && c.row < self.size && self.cells[c.row][c.column] == token
            })
            .map_or_else(
                || counter,
                |c| self.measure_sequence(&direction, c.shift(direction), token, counter + 1),
            )
    }

    fn check_victory_internal(
        &self,
        direction: Direction,
        current: &Vector<usize>,
        token: T,
    ) -> bool {
        let vector = Vector::from_direction(direction);
        let backwards_vector = vector.invert();
        self.measure_sequence(
            &vector,
            current.shift(&vector),
            token,
            self.measure_sequence(
                &backwards_vector,
                current.shift(&backwards_vector),
                token,
                1,
            ),
        ) >= 4
    }

    pub fn check_victory(&self, last_place: Vector<usize>) -> bool {
        let token = self.cells[last_place.row][last_place.column];

        self.check_victory_internal(Direction::Horizontal, &last_place, token)
            || self.check_victory_internal(Direction::Vertical, &last_place, token)
            || self.check_victory_internal(Direction::UpDown, &last_place, token)
            || self.check_victory_internal(Direction::DownUp, &last_place, token)
    }

    pub fn available_columns(&self) -> &Vec<usize> {
        &self.available
    }

    pub fn has_available_columns(&self) -> bool {
        !self.available.is_empty()
    }

    pub fn to_vec(&self) -> &Vec<Vec<T>> {
        //        let mut matrix = Vec::with_capacity(self.size);
        //        for row in self.cells.iter() {
        //            matrix.push(row.to_vec());
        //        }
        //        matrix
        &self.cells
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Board<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(fmt, "|{}", cell)?;
            }
            write!(fmt, "|\n")?;
        }

        for _ in 0..self.size {
            write!(fmt, "---")?;
        }
        write!(fmt, "-\n")?;

        for i in 0..self.size {
            write!(fmt, " {:2}", i + 1)?;
        }
        write!(fmt, "\n")
    }
}
