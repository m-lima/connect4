mod game;

fn main() -> Result<(), game::Error> {
    let game = game::new();
    println!("{}", game);
    let (game, score) = game.play(game::Player::White, 3)?;
    println!("{}Score: {}", game, score);
    let (game, score) = game.play(game::Player::Black, 3)?;
    println!("{}Score: {}", game, score);
    let (game, score) = game.play(game::Player::White, 3)?;
    println!("{}Score: {}", game, score);
    let (game, score) = game.play(game::Player::White, 1)?;
    println!("{}Score: {}", game, score);
    let (game, score) = game.play(game::Player::Black, 2)?;
    println!("{}Score: {}", game, score);
    let (game, score) = game.play(game::Player::White, 2)?;
    println!("{}Score: {}", game, score);
    Ok(())
}
