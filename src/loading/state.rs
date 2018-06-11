use std::marker::PhantomData;

use amethyst::{
    self, assets::{AssetStorage, Loader}, prelude::*, ui::{FontAsset, TtfFormat},
};

/// `State` where resource loading takes place.
///
/// # Type Parameters
///
/// * `S`: State to return after loading is complete.
pub struct State<'a, 'b, S>
where
    S: amethyst::State<GameData<'a, 'b>> + 'static,
{
    /// The `State` that follows this one.
    next_state: Option<Box<S>>,
    /// Lifetime tracker.
    state_data: PhantomData<amethyst::State<GameData<'a, 'b>>>,
}

impl<'a, 'b, S> State<'a, 'b, S>
where
    S: amethyst::State<GameData<'a, 'b>> + 'static,
{
    /// Returns a new `State`
    pub fn new(next_state: Box<S>) -> Self {
        State {
            next_state: Some(next_state),
            state_data: PhantomData,
        }
    }
}

impl<'a, 'b, S> amethyst::State<GameData<'a, 'b>> for State<'a, 'b, S>
where
    S: amethyst::State<GameData<'a, 'b>> + 'static,
{
    fn on_start(&mut self, data: StateData<GameData>) {
        let font_handle = {
            // `world` is borrowed immutably in here for `loader` and `font_storage`
            let loader = data.world.read_resource::<Loader>();
            let font_storage = data.world.read_resource::<AssetStorage<FontAsset>>();
            loader.load(
                "font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-Bold.ttf",
                TtfFormat,
                (),
                (),
                &font_storage,
            )
        };

        // `world` is borrowed mutably here to add the font handle
        data.world.add_resource(font_handle);
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);

        Trans::Switch(
            self.next_state
                .take()
                .expect("Expected `next_state` to be set"),
        )
    }
}
