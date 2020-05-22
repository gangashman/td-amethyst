use amethyst::{
    core::{
        math::{Point3, Vector3},
        Transform,
    },
    prelude::*,
    renderer::{
        sprite::{SpriteSheet},
    },
    assets::{Handle},
};
use std::fs;
use amethyst_tiles::{MortonEncoder, Tile, TileMap};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct LayerData {
    data: Vec<u32>,
    id: u32,
    name: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct LevelData {
    pub layers: Vec<LayerData>,
    
    pub height: u32,
    pub width: u32,

    pub tileheight: u32,
    pub tilewidth: u32,
}

#[derive(Default, Clone)]
pub struct BlockTile;
impl Tile for BlockTile {
    fn sprite(&self, point: Point3<u32>, world: &World) -> Option<usize> {
        match world.fetch::<LevelData>().layers.iter().find(|&x| x.id == point.z + 1) {
            Some(e) => {
                let index: usize = (point.y * world.fetch::<LevelData>().height + point.x) as usize;
                Some((e.data[index]) as usize)
            },
            None => None
        }
    }
}

pub fn initialise_map(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, json_path: &str) {
    let json_string = match fs::read_to_string(json_path) {
        Ok(e) => e,
        Err(e) => {
            println!("Failed to load level json {}: {}", json_path, e);
            std::process::exit(1);
        }
    };
    let _level_data = match serde_json::from_str::<LevelData>(&json_string[..]) {
        Ok(e) => e,
        Err(e) => {
            println!("Failed to parse json {}: {}", json_path, e);
            std::process::exit(1);
        }
    };

    world.insert::<LevelData>(_level_data);

    let map = TileMap::<BlockTile, MortonEncoder>::new(
        Vector3::new(world.fetch::<LevelData>().height, world.fetch::<LevelData>().width, world.fetch::<LevelData>().layers.len() as u32),
        Vector3::new(world.fetch::<LevelData>().tileheight, world.fetch::<LevelData>().tilewidth, 1),
        Some(sprite_sheet_handle),
    );

    let _map_entity = world
        .create_entity()
        .with(map)
        .with(Transform::default())
        .build();
}
