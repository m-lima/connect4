mod board;

fn get_input() -> Result<u8, &'static str> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("get_input: unable to read user input");
    match input.trim().parse::<u8>() {
        Ok(value) => Ok(value),
        Err(_) => Err("not a number"),
    }
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
    let mut running = true;
    let mut current_player = board::Player::One;
    println!("{}", board);

    while running {
        use std::io::Write;
        print!("Select the column for {}: ", current_player);
        std::io::stdout()
            .flush()
            .expect("error: enable to flush output");
        get_input()
            .and_then(|v| {
                if v == 0 {
                    running = false;
                    Ok(())
                } else {
                    board.place(current_player, v).map_err(|e| e.to_string())
                }
            })
            .and_then(|_| {
                if running {
                    println!("{}", board);
                    swap_player(&mut current_player);
                }
                Ok(())
            })
            .unwrap_or_else(|s| println!("Invalid input: {}", s));
    }
}

#[test]
fn should_not_crash() {
    main();
}