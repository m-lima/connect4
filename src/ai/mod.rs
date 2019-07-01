use crate::board;

fn defend(player: board::Player, board: &board::Board) -> Option<usize> {
    Option::None
}

fn attack(player: board::Player, board: &board::Board) -> Option<usize> {
    Option::None
}

fn random(board: &board::Board) -> Option<usize> {
    let column = rand::random::<usize>() % board::Board::SIZE;
    for i in 0..board::Board::SIZE {
        if !board.full_column((column + i) % board::Board::SIZE) {
            return Some((column + i) % board::Board::SIZE);
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

    pub fn next_move(&self, board: &board::Board) -> Option<usize> {
        defend(self.player, &board)
            .or_else(|| attack(self.player, &board))
            .or_else(|| random(&board))
    }
}
