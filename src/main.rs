//! Draws text using loaded fonts.

extern crate amethyst;
#[macro_use]
extern crate log;

mod demo_setup;
mod loading;
mod main_menu;
mod menu;
mod other;

use std::process;

use amethyst::{
    core::transform::TransformBundle, input::InputBundle, prelude::*,
    renderer::{DisplayConfig, Pipeline, RenderBundle, Stage}, ui::{DrawUi, UiBundle},
};

use main_menu::MainMenuBundle;

fn run() -> Result<(), amethyst::Error> {
    let display_config = DisplayConfig::load("resources/display_config.ron");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.], 1.)
            .with_pass(DrawUi::new()),
    );

    let loading_state = loading::State::new(Box::new(main_menu::State::new()));

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
        .with_bundle(MainMenuBundle)?;
    let mut app = Application::new("assets", loading_state, game_data)?;

    app.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Failed to run application: {}", e);
        process::exit(1);
    }
}
