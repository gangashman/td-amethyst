use amethyst::{
    prelude::*,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    assets::Handle,
    renderer::{
        SpriteRender, SpriteSheet,
    },
};
use std::collections::HashMap;
use crate::utils::load_sprite_sheet;

#[derive(Default)]
pub struct Unit {
    pub team: u32,
}

impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_unit(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, sprite_number: usize) {
    let mut block_transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: sprite_number,
    };
    
    block_transform.set_translation_xyz(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Unit::default())
        .with(block_transform)
        .build();
}

#[derive(Default)]
pub struct SpriteData {
    pub sprite_handles: HashMap<String, Handle<SpriteSheet>>,
}

pub fn load_unit_info(world: &mut World) {
    world.register::<Unit>();

    let mut sprite_data = SpriteData::default();

    sprite_data.sprite_handles.insert(
        "hyptosis_sprites".to_string(),
        load_sprite_sheet(world, "images/hyptosis_sprites.png", "images/hyptosis_sprites.ron")
    );
}
