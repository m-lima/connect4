pub struct Ai<T> {
    token: T,
}

impl<T: Copy> super::Player<T> for Ai<T> {
    fn next_move(&self, available: &Vec<usize>) -> usize {
        rand::random::<usize>() % available.len()
    }

    fn token(&self) -> T {
        self.token
    }
}

impl<T> super::PlayerBuilder<T> for Ai<T> {
    fn new(token: T) -> Ai<T> {
        Ai { token }
    }
}
