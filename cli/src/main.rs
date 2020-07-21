#![deny(warnings)]
#![deny(clippy::pedantic)]
#![warn(rust_2018_idioms)]

use connect4::ai::Ai;
use connect4::board::Board;
use connect4::board::Token;
use connect4::game;

mod canvas;

enum Result {
    Players(Player, Player, u8),
    Help,
    Error,
}

enum Player {
    Human,
    Ai(Ai),
}

fn usage() {
    println!("Usage: connect4 [-h] [-v] [PLAYER [PLAYER]]");
    println!("    PLAYER:");
    println!("        h              Human player");
    println!("        a[level]       AI player, where level=difficulty");
    println!("    s<size>            Set the size of the board");
    println!("    -h                 Show this help message");
    println!("    -v                 If an AI is present, make it verbose");
    println!();
    println!("Example:");
    println!("    connect4           White: Human, Black: AI[level=8]");
    println!("    connect4 a6 h      White: AI[level=6], Black: Human");
    println!("    connect4 h         White: Human, Black: Human");
    println!("    connect4 a a9      White: AI[level=8], Black: AI[level=9]");
    println!("    connect4 s9 a a    White: AI[level=8], Black: AI[level=8], Board: 9x9");
}

fn start(white: &Player, black: &Player, size: u8) {
    let mut board = Board::new(size);
    let mut token = game::Token::White;
    let mut canvas = Canvas::new(size + 4);

    loop {
        clear_size = print(&game, &mut error, clear_size);
        //
        //     let play = match token {
        //         game::Token::White => white.play(&game, token),
        //         game::Token::Black => black.play(&game, token),
        //     };
        //
        //     match play {
        //         player::Result::Ok(input) => match game.place(token, input) {
        //             Ok(new_state) => {
        //                 game = new_state;
        //                 match game.status() {
        //                     game::Status::Victory => {
        //                         print(&game, &mut None, clear_size);
        //                         println!("Player {} won by playing {}", token, input + 1);
        //                         break;
        //                     }
        //                     game::Status::Tie => {
        //                         print(&game, &mut None, clear_size);
        //                         println!("It's a draw...");
        //                         break;
        //                     }
        //                     _ => {}
        //                 }
        //                 token = !token;
        //             }
        //             Err(e) => {
        //                 error = Some(e.to_string());
        //             }
        //         },
        //         player::Result::Error(message) => {
        //             error = Some(message);
        //         }
        //         player::Result::Repeat => {}
        //         player::Result::Quit => {
        //             break;
        //         }
        //     }
    }
}

fn parse_args() -> Result {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut verbose = false;
    let mut white: Option<Player> = None;
    let mut black: Option<Player> = None;
    let mut size = 7_u8;

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
                    white = Some(Player::Human)
                } else if black.is_none() {
                    black = Some(Player::Human)
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
                                white = Some(Player::Ai(Ai::new(Token::White, level, verbose)));
                                continue;
                            } else if black.is_none() {
                                black = Some(Player::Ai(Ai::new(Token::Black, level, verbose)));
                                continue;
                            }
                        }
                    } else if c == 's' {
                        let size_string = arg.chars().skip(1).collect::<String>();
                        if let Ok(parsed_size) = size_string.parse::<u8>() {
                            size = parsed_size;
                        } else {
                            return Result::Error;
                        }
                    }
                }
                return Result::Error;
            }
        }
    }

    Result::Players(
        white.unwrap_or_else(|| Player::Ai(Ai::new(Token::White, 8, verbose))),
        black.unwrap_or_else(|| Player::Human),
        size,
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
        Result::Players(white, black, size) => {
            start(&white, &black, size);
        }
    }
}
