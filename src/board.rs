use crate::cartesian::Direction;
use crate::cartesian::Position;

pub struct Board {
    cells: Vec<Token>,
    size: u8,
}

impl Board {
    #[must_use]
    pub fn new(size: u8) -> Self {
        Self {
            cells: vec![Token::Empty; usize::from(size * size)],
            size,
        }
    }

    #[must_use]
    pub const fn size(&self) -> u8 {
        self.size
    }

    #[must_use]
    pub fn cell(&self, position: Position) -> Option<Token> {
        if out_of_bounds(i16::from(self.size), position) {
            None
        } else {
            Some(self.cells[index(self.size, position)])
        }
    }

    #[must_use]
    pub const fn iter(&self, position: Position, direction: Direction) -> Iterator<'_> {
        Iterator {
            position,
            direction,
            board: &self,
        }
    }

    pub(super) fn count(&self, token: Token, position: Position, direction: Direction) -> u8 {
        let mut counter = 0;
        for t in self.iter(position, direction) {
            if t == token {
                counter += 1;
            } else {
                break;
            }
        }
        counter
    }

    pub(super) fn set_cell(
        &mut self,
        position: Position,
        token: Token,
    ) -> Result<(), crate::Error> {
        if out_of_bounds(i16::from(self.size), position) {
            Err(crate::Error::OutOfBounds)
        } else {
            self.cells[index(self.size, position)] = token;
            Ok(())
        }
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size {
            let pos = Position::new(0, i16::from(row));
            for token in self.iter(pos, Direction::E) {
                let token = match token {
                    Token::White => 'O',
                    Token::Black => 'X',
                    Token::Empty => '_',
                };
                write!(fmt, "{} ", token)?;
            }
            writeln!(fmt)?;
        }
        Ok(())
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

#[derive(Debug)]
pub struct Iterator<'a> {
    position: Position,
    direction: Direction,
    board: &'a Board,
}

impl std::iter::Iterator for Iterator<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let cell = self.board.cell(self.position);
        if cell.is_some() {
            self.position += self.direction;
        }
        cell
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    Empty,
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
            Self::Empty => Self::Empty,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Board;
    use crate::cartesian::Position;

    #[test]
    fn out_of_bounds() {
        let board = Board::new(7);
        assert_eq!(board.cell(Position::new(8, 1)), None);
        assert_eq!(board.cell(Position::new(1, 8)), None);
        assert_eq!(board.cell(Position::new(8, 8)), None);
        assert_ne!(board.cell(Position::new(1, 1)), None);
    }
}
