use crate::board;

fn defend(player: board::Player, board: &board::Board) -> Option<i8> {
    Option::None
}

fn attack(player: board::Player, board: &board::Board) -> Option<i8> {
    Option::None
}

fn random(board: &board::Board) -> Option<i8> {
    let column = rand::random::<i8>() % board::Board::SIZE as i8;
    for i in 0..board::Board::SIZE {
        if !board.full_column((column + i as i8) % board::Board::SIZE as i8) {
            return Some((column + i as i8) % board::Board::SIZE as i8);
        }
    }

    None
}

struct Ai {
    player: board::Player,
}

impl Ai {
    pub fn new(player: board::Player) -> Ai {
        Ai { player }
    }

    pub fn next_move(&self, board: &board::Board) -> Option<i8> {
        defend(self.player, &board)
            .or_else(|| attack(self.player, &board))
            .or_else(|| random(&board))
    }
}
