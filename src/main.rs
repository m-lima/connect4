use player::Player;

mod board;
mod player;
mod token;

enum Outcome {
    Nothing,
    Victory,
    Tie,
}

fn turn(
    board: &mut board::Board<token::Token>,
    player: &impl player::Player<token::Token>,
) -> Outcome {
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
    let mut white_is_current_player = true;

    // TODO: Dynamically set the players based on args
    let players = player::Players::<
        token::Token,
        player::human::Human<token::Token>,
        player::human::Human<token::Token>,
    >::new(token::Token::White, token::Token::Black);

    println!("{}[2J{}", 27 as char, board);

    loop {
        // TODO: Less verbose and copy/paste fetching of current player
        let outcome = turn(
            &mut board,
            if white_is_current_player {
                players.white()
            } else {
                players.black()
            },
        );
        println!("{}[2J{}", 27 as char, board);
        match outcome {
            Outcome::Nothing => (),
            Outcome::Victory => {
                // TODO: Less verbose and copy/paste fetching of current player
                println!(
                    "Congratulations, {}! You win!",
                    if white_is_current_player {
                        players.white().token()
                    } else {
                        players.black().token()
                    }
                );
                break;
            }
            Outcome::Tie => {
                println!("It's a tie..");
                break;
            }
        }
        white_is_current_player = !white_is_current_player;
    }
}
