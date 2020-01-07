#![deny(warnings)]
#![deny(clippy::pedantic)]
#![warn(rust_2018_idioms)]

mod game;
mod player;

fn usage() {
    println!("Usage: connect4 [PLAYERS]");
    println!("\tPLAYERS: \"h\" for human \"a\" for ai");
    println!("\t         \"s\" is a short-hand for \"ah\"");
    println!();
    println!("Example:");
    println!("\tconnect4 ha\t(Player 1: Human, Player 2: AI) [default]");
    println!("\tconnect4 \t(Player 1: Human: Player 2: AI)");
    println!("\tconnect4 s\t(Player 1: AI, Player 2: Human)");
}

fn print<Game: game::Game>(game: &Game, error: &Option<String>) {
    // TODO: Only clear the printed area
    //    print!("\x1b[2J");

    if let Some(message) = error {
        println!("Error: {}", message);
    }

    println!("{}", &game);
}

fn start<White: player::Player, Black: player::Player>(white: &White, black: &Black) {
    use game::Game;
    let mut game = game::new();
    let mut whites_turn = true;
    let mut error: Option<String> = None;

    loop {
        print(&game, &error);
        error = None;

        match if whites_turn {
            white.play(&game)
        } else {
            black.play(&game)
        } {
            player::Result::Error(message) => {
                error = Some(message);
            }
            player::Result::Quit => {
                break;
            }
            player::Result::Repeat => {}
            player::Result::Ok((input, token)) => match game.place(token, input) {
                Ok(new_state) => {
                    game = new_state;
                    match game.status() {
                        game::Status::Victory => {
                            print(&game, &None);
                            println!("Player {} won by playing {}", token, input + 1);
                            break;
                        }
                        game::Status::Tie => {
                            print(&game, &None);
                            println!("It's a draw...");
                            break;
                        }
                        _ => {}
                    }
                    whites_turn = !whites_turn;
                }
                Err(e) => {
                    error = Some(e.to_string());
                }
            },
        }
    }
}

fn main() {
    // TODO: Configure players (possibly over TCP), size, and depth
    usage();

    let (white, black) = (
        player::Ai::new(game::Token::White, 8, false),
        player::Human::new(game::Token::Black),
    );

    start(&white, &black);
}
