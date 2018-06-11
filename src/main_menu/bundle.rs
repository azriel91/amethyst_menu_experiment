use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;

use main_menu::UiEventHandlerSystem;

/// This bundle prepares the world for a menu.
#[derive(Debug)]
pub struct MainMenuBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MainMenuBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            UiEventHandlerSystem::new(),
            "",
            &["ui_system", "ui_mouse_system"],
        );
        Ok(())
    }
}
