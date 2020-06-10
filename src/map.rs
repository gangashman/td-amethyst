use amethyst::{
    core::{
        math::{Point3, Vector3},
        Transform,
    },
    prelude::{World, WorldExt, Builder},
    renderer::{
        sprite::SpriteSheet, Transparent,
    },
    assets::Handle,
};
use amethyst_tiles::{MortonEncoder2D, Tile, TileMap};
use serde::{Deserialize, Serialize};
use amethyst_rendy::palette::Srgba;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Default, Serialize, Deserialize)]
pub struct LayerData {
    data: Vec<u32>,
    id: u32,
    name: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct MapData {
    pub layers: Vec<LayerData>,
    
    pub height: u32,
    pub width: u32,

    pub tileheight: u32,
    pub tilewidth: u32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct LevelInfo {
    pub enemy_spawn: Vec<Vec<u32>>,
    pub spawn_speed: u32,
    pub levels: Vec<HashMap<String, u32>>,
}

impl LevelInfo {
    pub fn get_units_count(&self, wave: u32) -> u32 {
        let mut count: u32 = 0;
        for (_key, value) in &self.levels[wave as usize] {
            count += value;
        }
        count
    }

    pub fn get_unit_by_index(&self, wave: u32, index: u32) -> Option<String> {
        let units_count = self.get_units_count(wave);

        let mut wave_units = self.levels[wave as usize].clone();
        let units_types = Vec::from_iter(self.levels[wave as usize].keys());

        for i in 0..units_count {
            let key = units_types[(i % units_types.len() as u32) as usize];
            if i == index {
                return Some(key.to_string());
            }

            *wave_units.get_mut(key).unwrap() += 1;

            if wave_units[key] <= 0 {
                wave_units.remove(key);
            }
        }
        None
    }
}

impl MapData {

    pub fn x_y_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.height + x) as usize
    }

    pub fn get_id_in_point(&self, point: Point3<u32>) -> Option<usize> {
        let id_from_json = self.layers[point.z as usize].data[self.x_y_to_index(point.x, point.y)];
        match id_from_json {
            0 => None,
            _ => Some((id_from_json - 1) as usize),
        }
    }

    pub fn change_id_on_point(&mut self, point: Point3<u32>, new_id: u32) {
        let index = MapData::x_y_to_index(&self, point.x, point.y);
        self.layers[point.z as usize].data[index] = new_id;
    }
}

#[derive(Default, Clone)]
pub struct BlockTile;
impl Tile for BlockTile {
    fn sprite(&self, point: Point3<u32>, world: &World) -> Option<usize> {
        return world.fetch::<MapData>().get_id_in_point(point)
    }

    fn tint(&self, _point: Point3<u32>, _world: &World) -> Srgba {
        Srgba::new(1.0, 1.0, 1.0, 1.0)
    }
}

pub fn initialise_map(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let layer_size = world.fetch::<MapData>().layers.len() as u32;
    let map = TileMap::<BlockTile, MortonEncoder2D>::new(
        Vector3::new(world.fetch::<MapData>().height, world.fetch::<MapData>().width, layer_size),
        Vector3::new(world.fetch::<MapData>().tileheight, world.fetch::<MapData>().tilewidth, 1),
        Some(sprite_sheet_handle),
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, layer_size as f32 * -1.0);

    let _map_entity = world
        .create_entity()
        .with(map)
        .with(transform)
        .with(Transparent)
        .build();
}
