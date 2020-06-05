use amethyst::{
    prelude::*,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    ecs::Join,
    assets::Handle,
    renderer::{
        SpriteRender, SpriteSheet,
    },
    core::math::{
        UnitQuaternion, Vector3, Translation3, Point3,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::load_sprite_sheet;
use amethyst_tiles::{TileMap, MortonEncoder2D, Map};
use crate::map::{LevelInfo, BlockTile};

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

fn initialise_unit<'a>(world: &'a mut World, type_name: String, pos: Vector3<f32>, team: u32) {
    let units_types = world.fetch::<UnitTyes>().types.clone();
    let unit_type = units_types.iter().find(|&x| x.name == type_name).unwrap();
    
    let sprite_sheet_handle = world.fetch::<SpriteData>().sprite_handles[&unit_type.sprite_name].clone();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: unit_type.sprite_id as usize,
    };

    println!("Spwan {} team {} on {}, {}", &type_name, &team, &pos.x, &pos.y);
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

pub fn spawn_wave_units(world: &mut World, wave: u32) {
    let mut spawn_points = Vec::<Vector3<f32>>::new();
    let spawn_points_len = world.fetch::<LevelInfo>().enemy_spawn.len();

    let storage_tilemap = (&world).write_storage::<TileMap::<BlockTile, MortonEncoder2D>>();
    {
        let tilemap_vec = (storage_tilemap).join().collect::<Vec<_>>();
        {
            let tile_map = tilemap_vec.first().unwrap();
            for i_point in 0..spawn_points_len {
                let point = &world.fetch::<LevelInfo>().enemy_spawn[i_point];
                spawn_points.push(tile_map.to_world(&Point3::new(point[0] as u32, point[1] as u32, 2), None));
            }
        }
    }
    drop(storage_tilemap);

    let mut i: u32 = 0;
    
    let wave_info: HashMap<String, u32> = world.fetch::<LevelInfo>().waves[wave as usize].clone();
    for (type_name, ammount) in wave_info.iter() {
        for _ in 0..ammount.clone() {
            initialise_unit(
                world,
                type_name.to_string(),
                spawn_points[(i % spawn_points_len as u32) as usize],
                1
            );
            i += 1;
        }
    }
}
