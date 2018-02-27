use amethyst;
use amethyst::prelude::*;
use amethyst::renderer::ScreenDimensions;
use amethyst::ui::{FontHandle, MouseReactive, UiResize, UiText, UiTransform};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ecs::Entity;

use menu::main_menu;
use menu::{MenuEvent, MenuItem};
use other;

const FONT_SIZE: f32 = 25.;

/// Main menu with options to start a game or exit.
#[derive(Debug, Default)]
pub struct State {
    /// ID of the reader for application events.
    menu_event_reader: Option<ReaderId<MenuEvent<main_menu::Index>>>,
    /// Menu item entities, to be deleted `on_stop()`
    entities: Vec<Entity>,
}

impl State {
    /// Returns a `State`
    pub fn new() -> Self {
        Default::default()
    }

    fn initialize_menu_event_channel(&mut self, world: &mut World) {
        let mut menu_event_channel = EventChannel::<MenuEvent<main_menu::Index>>::with_capacity(20);
        let reader_id = menu_event_channel.register_reader();
        self.menu_event_reader.get_or_insert(reader_id);

        world.add_resource(menu_event_channel);
    }

    fn terminate_menu_event_channel(&mut self, _world: &mut World) {
        // By design there is no function to unregister a reader from an `EventChannel`.
        // Nor is there one to remove a resource from the `World`.

        self.menu_event_reader.take();
    }

    fn initialize_menu_items(&mut self, world: &mut World) {
        let font_bold = read_font(world);

        let mut menu_items = vec![main_menu::Index::StartGame, main_menu::Index::Exit];
        menu_items
            .drain(..)
            .enumerate()
            .for_each(|(index, menu_item)| {
                let mut text_transform = UiTransform::new(
                    menu_item.title().to_string(),
                    20.,
                    index as f32 * 50. + 20.,
                    1.,
                    400.,
                    100.,
                    0,
                );
                let ui_text_size_fn = |_transform: &mut UiTransform, (_width, _height)| {};

                {
                    let dim = world.read_resource::<ScreenDimensions>();
                    ui_text_size_fn(&mut text_transform, (dim.width(), dim.height()));
                }

                let entity = world
                    .create_entity()
                    .with(text_transform)
                    .with(UiText::new(
                        font_bold.clone(),
                        menu_item.title().to_string(),
                        [1., 1., 1., 1.],
                        FONT_SIZE,
                    ))
                    .with(UiResize(Box::new(ui_text_size_fn)))
                    .with(MouseReactive)
                    .with(MenuItem { index: menu_item })
                    .build();

                self.entities.push(entity);
            });
    }

    fn terminate_menu_items(&mut self, world: &mut World) {
        self.entities.drain(..).for_each(|entity| {
            world
                .delete_entity(entity)
                .expect("Failed to delete menu item entity.");
        });
    }
}

impl amethyst::State for State {
    fn on_start(&mut self, world: &mut World) {
        self.initialize_menu_event_channel(world);
        self.initialize_menu_items(world);
    }

    fn update(&mut self, world: &mut World) -> Trans {
        let menu_event_channel = world.read_resource::<EventChannel<MenuEvent<main_menu::Index>>>();

        let mut reader_id = self.menu_event_reader
            .as_mut()
            .expect("Expected menu_event_reader to be set");
        let mut storage_iterator = menu_event_channel.read(&mut reader_id);
        match storage_iterator.next() {
            Some(event) => match *event {
                MenuEvent::Select(main_menu::Index::StartGame) => {
                    Trans::Push(Box::new(other::State::new()))
                }
                MenuEvent::Select(main_menu::Index::Exit) => Trans::Quit,
                MenuEvent::Close => Trans::Quit,
            },
            None => Trans::None,
        }
    }

    fn on_stop(&mut self, world: &mut World) {
        self.terminate_menu_items(world);
        self.terminate_menu_event_channel(world);
    }
}

fn read_font(world: &mut World) -> FontHandle {
    world.read_resource::<FontHandle>().clone()
}
