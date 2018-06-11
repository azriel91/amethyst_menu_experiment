use amethyst::{
    self, ecs::Entity, input::is_key, prelude::*,
    renderer::{Event, ScreenDimensions, VirtualKeyCode},
    ui::{Anchor, FontHandle, UiText, UiTransform},
};

const FONT_SIZE: f32 = 17.;

#[derive(Debug, Default)]
pub struct State {
    /// Holds the info label.
    entity: Option<Entity>,
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    fn initialize_informative(&mut self, world: &mut World) {
        let font_bold = read_font(world);

        let screen_w = {
            let dim = world.read_resource::<ScreenDimensions>();
            dim.width()
        };
        let text_w = screen_w / 3.;
        let text_h = 50.;

        let text_transform = UiTransform::new(
            "informative".to_string(),
            Anchor::Middle,
            0.,
            text_h / 2.,
            1.,
            text_w,
            text_h,
            0,
        );
        let info_entity = world
            .create_entity()
            .with(text_transform)
            .with(UiText::new(
                font_bold.clone(),
                "Press [Escape] to return to the previous menu.".to_string(),
                [1., 1., 1., 1.],
                FONT_SIZE,
            ))
            .build();

        self.entity.get_or_insert(info_entity);
    }

    fn terminate_informative(&mut self, world: &mut World) {
        world
            .delete_entity(self.entity.take().expect("Expected info_entity to be set."))
            .expect("Failed to delete info_entity.");
    }
}

impl<'a, 'b> amethyst::State<GameData<'a, 'b>> for State {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.initialize_informative(&mut data.world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_key(&event, VirtualKeyCode::Escape) {
            info!("Returning from `other::State`.");
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.terminate_informative(&mut data.world);
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn read_font(world: &mut World) -> FontHandle {
    world.read_resource::<FontHandle>().clone()
}
