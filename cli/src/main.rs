#![deny(warnings)]
#![deny(clippy::pedantic)]
#![warn(rust_2018_idioms)]

enum Result {
    Players(player::Player, player::Player),
    Help,
    Error,
}

fn usage() {
    println!("Usage: connect4 [-h] [-v] [PLAYER [PLAYER]]");
    println!("    PLAYER:");
    println!("        h              Human player");
    println!("        a[level]       AI player, where level=difficulty");
    println!("    -h                 Show this help message");
    println!("    -v                 If an AI is present, make it verbose");
    println!();
    println!("Example:");
    println!("    connect4           White: Human, Black: AI[level=8]");
    println!("    connect4 a6 h      White: AI[level=6], Black: Human");
    println!("    connect4 h         White: Human, Black: Human");
    println!("    connect4 a a9      White: AI[level=8], Black: AI[level=9]");
}

// TODO: Make the stateful canvas less messy
//       If verbose, printing goes whack
fn print<Game: game::Game>(game: &Game, error: &mut Option<String>, clear_size: usize) -> usize {
    println!("{}", &game);

    if let Some(message) = error {
        println!("Error: {}", message);
        *error = None;
        12
    } else {
        11
    }
}

fn start(white: &player::Player, black: &player::Player) {
    use game::Game;
    let mut game = game::new();
    let mut token = game::Token::White;
    let mut error: Option<String> = None;
    let mut canvas = Canvas::new(game.size() + 4);

    loop {
        clear_size = print(&game, &mut error, clear_size);

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
                            print(&game, &mut None, clear_size);
                            println!("Player {} won by playing {}", token, input + 1);
                            break;
                        }
                        game::Status::Tie => {
                            print(&game, &mut None, clear_size);
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

fn parse_args() -> Result {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut verbose = false;
    let mut white: Option<player::Player> = None;
    let mut black: Option<player::Player> = None;

    for arg in &args {
        match arg.as_str() {
            "-h" => {
                if white.is_some() {
                    return Result::Error;
                }
                return Result::Help;
            }
            "-v" => {
                if white.is_some() {
                    return Result::Error;
                }
                verbose = true;
            }
            "h" => {
                if white.is_none() {
                    white = Some(player::Player::Human)
                } else if black.is_none() {
                    black = Some(player::Player::Human)
                }
            }
            _ => {
                if let Some(c) = arg.chars().nth(0) {
                    if c == 'a' {
                        let level_string = arg.chars().skip(1).collect::<String>();
                        let level = if level_string.is_empty() {
                            Ok(8_u8)
                        } else {
                            level_string.parse::<u8>()
                        };
                        if let Ok(level) = level {
                            if white.is_none() {
                                white = Some(player::Player::Ai(player::Ai::new(level, verbose)));
                                continue;
                            } else if black.is_none() {
                                black = Some(player::Player::Ai(player::Ai::new(level, verbose)));
                                continue;
                            }
                        }
                    }
                }
                return Result::Error;
            }
        }
    }

    Result::Players(
        white.unwrap_or_else(|| player::Player::Ai(player::Ai::new(8, verbose))),
        black.unwrap_or_else(|| player::Player::Human),
    )
}

fn main() {
    match parse_args() {
        Result::Help => {
            usage();
        }
        Result::Error => {
            println!("Invalid arguments");
            println!();
            usage();
        }
        Result::Players(white, black) => {
            start(&white, &black);
        }
    }
}
