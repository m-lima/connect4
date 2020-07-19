#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    x: i16,
    y: i16,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Direction {
    x: i8,
    y: i8,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub const fn x(self) -> i16 {
        self.x
    }

    pub const fn y(self) -> i16 {
        self.y
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;

    fn add(self, direction: Direction) -> Position {
        Position {
            x: self.x + i16::from(direction.x),
            y: self.y + i16::from(direction.y),
        }
    }
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, direction: Direction) {
        self.x += i16::from(direction.x);
        self.y += i16::from(direction.y);
    }
}

impl Direction {
    pub const fn reverse(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    pub const NE: Self = Self { x: 1, y: -1 };
    pub const E: Self = Self { x: 1, y: 0 };
    pub const SE: Self = Self { x: 1, y: 1 };
    pub const S: Self = Self { x: 0, y: 1 };
}

#[cfg(test)]
mod test {
    #[test]
    fn translation() {
        let position = super::Position { x: 2, y: 3 };
        let direction = super::Direction { x: -9, y: 1 };
        assert_eq!(position + direction, super::Position { x: -7, y: 4 });
    }

    #[test]
    fn reversal() {
        let direction = super::Direction { x: -1, y: 1 };
        assert_eq!(direction.reverse(), super::Direction::NE);
    }
}
