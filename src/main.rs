mod board;
mod player;
mod token;

enum Outcome {
    Nothing,
    Victory,
    Tie,
}

fn turn(board: &mut board::Board<token::Token>, player: &player::Player<token::Token>) -> Outcome {
    if board.has_available_columns() {
        let next_move = player.next_move(board.available_columns());
        let place_position = board.place(player.token(), next_move);
        if board.check_victory(place_position) {
            Outcome::Victory
        } else {
            Outcome::Nothing
        }
    } else {
        Outcome::Tie
    }
}

fn main() {
    let mut board = board::Board::new();

    // TODO: Dynamically set the players based on args
    let mut opponents = player::Opponents::<
        token::Token,
        player::ai::Ai<token::Token>,
        player::human::Human<token::Token>,
    >::new(token::Token::White, token::Token::Black);

    println!("{}[2J{}", 27 as char, board);

    loop {
        let current_player = opponents.next();
        let outcome = turn(&mut board, current_player);
        println!("{}[2J{}", 27 as char, board);
        match outcome {
            Outcome::Nothing => (),
            Outcome::Victory => {
                println!("Congratulations, {}! You win!", current_player.token());
                break;
            }
            Outcome::Tie => {
                println!("It's a tie..");
                break;
            }
        }
    }
}
