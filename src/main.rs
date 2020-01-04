#![deny(warnings)]
#![deny(clippy::pedantic)]
#![warn(rust_2018_idioms)]

mod game;

fn main() -> Result<(), game::Error> {
    let game = game::new();
    game.play(game::Player::White, 3)?;
    game.play(game::Player::Black, 3)?;
    Ok(())
}
