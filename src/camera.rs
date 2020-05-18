use amethyst::{
    prelude::*,
    core::transform::{Transform},
    renderer::{
        Camera,
    },
};
use crate::utils::get_world_config;

pub fn initialise_camera(world: &mut World) {

    let world_config = get_world_config();
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    transform.set_translation_xyz(world_config.size.0 * 0.5, world_config.size.1 * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(world_config.size.0, world_config.size.1))
        .with(transform)
        .build();
}
