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

        world.insert(Progress::default());

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Tile>();
        initialise_grid(world, sprite_sheet_handle.clone(), &self.config.arena);

        initialise_camera(world, &self.config.arena);

        initialise_player(world, sprite_sheet_handle);
    }

}

#[derive(Default)]
pub struct Progress {
    pub previous_y: f32,
    pub previous_y_start_time: f64,
    pub is_easing: bool,
    pub ease_start_y: f32,
    pub ease_start_time: f64,
    pub ease_end_y: f32,
    pub ease_magnitude_y: f32,
}

fn initialise_camera(world: &mut World, config: &ArenaConfig) {

    let arena_width = config.arena_width();

    let mut transform = Transform::default();
    transform.set_translation_xyz(arena_width * 0.5, arena_width * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(arena_width, arena_width))
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
            width: tile_size * 2.0,
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
    world.insert((*config).clone());

    for (i, tile_code) in config.tiles.iter().enumerate() {

        let tile_code = *tile_code as usize;

        if tile_code == 0 {
            continue;
        }

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: tile_code - 1
        };

        let i = i as u16;
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

#[derive(Default)]
pub struct Moveable {
    pub jump_boost_delay: f64,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub evanescent: bool,
    pub evanescent_start_time: f64,
}

impl Component for Moveable {
    type Storage = DenseVecStorage<Self>;
}

pub struct Animated;

impl Component for Animated {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Facing {
    pub left: bool,
    pub previous_x: f32
}

impl Component for Facing {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_player(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 3
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(72.0, 56.0, 0.1);

    world.create_entity()
        .with(Moveable::default())
        .with(Animated)
        .with(Facing::default())
        .with(sprite_render)
        .with(transform)
        .build();
}
