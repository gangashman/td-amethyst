use amethyst::{
    prelude::*,
    renderer::{
        ImageFormat, SpriteSheet, SpriteSheetFormat, Texture,
    },
    ecs::Join,
    assets::{AssetStorage, Loader, Handle},
    core::math::{
        Vector3, Point3,
    },
};
use std::fs;
use serde::de::DeserializeOwned;
use crate::map::{LevelInfo, BlockTile};
use amethyst_tiles::{TileMap, MortonEncoder2D, Map};

pub fn load_sprite_sheet(world: &mut World, image_path: &str, ron_path: &str) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            image_path,
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn load_json_data<T: DeserializeOwned>(json_path: &str) -> T {
    let json_string = match fs::read_to_string(&json_path) {
        Ok(e) => e,
        Err(e) => {
            println!("Failed to load map json {}: {}", json_path, e);
            std::process::exit(1);
        }
    };
    match serde_json::from_str::<T>(json_string.as_str()) {
        Ok(e) => e,
        Err(e) => {
            println!("Failed to parse map json {}: {}", json_path, e);
            std::process::exit(1);
        }
    }
}

pub fn get_world_spawn_points(world: &mut World) -> Vec::<Vector3<f32>> {
    let mut spawn_points = Vec::<Vector3<f32>>::new();
    let spawn_points_len = world.fetch::<LevelInfo>().enemy_spawn.len();

    let storage_tilemap = world.write_storage::<TileMap::<BlockTile, MortonEncoder2D>>();
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
    spawn_points
}
