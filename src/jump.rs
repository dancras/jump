use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::config::{JumpConfig, ArenaConfig};

pub struct Jump {
    pub config: JumpConfig
}

impl SimpleState for Jump {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Tile>();
        initialise_grid(world, sprite_sheet_handle, &self.config.arena);

        initialise_camera(world, &self.config.arena);
    }

}

fn initialise_camera(world: &mut World, config: &ArenaConfig) {

    let arena_width = config.cols as f32 * config.tile_size;
    let arena_height = config.rows as f32 * config.tile_size;

    let mut transform = Transform::default();
    transform.set_translation_xyz(arena_width * 0.5, arena_height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(arena_width, arena_height))
        .with(transform)
        .build();
}

pub struct Tile {
    pub width: f32,
    pub height: f32,
}

impl Tile {
    fn new(tile_size: f32) -> Self {
        Self {
            width: tile_size,
            height: tile_size,
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_grid(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    config: &ArenaConfig
) {
    for (i, tile_code) in config.tiles.iter().enumerate() {

        let tile_code = *tile_code as usize;

        if tile_code == 0 {
            continue;
        }

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: tile_code - 1
        };

        let i = i as u8;
        let mut transform = Transform::default();

        let half_tile = config.tile_size / 2.0;

        let y = (config.rows - (i / 9)) as f32 * config.tile_size - half_tile;
        let x = (i % 9) as f32 * config.tile_size + half_tile;

        transform.set_translation_xyz(x, y, 0.0);

        world
            .create_entity()
            .with(sprite_render)
            .with(Tile::new(config.tile_size))
            .with(transform)
            .build();

    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/jump_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/jump_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
