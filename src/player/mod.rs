pub mod ai;
pub mod human;

pub trait Player<T> {
    fn next_move(&self, available: &Vec<usize>, board: &Vec<Vec<T>>) -> usize;
    fn token(&self) -> T;
}

pub trait PlayerBuilder<T> {
    fn new(token: T) -> Self;
}

pub struct Opponents<T, W: Player<T>, B: Player<T>> {
    white: W,
    black: B,
    toggle: bool,
    phantom: std::marker::PhantomData<T>,
}

impl<T, W: Player<T> + PlayerBuilder<T>, B: Player<T> + PlayerBuilder<T>> Opponents<T, W, B> {
    pub fn new(w: T, b: T) -> Opponents<T, W, B> {
        Opponents {
            white: W::new(w),
            black: B::new(b),
            toggle: false,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T, W: Player<T>, B: Player<T>> Opponents<T, W, B> {
    pub fn next(&mut self) -> &Player<T> {
        self.toggle = !self.toggle;
        if self.toggle {
            &self.white
        } else {
            &self.black
        }
    }
}
