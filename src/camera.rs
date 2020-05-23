use amethyst::{
    prelude::*,
    utils::application_root_dir,
    ecs::{
        Join, Read, ReadStorage, System, WriteStorage, Write,
        Entities, ReadExpect, SystemData,
    },
    core::{
        geometry::Plane, Transform, Named,
        math::{Point2, Vector2, Vector3},
    },
    derive::SystemDesc,
    assets::{AssetStorage},
    renderer::{
        camera::{ActiveCamera, Camera},
        sprite::{SpriteRender, SpriteSheet},
    },
    ui::{UiFinder, UiText},
    window::ScreenDimensions,
};
use std::fs::File;
use amethyst_input::VirtualKeyCode;
use amethyst_tiles::{MortonEncoder, TileMap, Map};
use crate::map::{BlockTile, LevelData};
use winit::MouseButton;

use amethyst::input::{InputHandler, StringBindings};
use amethyst_window::{DisplayConfig};


pub fn initialise_camera(world: &mut World) {
    use ron::de::from_reader;

    let app_root = application_root_dir().unwrap();

    let display_config_path = app_root.join("config").join("display.ron");
    let display_conf_file = File::open(&display_config_path).expect("Failed opening file");

    let display_config: DisplayConfig = match from_reader(display_conf_file) {
        Ok(e) => e,
        Err(e) => {
            println!("Failed to load display config: {}", e);
            std::process::exit(1);
        }
    };

    let dimensions = display_config.dimensions.unwrap();

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

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
                camera_transform.prepend_translation_x(x_move * 20.0 * multiplayer);
                camera_transform.prepend_translation_y(y_move * 20.0 * multiplayer);

                let z_scale = 0.3 * scrool;
                let scale = camera_transform.scale();
                let scale = Vector3::new(scale.x + z_scale, scale.y + z_scale, scale.z + z_scale);
                camera_transform.set_scale(scale);
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct MouseRaycastSystem;

impl<'s> System<'s> for MouseRaycastSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, Named>,
        WriteStorage<'s, UiText>,
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
        UiFinder<'s>,
        WriteStorage<'s, TileMap<BlockTile, MortonEncoder>>,
        Write<'s, LevelData>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            cameras,
            sprites,
            names,
            mut ui_texts,
            sprite_sheets,
            screen_dimensions,
            active_camera,
            input,
            ui_finder,
            mut tilemaps,
            mut level_data,
        ): Self::SystemData,
    ) {
        // Get the mouse position if its available
        if let Some(mouse_position) = input.mouse_position() {
            // Get the active camera if it is spawned and ready
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                // Project a ray from the camera to the 0z axis
                let ray = camera.projection().screen_ray(
                    Point2::new(mouse_position.0, mouse_position.1),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = ray.at_distance(distance);

                // TileMap click
                if input.mouse_button_is_down(MouseButton::Left) {
                    for tilemap in (&mut tilemaps).join() {
                        let pos = Vector3::new(mouse_world_position.x, mouse_world_position.y, 0.0);
                        match tilemap.to_tile(&pos, None) {
                            Ok(p) => {
                                println!("{}", p);
                                // let mut point = tilemap.get_mut(&p);
                                // tilemap.change_id_on_point(p, world);
                                level_data.change_id_on_point(p, 616);
                            },
                            Err(_e) => (),
                        }
                    }
                }

                // Find any sprites which the mouse is currently inside
                let mut found_name = None;
                for (sprite, transform, name) in (&sprites, &transforms, &names).join() {
                    let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                    let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                    let (min_x, max_x, min_y, max_y) = {
                        // Sprites are centered on a coordinate, so we build out a bbox for the sprite coordinate
                        // and dimensions
                        // Notice we ignore z-axis for this example.
                        (
                            transform.translation().x - (sprite.width * 0.5),
                            transform.translation().x + (sprite.width * 0.5),
                            transform.translation().y - (sprite.height * 0.5),
                            transform.translation().y + (sprite.height * 0.5),
                        )
                    };
                    if mouse_world_position.x > min_x
                        && mouse_world_position.x < max_x
                        && mouse_world_position.y > min_y
                        && mouse_world_position.y < max_y
                    {
                        found_name = Some(&name.name);
                    }
                }

                if let Some(t) = ui_finder
                    .find("under_mouse")
                    .and_then(|e| ui_texts.get_mut(e))
                {
                    if let Some(name) = found_name {
                        t.text = format!("{}", name);
                    } else {
                        t.text = "".to_string();
                    }
                }
            }
        }
    }
}
