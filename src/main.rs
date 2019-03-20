use amethyst::{
    assets::{PrefabLoader, PrefabLoaderSystem, Processor, RonFormat},
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::prelude::{System, Write},
    audio::{output::init_output, Source},
    input::{is_close_requested, is_key_down, InputBundle},
    prelude::*,
    renderer::{DrawShaded, PosNormTex},
    shrev::{EventChannel, ReaderId},
    ui::{UiBundle, UiCreator, UiEvent, UiEventType},
    utils::{
        scene::BasicScenePrefab,
    },
    winit::VirtualKeyCode,
};
use crate::game::Gameplay;
pub mod game;
// use amethyst_core::specs::prelude::Entity;

type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

#[derive(Default)]
struct Main;

impl SimpleState for Main {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        // Initialise the scene with an object, a light and a camera.
        // let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //     loader.load("prefab/sphere.ron", RonFormat, (), ())
        // });
        // world.create_entity().with(handle).build();
        init_output(&mut world.res);
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/main.ron", ());
        });
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                // println!("{}", ui_event.event_type);
                // LogLevelFilter.
                // info!(
                //     "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                //     ui_event
                // );
                if ui_event.event_type == UiEventType::Click {
                    Trans::Switch(Box::new(Gameplay))
                    // Trans::None
                } else {
                    Trans::None
                }
            }
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
                .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
                .level_for("gfx_glyph", amethyst::LogLevelFilter::Error)
                .level_for("ui", amethyst::LogLevelFilter::Info)
                .start();

    // let app_root = application_root_dir();

    let display_config_path = "resources/display.ron";
    let resources = "resources";

    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with(UiEventHandlerSystem::new(), "ui_event_handler", &[])
        .with_bundle(InputBundle::<String, String>::new())?
        .with_basic_renderer(display_config_path, DrawShaded::<PosNormTex>::new(), true)?;
    let mut game = Application::build(resources, Gameplay)?
        // Unlimited FPS
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;
    game.run();
    Ok(())
}

/// This shows how to handle UI events.
pub struct UiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl UiEventHandlerSystem {
    pub fn new() -> Self {
        UiEventHandlerSystem { reader_id: None }
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, mut events: Self::SystemData) {
        let _reader_id = self
            .reader_id
            .get_or_insert_with(|| events.register_reader());

        // Reader id was just initialized above if empty
        // for ev in events.read(reader_id) {
        //     // info!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
        // }
    }
}