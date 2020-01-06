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

fn clear() {}

fn print(game: &game::Game, error: &Option<String>) {
    // TODO: Only clear the printed area
//    print!("\x1b[2J");

    if let Some(message) = error {
        println!("Error: {}", message);
    }

    println!("{}", &game);
}

fn main() {
    // TODO: Configure players, size, and depth
    let mut game = game::new();

    let mut turn = false;
    let white = player::new_human(game::Token::White);
    let black = player::new_ai(game::Token::Black, 7);

    let mut error: Option<String> = None;

    clear();
    usage();

    loop {
        let player: &dyn player::Player = if turn { &white } else { &black };
        print(&game, &error);
        error = None;

        match player.play(&game) {
            player::Result::Error(message) => {
                error = Some(message);
            }
            player::Result::Quit => {
                break;
            }
            player::Result::Repeat => {}
            player::Result::Ok(input) => match game.place(player.token(), input) {
                Ok(new_state) => {
                    game = new_state;
                    match game.status() {
                        game::Status::Victory => {
                            print(&game, &None);
                            println!("Player {} won by playing {}", player.token(), input + 1);
                            break;
                        }
                        game::Status::Tie => {
                            print(&game, &None);
                            println!("It's a draw...");
                            break;
                        }
                        _ => {}
                    }
                    turn = !turn;
                }
                Err(e) => {
                    error = Some(e.to_string());
                }
            },
        }
    }
}
