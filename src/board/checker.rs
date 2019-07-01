use board::Board;
use board::Player;
use board::Vector;

pub enum Direction {
    Vertical,
    Horizontal,
    UpDown,
    DownUp,
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

  fn is_same_player(board: &Board, current: &Vector, player: Player) -> bool {
      current.column >= 0
          && current.column < Board::SIZE as i8
          && current.row >= 0
          && current.row < Board::SIZE as i8
          && board.cells[current.row as usize][current.column as usize] == player
  }

  fn measure_sequence(
      board: &Board,
      direction: &Vector,
      current: Vector,
      player: Player,
      counter: u8,
  ) -> u8 {
      if counter >= 4 || !is_same_player(&board, &current, player) {
          counter
      } else {
          measure_sequence(&board, &direction, current.shift(direction), player, counter + 1)
      }
  }

  pub fn check_victory(board: &Board, direction: Direction, current: &Vector, player: Player) -> bool {
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
