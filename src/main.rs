//! Displays a shaded sphere to the user.

use amethyst::{
    assets::PrefabLoaderSystem,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::prelude::{SystemData, WriteStorage},
    input::{is_close_requested, is_key_down, StringBindings},
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle, UiCreator, UiFinder, UiText},
    utils::{application_root_dir, scene::BasicScenePrefab},
    winit::VirtualKeyCode,
};

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Default)]
struct Example;
impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        world.exec(|mut creator: UiCreator<'_>| {
            // This spawns the appropriate entities for the UI, based on the RON
            creator.create("minimal.ron", ());
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
                log::info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    ui_event
                );
                Trans::None
            }
            StateEvent::Input(input) => {
                log::info!("Input Event detected: {:?}.", input);
                Trans::None
            }
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        // Fetch the resources we need. The 'UiTExt" is literally just a component like anything else, except on UI entities
        // This is how SystemData works basically.
        let (finder, mut ui_text_storage) =
            <(UiFinder<'_>, WriteStorage<'_, UiText>)>::fetch(&mut world.res);

        if let Some(entity) = finder.find("some_text") {
            // Load for a ui entity named 'some_text'
            if let Some(text_data) = ui_text_storage.get_mut(entity) {
                // If it exists, try to fetch its UiText component
                text_data.text = "OH HAI".to_string(); // Change the components text data
            }
        }

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config/display.ron");
    let assets_directory = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_directory, Example::default())?
        // Unlimited FPS
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;
    game.run();
    Ok(())
}
