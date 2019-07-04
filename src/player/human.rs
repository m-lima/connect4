pub struct Human<T> {
    token: T,
}

impl<T: std::fmt::Display + Copy> super::Player<T> for Human<T> {
    fn next_move(&self, available: &Vec<usize>) -> usize {
        self.get_input(available).unwrap()
    }

    fn token(&self) -> T {
        self.token
    }
}

impl<T> super::PlayerBuilder<T> for Human<T> {
    fn new(token: T) -> Human<T> {
        Human { token }
    }
}

impl<T: std::fmt::Display> Human<T> {
    fn prompt(&self) {
        use std::io::Write;
        print!("Select the column for {}: ", self.token);
        std::io::stdout().flush().expect("Unable to flush output");
    }

    fn get_input(&self, available: &Vec<usize>) -> Result<usize, &'static str> {
        self.prompt();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Unable to read user input");
        input
            .trim()
            .to_string()
            .parse::<i8>()
            .map_err(|_| "not a number")
            .map(|v| (v - 1) as usize)
            .and_then(|v| {
                available
                    .binary_search(&v)
                    .map_err(|_| "not a valid column")
            })
            .or_else(|e| {
                println!("Invalid input: {}", e);
                self.get_input(available)
            })
    }
}
