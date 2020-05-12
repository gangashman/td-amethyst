use amethyst::{
    prelude::*,
    core::transform::{Transform, TransformBundle},
    renderer::{
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    ecs::prelude::{Component, DenseVecStorage},
    assets::{AssetStorage, Loader, Handle},
};

pub const BLOCK_HEIGHT: f32 = 16.0;
pub const BLOCK_WIDTH: f32 = 4.0;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

struct MyState;

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

fn initialise_blocks(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut block_transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };
    
    block_transform.set_translation_xyz(BLOCK_WIDTH * 0.5, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Block::new())
        .with(block_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "hyptosis_sprites.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "hyptosis_sprites.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

impl SimpleState for MyState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        world.register::<Block>();
        
        let sprite_sheet_handle = load_sprite_sheet(world);

        load_sprite_sheet(world);
        initialise_blocks(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, MyState, game_data)?;
    game.run();

    Ok(())
}
