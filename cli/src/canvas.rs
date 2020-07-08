struct Canvas {
    height: usize,
}

impl Canvas {
    pub fn new() -> Self {
        Self { height: 0 }
    }

    fn clear(&mut self) {
        for _ in 0..self.height {
            print!("\x1b[K");
            print!("\x1b[1A");
        }
        print!("\x1b[K");
        self.height = 0;
    }

    pub fn print_header(&mut self, error: &str) {
        println!(error);
        self.height += error.chars().filter(|c|  c == &'\n').count() + 1;
    }

    pub fn print_game<Game: super::game::Game>(&mut self, game: Game) {
//        self.height += game.size() + 4;
    }
    pub fn print_footer(&mut self) {
        let a: String = String::new();
        let b: u16 = 234;
        u16::from_str()
    }
}