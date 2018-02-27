//! Draws text using loaded fonts.

extern crate amethyst;
#[macro_use]
extern crate log;

mod menu;
mod other;

use std::process;

use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, Pipeline, RenderBundle, Stage};
use amethyst::ui::{DrawUi, UiBundle};

use menu::main_menu;
use menu::MenuBundle;

fn run() -> Result<(), amethyst::Error> {
    let display_config = DisplayConfig::load("resources/display_config.ron");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.], 1.)
            .with_pass(DrawUi::new()),
    );

    let mut app = Application::build("assets", main_menu::State::new())?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
        .with_bundle(MenuBundle)?
        .build()
        .expect("Failed to build application.");

    app.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Failed to run application: {}", e);
        process::exit(1);
    }
}
