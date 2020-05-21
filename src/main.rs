mod utils;
mod camera;

use amethyst::{
    prelude::*,
    core::transform::{Transform, TransformBundle},
    renderer::{
        SpriteRender, SpriteSheet,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    ecs::prelude::{Component, DenseVecStorage},
    assets::{Handle},
    input::{InputBundle, StringBindings},
};
use nalgebra::base::Vector3;
use utils::load_sprite_sheet;
use camera::{initialise_camera, CameraSystem};
use amethyst_tiles::{DrawTiles2D, TileMap, Tile, CoordinateEncoder};


pub const BLOCK_HEIGHT: f32 = 16.0;
pub const BLOCK_WIDTH: f32 = 4.0;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const BLOCK_SIZE: i32 = 32;


pub struct Block {
    pub width: f32,
    pub height: f32,
}

impl Block {
    fn new() -> Block {
        Block {
            width: BLOCK_WIDTH,
            height: BLOCK_HEIGHT,
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_blocks(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, sprite_number: usize) {
    let mut block_transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: sprite_number,
    };
    
    block_transform.set_translation_xyz(BLOCK_WIDTH * 0.5, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Block::new())
        .with(block_transform)
        .build();
}

fn initialise_map<T: Tile, E: CoordinateEncoder>(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let tilemap = TileMap::<T, E>::new(Vector3::new(32, 32, 1), Vector3::new(32, 32, 1), Some(sprite_sheet_handle));
}

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        world.register::<Block>();
        
        let sprite_sheet_handle = load_sprite_sheet(
            world, "images/hyptosis_sprites.png", "images/hyptosis_sprites.ron"
        );

        initialise_blocks(world, sprite_sheet_handle, 0);
        initialise_camera(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;
    
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?

        .with_bundle(input_bundle)?
        .with(CameraSystem, "camera_system", &["input_system"])
        ;

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
