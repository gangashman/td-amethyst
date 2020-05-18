use amethyst::{
    prelude::*,
    renderer::{
        ImageFormat, SpriteSheet, SpriteSheetFormat, Texture,
    },
    utils::application_root_dir,
    assets::{AssetStorage, Loader, Handle},
};

use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    pub size: (f32, f32),
}

pub fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "images/hyptosis_sprites.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "images/hyptosis_sprites.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

pub fn get_world_config() -> WorldConfig {
    use ron::de::from_reader;

    let app_root = application_root_dir().unwrap();

    let world_config_path = app_root.join("config").join("world.ron");
    let world_conf_file = File::open(&world_config_path).expect("Failed opening file");

    let world_config: WorldConfig = match from_reader(world_conf_file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load world config: {}", e);

            std::process::exit(1);
        }
    };
    world_config
}
