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
use amethyst_tiles::{MortonEncoder, Tile, TileMap};

#[derive(Default, Clone)]
pub struct BlockTile;
impl Tile for BlockTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        Some(1)
    }
}

pub fn initialise_map(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let map = TileMap::<BlockTile, MortonEncoder>::new(
        Vector3::new(20, 20, 1),
        Vector3::new(32, 32, 1),
        Some(sprite_sheet_handle),
    );

    let _map_entity = world
        .create_entity()
        .with(map)
        .with(Transform::default())
        .build();
}
