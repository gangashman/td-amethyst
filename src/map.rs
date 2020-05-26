use amethyst::{
    core::{
        math::{Point3, Vector3},
        Transform,
    },
    prelude::*,
    renderer::{
        sprite::{SpriteSheet}, Transparent,
    },
    assets::{Handle},
};
use std::fs;
use amethyst_tiles::{MortonEncoder, Tile, TileMap, DrawTiles2D};
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

impl LevelData {
    pub fn get_mut_layer(&mut self, layer_index: u32) -> &mut LayerData {
        let layers_count = self.layers.len();
        for layer in self.layers.iter_mut() {
            if layer.id == (layers_count as u32 - layer_index) as u32 {
                return layer;
            }
        }
        println!("Failed to get mut layer {}", layer_index);
        std::process::exit(1);
    }
    pub fn get_layer(&self, layer_index: u32) -> &LayerData {
        match self.layers.iter().find(|&x| x.id == (self.layers.len() as u32 - layer_index) as u32) {
            Some(e) => e,
            None => {
                println!("Failed to get layer {}", layer_index);
                std::process::exit(1);
            }
        }
    }

    pub fn x_y_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.height + x) as usize
    }

    pub fn get_id_in_point(&self, point: Point3<u32>) -> Option<usize> {
        let id_from_json = self.get_layer(point.z).data[self.x_y_to_index(point.x, point.y)];
        if id_from_json == 0 {
            return None;
        }
        return Some((id_from_json - 1) as usize)
    }

    pub fn change_id_on_point(&mut self, point: Point3<u32>, new_id: u32) {
        let index = LevelData::x_y_to_index(&self, point.x, point.y);
        self.get_mut_layer(point.z).data[index] = new_id;
    }
}

#[derive(Default, Clone)]
pub struct BlockTile;
impl Tile for BlockTile {
    fn sprite(&self, point: Point3<u32>, world: &World) -> Option<usize> {
        world.fetch::<LevelData>().get_id_in_point(point)
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
        .with(Transparent)
        .build();
}
