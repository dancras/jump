use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, ReadStorage, Join, System, SystemData, WriteStorage};

use crate::{
    config::{ArenaConfig},
    jump::{Moveable}
};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Moveable>,
        Read<'s, ArenaConfig>,
    );

    fn run(&mut self, (mut transforms, moveables, config): Self::SystemData) {
        for (transform, _moveable) in (&mut transforms, &moveables).join() {

            let x = transform.isometry().translation.x;

            let tile_x = (transform.isometry().translation.x / 16.0) as u16;
            let tile_y = config.rows - (transform.isometry().translation.y / 16.0).ceil() as u16;

            if x % 16.0 > 10.0 && is_wall_tile(&config, tile_x + 1, tile_y) {
                transform.prepend_translation_x(-(x % 16.0 - 10.0));
            } else if x % 16.0 < 6.0 && is_wall_tile(&config, tile_x - 1, tile_y) {
                transform.prepend_translation_x(6.0 - x % 16.0);
            }

        }
    }
}

fn is_wall_tile(config: &ArenaConfig, x: u16, y: u16) -> bool {
    let i = y * config.cols + x;
    let tile = config.tiles[i as usize];

    tile == 1 || tile == 2
}
