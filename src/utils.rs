use amethyst::{
    prelude::*,
    renderer::{
        ImageFormat, SpriteSheet, SpriteSheetFormat, Texture,
    },
    assets::{AssetStorage, Loader, Handle},
};
use std::fs;
use serde::de::DeserializeOwned;

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
