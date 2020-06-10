use amethyst::{
    prelude::*,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    assets::Handle,
    renderer::{
        SpriteRender, SpriteSheet,
    },
    core::math::{
        UnitQuaternion, Vector3, Translation3,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::{load_sprite_sheet, get_world_spawn_points};
use crate::map::{LevelInfo};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct UnitType {
    pub name: String,
    pub sprite_name: String,
    pub sprite_id: u32,
    pub attack: f32,
    pub max_health: f32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct UnitTyes {
    pub types: Vec<UnitType>,
}

#[derive(Default)]
pub struct SpriteData {
    pub sprite_handles: HashMap<String, Handle<SpriteSheet>>,
}

#[derive(Default)]
pub struct Unit {
    pub team: u32,
    pub unit_type: UnitType,
    pub health: f32,
}

impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_unit<'a>(world: &'a mut World, type_name: &String, pos: Vector3<f32>, team: u32) {
    let units_types = world.fetch::<UnitTyes>().types.clone();
    let unit_type = units_types.iter().find(|&x| &x.name == type_name).unwrap();
    
    let sprite_sheet_handle = world.fetch::<SpriteData>().sprite_handles[&unit_type.sprite_name].clone();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: unit_type.sprite_id as usize,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Unit{
            team: team,
            unit_type: unit_type.clone(),
            health: unit_type.max_health
        })
        .with(Transform::new(
            Translation3::new(pos.x as f32, pos.y as f32, 0.0),
            UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0)
        ))
        .build();
}

pub fn load_unit_info(world: &mut World) {
    world.register::<Unit>();

    let mut sprite_data = SpriteData::default();

    sprite_data.sprite_handles.insert(
        "hyptosis_sprites".to_string(),
        load_sprite_sheet(world, "images/hyptosis_sprites.png", "images/hyptosis_sprites.ron")
    );
    world.insert::<SpriteData>(sprite_data);
}

pub fn spawn_unit(world: &mut World, wave: u32, index: u32) {
    let spawn_points = get_world_spawn_points(world);
    let spawn_points_len = world.fetch::<LevelInfo>().enemy_spawn.len();
    let pos = spawn_points[(index % spawn_points_len as u32) as usize];

    let unit_name = match world.fetch::<LevelInfo>().get_unit_by_index(wave, index) {
        Some(e) => e,
        None => {
            println!("get_unit_by_index return none. wave: {} index: {}", wave, index);
            std::process::exit(1);
        }
    };
    initialise_unit(world, &unit_name, pos, 1);
    println!("{}) Spwan {} in {}, {}", &index + 1, &unit_name, &pos.x, &pos.y);
}
