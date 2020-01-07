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

fn print<Game: game::Game>(game: &Game, error: &mut Option<String>) {
    // TODO: Only clear the printed area
    //    print!("\x1b[2J");

    if let Some(message) = error {
        println!("Error: {}", message);
        *error = None;
    }

    println!("{}", &game);
}

fn start(white: &player::Player, black: &player::Player) {
    use game::Game;
    let mut game = game::new();
    let mut token = game::Token::White;
    let mut error: Option<String> = None;

    loop {
        print(&game, &mut error);

        let play = match token {
            game::Token::White => white.play(&game, token),
            game::Token::Black => black.play(&game, token),
        };

        match play {
            player::Result::Ok(input) => match game.place(token, input) {
                Ok(new_state) => {
                    game = new_state;
                    match game.status() {
                        game::Status::Victory => {
                            print(&game, &mut None);
                            println!("Player {} won by playing {}", token, input + 1);
                            break;
                        }
                        game::Status::Tie => {
                            print(&game, &mut None);
                            println!("It's a draw...");
                            break;
                        }
                        _ => {}
                    }
                    token = !token;
                }
                Err(e) => {
                    error = Some(e.to_string());
                }
            },
            player::Result::Error(message) => {
                error = Some(message);
            }
            player::Result::Repeat => {}
            player::Result::Quit => {
                break;
            }
        }
    }
}

fn main() {
    // TODO: Configure players (possibly over TCP), size, and depth
    usage();

    //    let (white, black) = (
    //        player::Ai::new(game::Token::White, 8, false),
    //        player::Human::new(game::Token::Black),
    //    );
    let (white, black) = (
        player::Player::Ai(player::Ai::new(8, false)),
        player::Player::Human,
    );

    start(&white, &black);
}
