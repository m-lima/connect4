pub mod board;

fn main() {
    let b = board::Board::new();
    println!("{}", b);
}

#[test]
fn should_not_crash() {
    main();
}