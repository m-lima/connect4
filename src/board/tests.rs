use board::Board;
use board::Vector;

#[test]
fn should_detect_horizontal_victory() {
    let mut board = Board::new();
    board.cells[2][2] = 1;
    board.cells[2][3] = 1;
    board.cells[2][4] = 1;
    board.cells[2][5] = 1;
    assert!(board.check_victory(Vector { row: 2, column: 3 }));
}

#[test]
fn should_detect_vertical_victory() {
    let mut board = Board::new();
    board.cells[2][2] = 1;
    board.cells[3][2] = 1;
    board.cells[4][2] = 1;
    board.cells[5][2] = 1;
    assert!(board.check_victory(Vector { row: 3, column: 2 }));
}

#[test]
fn should_detect_up_down_victory() {
    let mut board = Board::new();
    board.cells[4][4] = 1;
    board.cells[5][3] = 1;
    board.cells[6][2] = 1;
    board.cells[7][1] = 1;
    assert!(board.check_victory(Vector { row: 6, column: 2 }));
}

#[test]
fn should_detect_down_up_victory() {
    let mut board = Board::new();
    board.cells[2][2] = 1;
    board.cells[3][3] = 1;
    board.cells[4][4] = 1;
    board.cells[5][5] = 1;
    assert!(board.check_victory(Vector { row: 3, column: 3 }));
}

#[test]
fn should_not_detect_victory() {
    let mut board = Board::new();
    board.cells[2][2] = 1;
    board.cells[4][3] = 1;
    board.cells[4][5] = 1;
    board.cells[5][7] = 1;
    assert!(!board.check_victory(Vector { row: 4, column: 5 }));
}
