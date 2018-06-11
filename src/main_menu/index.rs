use amethyst::{self, prelude::*};

use demo_setup;
use other;

/// Indicies of main menu items.
#[derive(Debug, Clone, Copy)]
pub enum Index {
    /// Menu item for starting a game.
    StartGame,
    /// Menu item for demo mode.
    Demo,
    /// Menu item for exiting the application.
    Exit,
}

impl Index {
    /// Returns a human readable string of this menu item.
    pub fn title(&self) -> &str {
        match *self {
            Index::StartGame => "Start Game",
            Index::Demo => "Demo",
            Index::Exit => "Exit",
        }
    }

    pub fn trans<'a, 'b>(&self) -> Trans<GameData<'a, 'b>> {
        match *self {
            Index::StartGame => Trans::Push(Box::new(other::State::new())),
            Index::Demo => {
                let next_state_fn = Box::new(|| -> Box<amethyst::State<GameData<'a, 'b>>> {
                    Box::new(other::State::new())
                });
                let demo_setup_state = demo_setup::State::new(next_state_fn);
                Trans::Push(Box::new(demo_setup_state))
            }
            Index::Exit => Trans::Quit,
        }
    }
}
