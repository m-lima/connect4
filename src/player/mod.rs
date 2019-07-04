pub mod ai;
pub mod human;

pub trait Player<T> {
    fn next_move(&self, available: &Vec<usize>) -> usize;
    fn token(&self) -> T;
}

pub trait PlayerBuilder<T> {
    fn new(token: T) -> Self;
}

pub struct Players<T, W: Player<T>, B: Player<T>> {
    white: W,
    black: B,
    phantom: std::marker::PhantomData<T>,
}

impl<T, W: Player<T> + PlayerBuilder<T>, B: Player<T> + PlayerBuilder<T>> Players<T, W, B> {
    pub fn new(w: T, b: T) -> Players<T, W, B> {
        Players {
            white: W::new(w),
            black: B::new(b),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn white(&self) -> &W {
        &self.white
    }

    pub fn black(&self) -> &B {
        &self.black
    }
}
