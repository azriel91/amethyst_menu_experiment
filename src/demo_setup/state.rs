use std::marker::PhantomData;

use amethyst::{self, prelude::*};

/// `State` where resource loading takes place.
///
/// # Type Parameters
///
/// * `S`: State to return after loading is complete.
pub struct State<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: amethyst::State<GameData<'a, 'b>> + 'static,
{
    /// Function to return a `State` that follows this one.
    next_state_fn: Box<F>,
    /// Lifetime tracker.
    state_data: PhantomData<amethyst::State<GameData<'a, 'b>>>,
}

impl<'a, 'b, F, S> State<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: amethyst::State<GameData<'a, 'b>> + 'static,
{
    /// Returns a new `State`
    pub fn new(next_state_fn: Box<F>) -> Self {
        State {
            next_state_fn,
            state_data: PhantomData,
        }
    }
}

impl<'a, 'b, F, S> amethyst::State<GameData<'a, 'b>> for State<'a, 'b, F, S>
where
    F: Fn() -> Box<S>,
    S: amethyst::State<GameData<'a, 'b>> + 'static,
{
    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);

        Trans::Switch((self.next_state_fn)())
    }
}
