use amethyst::{
    self, ecs::Entity, prelude::*, renderer::ScreenDimensions, shrev::{EventChannel, ReaderId},
    ui::{Anchor, FontHandle, MouseReactive, UiText, UiTransform},
};

use main_menu;
use menu::{MenuEvent, MenuItem};

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

        let screen_w = {
            let dim = world.read_resource::<ScreenDimensions>();
            dim.width()
        };
        let text_w = screen_w / 3.;
        let text_h = 50.;

        let mut menu_items = vec![main_menu::Index::StartGame, main_menu::Index::Exit];
        let total_items = menu_items.len() as f32;
        menu_items
            .drain(..)
            .enumerate()
            .for_each(|(index, menu_item)| {
                let text_transform = UiTransform::new(
                    menu_item.title().to_string(),
                    Anchor::Middle,
                    0.,
                    (index as f32 * text_h) - (total_items * text_h / 2.),
                    1.,
                    text_w,
                    text_h,
                    0,
                );

                let entity = world
                    .create_entity()
                    .with(text_transform)
                    .with(UiText::new(
                        font_bold.clone(),
                        menu_item.title().to_string(),
                        [1., 1., 1., 1.],
                        FONT_SIZE,
                    ))
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

impl<'a, 'b> amethyst::State<GameData<'a, 'b>> for State {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.initialize_menu_event_channel(&mut data.world);
        self.initialize_menu_items(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.terminate_menu_items(&mut data.world);
        self.terminate_menu_event_channel(&mut data.world);
    }

    // Need to explicitly hide and show the menu items during pause and resume
    fn on_resume(&mut self, mut data: StateData<GameData>) {
        self.initialize_menu_items(&mut data.world);
    }

    fn on_pause(&mut self, mut data: StateData<GameData>) {
        self.terminate_menu_items(&mut data.world);
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        let menu_event_channel = &data
            .world
            .read_resource::<EventChannel<MenuEvent<main_menu::Index>>>();

        let mut reader_id = self
            .menu_event_reader
            .as_mut()
            .expect("Expected menu_event_reader to be set");
        let mut storage_iterator = menu_event_channel.read(&mut reader_id);
        match storage_iterator.next() {
            Some(event) => match *event {
                MenuEvent::Select(index) => index.trans(),
                MenuEvent::Close => Trans::Quit,
            },
            None => Trans::None,
        }
    }
}

fn read_font(world: &mut World) -> FontHandle {
    world.read_resource::<FontHandle>().clone()
}
