use board::Board;
use board::Player;
use board::Vector;

pub enum Direction {
    Vertical,
    Horizontal,
    UpDown,
    DownUp,
}

impl Vector<usize> {
    fn shift(&self, direction: &Vector<i8>) -> Option<Vector<usize>> {
        if (direction.row < 0 && self.row == 0) || (direction.column < 0 && direction.column == 0) {
            None
        } else {
            Some(Vector {
                column: self.column + direction.column as usize,
                row: self.row + direction.row as usize,
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

fn is_same_player(board: &Board, current: &Vector<usize>, player: Player) -> bool {
    current.column < Board::SIZE
        && current.row < Board::SIZE
        && board.cells[current.row][current.column] == player
}

fn measure_sequence(
    board: &Board,
    direction: &Vector<i8>,
    current: Option<Vector<usize>>,
    player: Player,
    counter: u8,
) -> u8 {
    current
        .filter(|_| counter < 4)
        .filter(|c| is_same_player(&board, &c, player))
        .map_or_else(
            || counter,
            |c| measure_sequence(&board, &direction, c.shift(direction), player, counter + 1),
        )
}

pub fn check_victory(
    board: &Board,
    direction: Direction,
    current: &Vector<usize>,
    player: Player,
) -> bool {
    let vector = Vector::from_direction(direction);
    let backwards_vector = vector.invert();
    measure_sequence(
        &board,
        &vector,
        current.shift(&vector),
        player,
        measure_sequence(
            &board,
            &backwards_vector,
            current.shift(&backwards_vector),
            player,
            1,
        ),
    ) >= 4
}
