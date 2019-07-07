extern crate clap;

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
        let next_move = player.next_move(board.available_columns(), board.to_vec());
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

fn play<
    W: player::Player<token::Token> + player::PlayerBuilder<token::Token>,
    B: player::Player<token::Token> + player::PlayerBuilder<token::Token>,
>(
    mut board: board::Board<token::Token>,
) {
    let mut opponents =
        player::Opponents::<token::Token, W, B>::new(token::Token::White, token::Token::Black);

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

fn main() {
    let matches = clap::App::new("Connect4")
        .version("0.1")
        .author("Marcelo Lima")
        .about("Play Connect4 in Rust")
        .arg(
            clap::Arg::with_name("size")
                .short("s")
                .long("size")
                .default_value("8")
                .value_name("SIZE")
                .help("Sets the size of the board")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("player1")
                .short("1")
                .long("p1")
                .default_value("h")
                .possible_values(&["a", "h"])
                .help("Sets the player 1 (a: AI, h: Human)")
                .value_name("PLAYER")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("player2")
                .short("2")
                .long("p2")
                .default_value("a")
                .possible_values(&["a", "h"])
                .help("Sets the player 2 (a: AI, h: Human)")
                .value_name("PLAYER")
                .takes_value(true),
        )
        .get_matches();

    let board = board::Board::new(
        matches
            .value_of("size")
            .unwrap_or("8")
            .parse::<usize>()
            .unwrap(),
    );

    let players = matches.value_of("player1").unwrap_or("h").to_owned()
        + matches.value_of("player2").unwrap_or("a");

    {
        use player::ai;
        use player::human;
        match players.as_str() {
            "aa" => play::<ai::Ai<token::Token>, ai::Ai<token::Token>>(board),
            "ah" => play::<ai::Ai<token::Token>, human::Human<token::Token>>(board),
            "ha" => play::<human::Human<token::Token>, ai::Ai<token::Token>>(board),
            "hh" => play::<human::Human<token::Token>, human::Human<token::Token>>(board),
            _ => panic!("Invalid players"),
        };
    }
}
