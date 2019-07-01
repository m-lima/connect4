use board::Board;
use board::Player;
use board::Vector;

#[test]
fn should_detect_horizontal_victory() {
    let mut board = Board::new();
    board.cells[2][2] = Player::One;
    board.cells[2][3] = Player::One;
    board.cells[2][4] = Player::One;
    board.cells[2][5] = Player::One;
    assert!(board.check_victory(Vector { row: 2, column: 3 }));
}

#[test]
fn should_detect_vertical_victory() {
    let mut board = Board::new();
    board.cells[2][2] = Player::One;
    board.cells[3][2] = Player::One;
    board.cells[4][2] = Player::One;
    board.cells[5][2] = Player::One;
    assert!(board.check_victory(Vector { row: 3, column: 2 }));
}

#[test]
fn should_detect_up_down_victory() {
    let mut board = Board::new();
    board.cells[4][4] = Player::One;
    board.cells[5][3] = Player::One;
    board.cells[6][2] = Player::One;
    board.cells[7][1] = Player::One;
    assert!(board.check_victory(Vector { row: 6, column: 2 }));
}

#[test]
fn should_detect_down_up_victory() {
    let mut board = Board::new();
    board.cells[2][2] = Player::One;
    board.cells[3][3] = Player::One;
    board.cells[4][4] = Player::One;
    board.cells[5][5] = Player::One;
    assert!(board.check_victory(Vector { row: 3, column: 3 }));
}

#[test]
fn should_not_detect_victory() {
    let mut board = Board::new();
    board.cells[2][2] = Player::One;
    board.cells[4][3] = Player::One;
    board.cells[4][5] = Player::One;
    board.cells[5][7] = Player::One;
    assert!(!board.check_victory(Vector { row: 4, column: 5 }));
}
