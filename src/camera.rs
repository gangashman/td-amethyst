use amethyst::{
    prelude::*,
    renderer::Camera,
    utils::application_root_dir,
};
use std::fs::File;
use amethyst_input::VirtualKeyCode;

use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst_window::{DisplayConfig};

pub fn initialise_camera(world: &mut World) {
    use ron::de::from_reader;

    let app_root = application_root_dir().unwrap();

    let display_config_path = app_root.join("config").join("display.ron");
    let display_conf_file = File::open(&display_config_path).expect("Failed opening file");

    let display_config: DisplayConfig = match from_reader(display_conf_file) {
        Ok(e) => {
            // println!("DisplayConfig: {:#?}", e);
            e
        },
        Err(e) => {
            println!("Failed to load display config: {}", e);
            std::process::exit(1);
        }
    };

    let dimensions = display_config.dimensions.unwrap();

    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.0 as f32 * 0.25, dimensions.1 as f32 * 0.25, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.0 as f32 * 0.5, dimensions.1 as f32 * 0.5))
        .with(transform)
        .build();
}

#[derive(SystemDesc)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, cameras, input): Self::SystemData) {
        for (_camera, transform) in (&cameras, &mut transforms).join() {
            let multiplayer = match input.key_is_down(VirtualKeyCode::LShift) {
                true => 2.0,
                false => 1.0,
            };
            let updown = input.axis_value("updown");
            if let Some(mv_updown) = updown {
                if mv_updown != 0.0 {
                    transform.prepend_translation_y(3.0 * mv_updown as f32 * multiplayer);
                }
            }

            let leftright = input.axis_value("leftright");
            if let Some(mv_leftright) = leftright {
                if mv_leftright != 0.0 {
                    transform.prepend_translation_x(3.0 * mv_leftright as f32 * multiplayer);
                }
            }
        }
    }
}
