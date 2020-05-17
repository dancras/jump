use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, Join, System, SystemData, WriteStorage};

use crate::{
    config::{ArenaConfig},
    jump::{Moveable}
};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Moveable>,
        Read<'s, ArenaConfig>,
    );

    fn run(&mut self, (mut transforms, mut moveables, config): Self::SystemData) {
        for (transform, moveable) in (&mut transforms, &mut moveables).join() {

            let x = transform.isometry().translation.x;
            let y = transform.isometry().translation.y;

            let tile_x = (transform.isometry().translation.x / 16.0) as u16;
            let tile_y = config.rows - (transform.isometry().translation.y / 16.0).ceil() as u16;

            if x % 16.0 > 10.0 && is_wall_tile(&config, tile_x + 1, tile_y) {
                moveable.velocity_x = 0.0;
                transform.prepend_translation_x(-(x % 16.0 - 10.0));
            } else if x % 16.0 < 6.0 && is_wall_tile(&config, tile_x - 1, tile_y) {
                moveable.velocity_x = 0.0;
                transform.prepend_translation_x(6.0 - x % 16.0);
            }

            if y % 16.0 < 8.0 && is_floor_tile(&config, tile_x, tile_y + 1, x % 16.0) {
                moveable.velocity_y = 0.0;
                transform.prepend_translation_y(8.0 - y % 16.0);
            }

            if moveable.velocity_y > 0.0 &&
                y % 16.0 > 8.0 &&
                is_roof_tile(&config, tile_x, tile_y - 1)
            {
                moveable.velocity_y = -0.001;
                moveable.jump_boost_delay += 1000.0;
                transform.prepend_translation_y(8.0 - y % 16.0);
            }

        }
    }
}

fn is_wall_tile(config: &ArenaConfig, x: u16, y: u16) -> bool {
    let i = y * config.cols + x;
    let tile = config.tiles[i as usize];

    tile == 1 || tile == 2
}

fn is_floor_tile(config: &ArenaConfig, x: u16, y: u16, offset_x: f32) -> bool {
    let i = y * config.cols + x;
    let tile = config.tiles[i as usize];

    tile == 1 || tile == 2 || tile == 3 && offset_x >= 11.0 || tile == 4 && offset_x <= 5.0
}

fn is_roof_tile(config: &ArenaConfig, x: u16, y: u16) -> bool {
    let i = y * config.cols + x;
    let tile = config.tiles[i as usize];

    tile >= 1 && tile <= 4
}
