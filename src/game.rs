use crate::board::Board;
use crate::board::Token;
use crate::cartesian::Direction;
use crate::cartesian::Position;

pub fn place(board: &mut Board, token: Token, x: u8) -> Result<State, crate::Error> {
    let position = fall_position(board, x)?;
    board.set_cell(position, token)?;
    Ok(update_state(&board, token, position))
}

fn update_state(board: &Board, token: Token, position: Position) -> State {
    if tie(board, position) {
        State::Tie
    } else if victory(board, token, position) {
        State::Victory
    } else {
        State::Ongoing
    }
}

fn tie(board: &Board, position: Position) -> bool {
    position.y() == 0
        && board
            .iter(Position::new(0, 0), Direction::E)
            .any(|c| c == Token::Empty)
}

fn victory(board: &Board, token: Token, position: Position) -> bool {
    direction_victory(board, token, position, Direction::S)
        || direction_victory(&board, token, position, Direction::E)
        || direction_victory(&board, token, position, Direction::NE)
        || direction_victory(&board, token, position, Direction::SE)
}

fn direction_victory(
    board: &Board,
    token: Token,
    position: Position,
    direction: Direction,
) -> bool {
    let reverse = direction.reverse();
    board.count(token, position, direction) + board.count(token, position, reverse) > 4
}

fn fall_position(board: &Board, x: u8) -> Result<Position, crate::Error> {
    if x >= board.size() {
        return Err(crate::Error::OutOfBounds);
    }

    let position = board.count(Token::Empty, Position::new(i16::from(x), 0), Direction::S);

    if position == 0 {
        Err(crate::Error::ColumnFull)
    } else {
        Ok(Position::new(i16::from(x), i16::from(position) - 1))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Victory,
    Tie,
    Ongoing,
}

#[cfg(test)]
mod test {
    use super::State;
    use crate::board::Board;
    use crate::board::Token::{Black, White};
    use crate::cartesian::Position;

    #[test]
    fn place() {
        let mut board = Board::new(7);
        let state = super::place(&mut board, Black, 1).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, White, 2).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, Black, 3).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, White, 3).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, Black, 3).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, White, 3).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, Black, 4).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, White, 1).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, Black, 2).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, White, 2).unwrap();
        assert_eq!(state, State::Ongoing);
        let state = super::place(&mut board, White, 0).unwrap();
        assert_eq!(state, State::Victory);
    }

    #[test]
    fn place_errors() {
        let mut board = Board::new(7);

        super::place(&mut board, White, 3).unwrap();
        super::place(&mut board, White, 3).unwrap();
        super::place(&mut board, White, 3).unwrap();
        super::place(&mut board, White, 3).unwrap();
        super::place(&mut board, White, 3).unwrap();
        super::place(&mut board, White, 3).unwrap();
        super::place(&mut board, White, 3).unwrap();

        assert_eq!(
            super::place(&mut board, Black, 3).err().unwrap(),
            crate::Error::ColumnFull
        );
        assert_eq!(
            super::place(&mut board, Black, 9).err().unwrap(),
            crate::Error::OutOfBounds
        );
    }

    #[test]
    fn victory() {
        let size = 7;
        let mut board = Board::new(size);

        board.set_cell(Position::new(2, 6), Black).unwrap();
        board.set_cell(Position::new(2, 5), Black).unwrap();
        board.set_cell(Position::new(2, 4), Black).unwrap();
        board.set_cell(Position::new(2, 3), White).unwrap();

        board.set_cell(Position::new(0, 0), White).unwrap();
        board.set_cell(Position::new(0, 1), White).unwrap();

        board.set_cell(Position::new(6, 6), Black).unwrap();
        board.set_cell(Position::new(5, 5), Black).unwrap();
        board.set_cell(Position::new(4, 4), Black).unwrap();
        board.set_cell(Position::new(5, 4), Black).unwrap();

        assert_eq!(super::victory(&board, Black, Position::new(2, 5)), false);
        assert_eq!(super::victory(&board, White, Position::new(0, 1)), false);
        assert_eq!(super::victory(&board, Black, Position::new(5, 5)), false);

        board.set_cell(Position::new(3, 3), Black).unwrap();
        assert_eq!(super::victory(&board, White, Position::new(5, 5)), false);
        assert_eq!(super::victory(&board, Black, Position::new(5, 5)), true);
    }

    #[test]
    fn fall_position() {
        let size = 7;
        let mut board = Board::new(size);
        board.set_cell(Position::new(2, 3), Black).unwrap();
        board.set_cell(Position::new(4, 6), White).unwrap();

        assert_eq!(
            super::fall_position(&board, 0).unwrap(),
            Position::new(0, 6)
        );
        assert_eq!(
            super::fall_position(&board, 2).unwrap(),
            Position::new(2, 2)
        );
        assert_eq!(
            super::fall_position(&board, 4).unwrap(),
            Position::new(4, 5)
        );
    }
}
