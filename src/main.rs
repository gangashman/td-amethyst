mod utils;
mod camera;
mod map;
mod unit;
mod states;

use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    ecs::prelude::Entity,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    utils::application_root_dir,
    assets::ProgressCounter,
    ui::{RenderUi, UiBundle, UiCreator, UiEventType, UiFinder, UiText},
    winit::VirtualKeyCode,
};
use amethyst_tiles::{MortonEncoder2D, RenderTiles2D};
use utils::{load_sprite_sheet, load_json_data};
use camera::{initialise_camera, CameraSystem, MouseRaycastSystem};
use map::{initialise_map, BlockTile, LevelInfo, MapData};
use unit::load_unit_info;
use log::info;
use states::play::PlayState;

pub struct GameState {
    pub progress_counter: Option<ProgressCounter>,
    pub game_speed: f32,
}

impl GameState {
    fn new(game_speed: f32) -> GameState {
        GameState {
            progress_counter: Option::default(),
            game_speed: game_speed,
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        self.progress_counter = Some(Default::default());

        load_unit_info(world);

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create(
                "ui/main.ron", self.progress_counter.as_mut().unwrap(),
            );
        });
        
        initialise_camera(world);

        let batch_1_sprite_sheet_handle = load_sprite_sheet(
            world, "images/hyptosis_tile-art-batch-1.png", "images/hyptosis_tile-art-batch-1.ron"
        );
        
        world.insert::<MapData>(load_json_data::<MapData>("assets/levels/1_40_40.json"));
        world.insert::<LevelInfo>(load_json_data::<LevelInfo>("assets/levels/1_info.json"));
        
        initialise_map(world, batch_1_sprite_sheet_handle);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
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
                // TODO: remove this
                let mut top_center_entity: Option<Entity> = None;

                data.world.exec(|ui_finder: UiFinder<'_>| {
                    top_center_entity = ui_finder.find("top-center");
                });

                if ui_event.event_type == UiEventType::Click && ui_event.target == top_center_entity.unwrap() {
                    let mut ui_text = data.world.write_storage::<UiText>();
                    let mut top_center_text = ui_text.get_mut(top_center_entity.unwrap()).unwrap();

                    top_center_text.text = String::from("pause game");

                    return Trans::Push(Box::new(PlayState));
                }
                Trans::None
            }
            StateEvent::Input(_input) => {
                // info!("Input Event detected: {:?}.", _input);
                Trans::None
            }
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.05, 0.05, 0.05, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderTiles2D::<BlockTile, MortonEncoder2D>::default()),
        )?
        .with_bundle(TransformBundle::new())?

        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?

        .with(CameraSystem, "camera_system", &["input_system"])
        .with(MouseRaycastSystem, "mouse_raycast_system", &["input_system"])
        ;

    let mut game = Application::new(assets_dir, GameState::new(1.0), game_data)?;
    game.run();

    Ok(())
}
