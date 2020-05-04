use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_ROWS: u8 = 4;
pub const ARENA_COLS: u8 = 9;
pub const TILE_SIZE: f32 = 16.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Tile>();
        initialise_grid(world, sprite_sheet_handle);

        initialise_camera(world);
    }

}

fn initialise_camera(world: &mut World) {

    let arena_width = ARENA_COLS as f32 * TILE_SIZE;
    let arena_height = ARENA_ROWS as f32 * TILE_SIZE;

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
    fn new() -> Self {
        Self {
            width: TILE_SIZE,
            height: TILE_SIZE,
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_grid(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let w = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let f = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1,
    };

    let s = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 2,
    };

    let map = [
        &w, &s, &s, &s, &s, &s, &s, &s, &w,
        &w, &f, &s, &s, &s, &s, &s, &f, &w,
        &w, &s, &s, &s, &s, &s, &s, &s, &w,
        &w, &f, &f, &f, &f, &f, &f, &f, &w,
    ];

    for (i, sprite_render) in map.iter().enumerate() {

        let i = i as u8;
        let mut transform = Transform::default();

        let half_tile = TILE_SIZE / 2.0;

        let y = (ARENA_ROWS - (i / 9)) as f32 * TILE_SIZE - half_tile;
        let x = (i % 9) as f32 * TILE_SIZE + half_tile;

        transform.set_translation_xyz(x, y, 0.0);

        world
            .create_entity()
            .with((*sprite_render).clone())
            .with(Tile::new())
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
