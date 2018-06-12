use amethyst::{self, prelude::*};

/// `State` where resource loading takes place.
///
/// # Type Parameters
///
/// * `S`: State to return after loading is complete.
pub struct State<F, T>
where
    F: Fn() -> Box<amethyst::State<T>>,
{
    /// Function to return a `State` that follows this one.
    next_state_fn: Box<F>,
}

impl<F, T> State<F, T>
where
    F: Fn() -> Box<amethyst::State<T>>,
{
    /// Returns a new `State`
    pub fn new(next_state_fn: Box<F>) -> Self {
        State { next_state_fn }
    }
}

impl<F, T> amethyst::State<T> for State<F, T>
where
    F: Fn() -> Box<amethyst::State<T>>,
{
    fn update(&mut self, data: StateData<T>) -> Trans<T> {
        Trans::Switch((self.next_state_fn)())
    }
}
