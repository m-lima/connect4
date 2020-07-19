use crate::cartesian::Direction;
use crate::cartesian::Position;

pub struct Board {
    cells: Vec<Cell>,
    size: u8,
}

#[must_use]
pub fn new(size: u8) -> Board {
    Board {
        cells: vec![Cell::Empty; usize::from(size * size)],
        size,
    }
}

impl Board {
    pub const fn size(&self) -> u8 {
        self.size
    }

    pub const fn iter(&self, position: Position, direction: Direction) -> BoardIterator<'_> {
        BoardIterator {
            position,
            direction,
            board: &self,
        }
    }

    pub(super) fn count(&self, cell: Cell, position: Position, direction: Direction) -> u8 {
        let mut counter = 0;
        for c in self.iter(position, direction) {
            if c == cell {
                counter += 1;
            } else {
                break;
            }
        }
        counter
    }

    pub(super) fn set_cell(&mut self, position: Position, cell: Cell) -> Result<(), crate::Error> {
        if out_of_bounds(i16::from(self.size), position) {
            Err(crate::Error::OutOfBounds)
        } else {
            Ok(self.cells[index(self.size, position)] = cell)
        }
    }

    fn cell(&self, position: Position) -> Cell {
        if out_of_bounds(i16::from(self.size), position) {
            Cell::OutOfBounds
        } else {
            self.cells[index(self.size, position)]
        }
    }
}

impl std::clone::Clone for Board {
    fn clone(&self) -> Self {
        Self {
            cells: self.cells.clone(),
            size: self.size,
        }
    }
}

fn out_of_bounds(size: i16, position: Position) -> bool {
    position.x() < 0 || position.y() < 0 || position.x() >= size || position.y() >= size
}

// Allowed because it is a private function and it is checked before being called
#[allow(clippy::cast_sign_loss)]
const fn index(size: u8, position: Position) -> usize {
    (position.x() * size as i16 + position.y()) as usize
}

pub struct BoardIterator<'a> {
    position: Position,
    direction: Direction,
    board: &'a Board,
}

impl std::iter::Iterator for BoardIterator<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let cell = self.board.cell(self.position);
        if Cell::OutOfBounds == cell {
            None
        } else {
            self.position += self.direction;
            Some(cell)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Empty,
    OutOfBounds,
    Token(Token),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    White,
    Black,
}

impl std::ops::Not for Token {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cartesian::Position;

    #[test]
    fn out_of_bounds() {
        let board = Board::new(7);
        assert_eq!(board.cell(Position::new(8, 1)), super::Cell::OutOfBounds);
        assert_eq!(board.cell(Position::new(1, 8)), super::Cell::OutOfBounds);
        assert_eq!(board.cell(Position::new(8, 8)), super::Cell::OutOfBounds);
        assert_ne!(board.cell(Position::new(1, 1)), super::Cell::OutOfBounds);
    }
}
