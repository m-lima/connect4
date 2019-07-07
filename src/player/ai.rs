pub struct Ai<T> {
    token: T,
}

impl<T: Copy + PartialEq + Default> super::Player<T> for Ai<T> {
    fn next_move(&self, available: &Vec<usize>, board: &Vec<Vec<T>>) -> usize {
        self.defend(&board)
            .or_else(|| self.attack(&board))
            .map(|c| available.binary_search(&c).unwrap())
            .unwrap_or_else(|| Ai::<T>::random(&available))
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

struct Location {
    row: usize,
    column: usize,
}

impl<T: PartialEq + Default> Ai<T> {
    fn match_vertically(
        location: &Location,
        board: &Vec<Vec<T>>,
        matches: &impl Fn(&T) -> bool,
    ) -> Option<usize> {
        if location.row < board.len() - 2
            && location.row > 0
            && board[location.row - 1][location.column] == T::default()
            && matches(&board[location.row + 1][location.column])
            && matches(&board[location.row + 2][location.column])
        {
            println!("Ai: matched three in a row vertically");
            Some(location.column)
        } else {
            None
        }
    }

    fn match_horizontally(
        location: &Location,
        board: &Vec<Vec<T>>,
        matches: &impl Fn(&T) -> bool,
    ) -> Option<usize> {
        let size = board.len();
        if location.column < size - 1 && matches(&board[location.row][location.column + 1]) {
            if location.column < size - 3
                && board[location.row][location.column + 2] == T::default()
                && (location.row == size - 1
                    || board[location.row + 1][location.column + 2] == T::default())
            {
                println!("Ai: matched two in a row horizontally with gap to the right");
                return Some(location.column + 2);
            }
            if location.column > 0
                && board[location.row][location.column - 1] == T::default()
                && (location.row == size - 1
                    || board[location.row - 1][location.column - 1] == T::default())
            {
                println!("Ai: matched two in a row horizontally with gap to the left");
                return Some(location.column - 1);
            }
        }
        None
    }

    fn match_up_down(
        location: &Location,
        board: &Vec<Vec<T>>,
        matches: &impl Fn(&T) -> bool,
    ) -> Option<usize> {
        let size = board.len();
        if location.column < size - 2
            && location.row < size - 2
            && matches(&board[location.row + 1][location.column + 1])
            && matches(&board[location.row + 2][location.column + 2])
        {
            if location.column < size - 3
                && location.row < size - 3
                && board[location.row + 3][location.column + 3] == T::default()
                && (location.row == size - 4
                    || board[location.row + 4][location.column + 3] != T::default())
            {
                println!("Ai: matched three in a row up-down with gap to the right");
                return Some(location.column + 3);
            }
            if location.column > 0
                && location.row > 0
                && board[location.row - 1][location.column - 1] == T::default()
                && board[location.row][location.column - 1] != T::default()
            {
                println!("Ai: matched three in a row up-down with gap to the left");
                return Some(location.column - 1);
            }
        }
        None
    }

    fn match_down_up(
        location: &Location,
        board: &Vec<Vec<T>>,
        matches: &impl Fn(&T) -> bool,
    ) -> Option<usize> {
        let size = board.len();
        if location.column < size - 2
            && location.row > 1
            && matches(&board[location.row - 1][location.column + 1])
            && matches(&board[location.row - 2][location.column + 2])
        {
            if location.column < size - 3
                && location.row > 2
                && board[location.row - 3][location.column + 3] == T::default()
                && board[location.row - 2][location.column + 3] != T::default()
            {
                println!("Ai: matched three in a row down-up with gap to the right");
                return Some(location.column + 3);
            }
            if location.column > 0
                && location.row < size - 1
                && board[location.row + 1][location.column - 1] == T::default()
                && (location.row == size - 1
                    || board[location.row + 1][location.column - 1] != T::default())
            {
                println!("Ai: matched three in a row up-down with gap to the left");
                return Some(location.column + 3);
            }
        }
        None
    }

    fn match_location_to_move(
        location: &Location,
        board: &Vec<Vec<T>>,
        matches: &impl Fn(&T) -> bool,
    ) -> Option<usize> {
        Ai::match_vertically(location, board, matches)
            .or_else(|| Ai::match_horizontally(location, board, matches))
            .or_else(|| Ai::match_up_down(location, board, matches))
            .or_else(|| Ai::match_down_up(location, board, matches))
    }

    fn find_token_locations(
        board: &Vec<Vec<T>>,
        discriminator: impl Fn(&T) -> bool,
    ) -> Vec<Location> {
        let mut locations = Vec::<Location>::new();

        for (i, row) in board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if discriminator(cell) {
                    locations.push(Location { row: i, column: j });
                }
            }
        }

        locations
    }

    fn defend(&self, board: &Vec<Vec<T>>) -> Option<usize> {
        let discriminator = |t: &T| *t != self.token && *t != T::default();
        Ai::find_token_locations(board, &discriminator)
            .iter()
            .find_map(|l| Ai::match_location_to_move(l, board, &discriminator))
    }

    fn attack(&self, board: &Vec<Vec<T>>) -> Option<usize> {
        let discriminator = |t: &T| *t == self.token;
        Ai::find_token_locations(board, &discriminator)
            .iter()
            .find_map(|l| Ai::match_location_to_move(l, board, &discriminator))
    }

    fn random(available: &Vec<usize>) -> usize {
        rand::random::<usize>() % available.len()
    }
}
