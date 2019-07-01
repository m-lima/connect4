mod board;

fn get_input() -> Result<String, &'static str> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("get_input: unable to read user input");
    Ok(input.trim().to_string())
}

fn swap_player(player: &mut board::Player) {
    *player = if *player == board::Player::One {
        board::Player::Two
    } else {
        board::Player::One
    };
}

fn main() {
    let mut board = board::Board::new();
    let mut current_player = board::Player::One;
    println!("{}", board);

    loop {
        use std::io::Write;
        print!(
            "Select the column for {} (or 'q' to quit): ",
            current_player
        );
        std::io::stdout()
            .flush()
            .expect("error: enable to flush output");
        get_input()
            .and_then(|v| {
                if v == "q" {
                    std::process::exit(0);
                }
                v.parse::<u8>().map_err(|_| "not a number")
            })
            .map(|v| v as i8)
            .and_then(|v| {
                board
                    .place(current_player, v - 1)
                    .map_err(|e| e.to_string())
            })
            .map(|p| board.check_victory(p))
            .and_then(|v| {
                println!("{}", board);
                if v {
                    println!("Congratulaions {}! You own!", current_player);
                    std::process::exit(0);
                }
                swap_player(&mut current_player);
                Ok(())
            })
            .unwrap_or_else(|s| println!("Invalid input: {}", s));
    }
}

#[test]
fn should_not_crash() {
    let _board = board::Board::new();
}

#[test]
fn player_should_swap() {
    let mut player = board::Player::One;
    swap_player(&mut player);

    assert_eq!(player, board::Player::Two);
}