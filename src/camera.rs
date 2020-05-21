use amethyst::{
    prelude::*,
    utils::application_root_dir,
    ecs::{
        Join, Read, ReadStorage, System, WriteStorage, Entities,
    },
    core::{
        math::Vector3,
        Transform,
    },
    renderer::{
        camera::{ActiveCamera, Camera},
    },
};
use std::fs::File;
use amethyst_input::VirtualKeyCode;

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

#[derive(Default)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        Read<'s, ActiveCamera>,
        Entities<'s>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (active_camera, entities, cameras, mut transforms, input): Self::SystemData) {
        let multiplayer = match input.key_is_down(VirtualKeyCode::LShift) {
            true => 2.0,
            false => 1.0,
        };
        let y_move = input.axis_value("updown").unwrap();
        let x_move = input.axis_value("leftright").unwrap();
        let scrool = input.axis_value("scrool").unwrap();

        if x_move != 0.0 || y_move != 0.0 || scrool != 0.0 {
            let mut camera_join = (&cameras, &mut transforms).join();
            if let Some((_, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                camera_transform.prepend_translation_x(x_move * 2.0 * multiplayer);
                camera_transform.prepend_translation_y(y_move * 2.0 * multiplayer);

                let z_scale = 0.04 * scrool;
                let scale = camera_transform.scale();
                let scale = Vector3::new(scale.x + z_scale, scale.y + z_scale, scale.z + z_scale);
                camera_transform.set_scale(scale);
            }
        }
    }
}
