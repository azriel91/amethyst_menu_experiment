use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::bundle::{ECSBundle, Result};
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::ui::{FontAsset, TtfFormat};

use menu::main_menu;
use menu::{MenuItem, UiEventHandlerSystem};

/// This bundle prepares the world for a menu.
pub struct MenuBundle;

impl<'a, 'b> ECSBundle<'a, 'b> for MenuBundle {
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register::<MenuItem<main_menu::Index>>();

        let font_handle = {
            // `world` is borrowed immutably in here for `loader` and `font_storage`
            let loader = world.read_resource::<Loader>();
            let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
            loader.load(
                "font/source-code-pro-2.030R-ro-1.050R-it/TTF/SourceCodePro-Bold.ttf",
                TtfFormat,
                (),
                (),
                &font_storage,
            )
        };

        // `world` is borrowed mutably here to add the font handle
        world.add_resource(font_handle);

        Ok(builder.add(UiEventHandlerSystem::new(), "ui_event_handler", &[]))
    }
}
