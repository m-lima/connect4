pub mod board;

fn main() {
    println!("Hello World!");
    let b = board::Board::new();
    // println!("Cell: {}", b.cell(9, 3));
    println!("{}", b);
}

#[test]
fn should_not_crash() {
    main();
}