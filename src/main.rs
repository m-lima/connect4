#![deny(warnings)]
#![deny(clippy::pedantic)]
#![warn(rust_2018_idioms)]

use crate::player::Player;

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

fn main() {
    let mut game = game::new();
    let white = player::new_human(game::Token::White);
    let black = player::new_human(game::Token::White);

    usage();

    game = game.place(game::Token::White, 3).unwrap();
    game = game.place(game::Token::Black, 3).unwrap();
    game = game.place(game::Token::White, 3).unwrap();
    game = game.place(game::Token::Black, 0).unwrap();
    game = game.place(game::Token::White, 1).unwrap();
    game = game.place(game::Token::Black, 2).unwrap();
    game = game.place(game::Token::White, 2).unwrap();

    loop {
        println!("Score: {}", game.last_score());
        println!("Potential: {}", game.plan(white.token(), 0).unwrap_or(0));
        println!("{}", &game);
        game = game
            .place(white.token(), white.play())
            .expect("failed to place");
        println!("Score: {}", game.last_score());
        println!("{}", &game);
        game = game
            .place(black.token(), black.play())
            .expect("failed to place");
    }
}
