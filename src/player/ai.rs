pub struct Ai<T> {
    token: T,
}

impl<T: Copy + PartialEq + Default> super::Player<T> for Ai<T> {
    fn next_move(&self, available: &Vec<usize>, _board: &Vec<Vec<T>>) -> usize {
//        self.defend(&available, &board)
//            .or_else(|| self.attack(&available, &board))
//            .unwrap_or_else(|| Ai::<T>::random(&available))
        Ai::<T>::random(&available)
    }

    fn token(&self) -> T {
        self.token
    }
}

impl<T> super::PlayerBuilder<T> for Ai<T> {
    fn new(token: T) -> Ai<T> {
        Ai { token }
    }
}

//struct Pattern {
//    sequence: [bool; 4],
//    action: usize,
//    reversable: bool,
//}
//
//struct Direction {
//    horizontal: bool,
//    vertical: bool,
//}
//
//struct Location {
//    row: usize,
//    column: usize,
//}
//
//impl Direction {
//    const Horizontal: Direction = Direction {
//        horizontal: true,
//        vertical: false,
//    };
//    const Vertical: Direction = Direction {
//        horizontal: false,
//        vertical: true,
//    };
//    const UpDown: Direction = Direction {
//        horizontal: true,
//        vertical: true,
//    };
//    const DownUp: Direction = Direction {
//        horizontal: true,
//        vertical: true,
//    };
//
//    fn horizontal(&self) -> usize {
//        self.horizontal as usize
//    }
//
//    fn vertical(&self) -> usize {
//        self.horizontal as usize
//    }
//}

impl<T: PartialEq + Default> Ai<T> {
//    const ThreeInARow: Pattern = Pattern {
//        sequence: [true, true, true, false],
//        action: 3,
//        reversable: true,
//    };
//    const TwoToTheSide: Pattern = Pattern {
//        sequence: [true, true, false, false],
//        action: 2,
//        reversable: true,
//    };
//    const TwoInTheMiddle: Pattern = Pattern {
//        sequence: [false, true, true, false],
//        action: 3,
//        reversable: false,
//    };
//    const Gap: Pattern = Pattern {
//        sequence: [true, true, false, true],
//        action: 2,
//        reversable: true,
//    };
//
//    // TODO: reverse pattern
//    fn map_pattern_to_location(
//        pattern: Pattern,
//        token: T,
//        defend: bool,
//        board: &Vec<Vec<T>>,
//        column: usize,
//        row: usize,
//        direction: Direction,
//    ) -> Option<usize> {
//        // Cannot fit pattern
//        if (column >= board.len() - 4 * direction.horizontal())
//            || (row >= board.len() - 4 * direction.vertical())
//        {
//            return None;
//        }
//
//        for (i, present) in pattern.sequence.iter().enumerate() {
//            let current =
//                &board[row + i * direction.vertical()][column + i * direction.horizontal()];
//            if *present {
//                // Token is not the expected one
//                if (!defend && *current != token) || (defend && *current == T::default()) {
//                    return None;
//                }
//            } else {
//                // Token is not the expected one (empty)
//                if *current != T::default() {
//                    return None;
//                }
//
//                // The token cannot be placed in this position
//                if row < board.len() - 1
//                    && board[1 + row + i * direction.vertical()]
//                        [column + i * direction.horizontal()]
//                        != T::default()
//                {
//                    return None;
//                }
//            }
//        }
//        Some(pattern.action + column)
//    }
//
//    fn find_pattern_internal(
//        pattern: Pattern,
//        token: T,
//        defend: bool,
//        board: &Vec<Vec<T>>,
//        direction: Direction,
//    ) -> Option<usize> {
//        None
//    }
//
//    fn find_pattern(
//        pattern: Pattern,
//        token: T,
//        defend: bool,
//        board: &Vec<Vec<T>>,
//    ) -> Option<usize> {
//        for i in 0..board.len() {
//            for j in 0..board.len() {
//                if i < board.len() - 4 {}
//            }
//        }
//        None
//    }
//
//    fn find_token_locations(
//        board: &Vec<Vec<T>>,
//        discriminator: impl Fn(&T) -> bool,
//    ) -> Vec<Location> {
//        let mut oponent_locations = Vec::<Location>::new();
//
//        for (i, row) in board.iter().enumerate() {
//            for (j, cell) in row.iter().enumerate() {
//                if discriminator(cell) {
//                    oponent_locations.push(Location { row: i, column: j });
//                }
//            }
//        }
//
//        oponent_locations
//    }
//
//    fn defend(&self, available: &Vec<usize>, board: &Vec<Vec<T>>) -> Option<usize> {
//        let locations = Ai::find_token_locations(board, |t| -> bool {
//            *t != self.token && *t != T::default()
//        });
//        None
//    }
//
//    fn attack(&self, available: &Vec<usize>, board: &Vec<Vec<T>>) -> Option<usize> {
//        let locations = Ai::find_token_locations(board, |t| -> bool { *t == self.token });
//        None
//    }

    fn random(available: &Vec<usize>) -> usize {
        rand::random::<usize>() % available.len()
    }
}
