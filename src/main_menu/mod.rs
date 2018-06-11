//! Main menu module.

pub use self::bundle::MainMenuBundle;
pub use self::index::Index;
pub use self::state::State;
pub use self::system::UiEventHandlerSystem;

mod bundle;
mod index;
mod state;
mod system;
