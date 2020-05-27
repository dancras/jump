use amethyst::{
    core::{
        timing::Time,
        Transform
    },
    derive::SystemDesc,
    ecs::{
        Read,
        Join,
        System,
        SystemData,
        WriteStorage
    }
};

use crate::{
    config::{ArenaConfig},
    jump::{Moveable}
};

const EVANESCENT_DURATION: f64 = 1.5;

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Moveable>,
        Read<'s, ArenaConfig>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut moveables, config, time): Self::SystemData) {
        for (transform, moveable) in (&mut transforms, &mut moveables).join() {

            let x = transform.isometry().translation.x;
            let y = transform.isometry().translation.y;

            if y <= 96.0 || moveable.evanescent_start_time + EVANESCENT_DURATION <= time.absolute_real_time_seconds() {
                moveable.evanescent = false;
            }

            if moveable.evanescent {
                continue;
            }

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

            if is_touching_spike(&config, x, y) {
                moveable.velocity_x = 0.0;
                moveable.velocity_y = -0.001;
                moveable.jump_boost_delay += 1000.0;

                moveable.evanescent = true;
                moveable.evanescent_start_time = time.absolute_real_time_seconds();
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

fn get_collision_info(config: &ArenaConfig, x: f32, y: f32) -> Vec<(u8, f32, f32)> {
    let mut collision_info: Vec<(u8, f32, f32)> = Vec::new();

    let collision_points = [
        (x - 2.5, y + 6.5),
        (x + 2.5, y + 6.5),
        (x - 5.5, y + 2.5),
        (x + 5.5, y + 2.5),
        (x - 4.5, y - 4.5),
        (x + 4.5, y - 4.5),
        (x - 2.5, y - 7.5),
        (x + 2.5, y - 7.5),
    ];

    for (collision_x, collision_y) in collision_points.iter() {
        let tile_x = (collision_x / config.tile_size) as u16;
        let tile_y = config.rows - (collision_y / config.tile_size).ceil() as u16;

        let offset_x = collision_x % config.tile_size;
        let offset_y = collision_y % config.tile_size;

        collision_info.push(
            (config.tile(tile_x, tile_y), offset_x, offset_y)
        );
    }

    collision_info
}

fn is_touching_spike(config: &ArenaConfig, x: f32, y: f32) -> bool {

    let collision_info = get_collision_info(config, x, y);

    for (tile, offset_x, offset_y) in collision_info {

        if tile == 12 && offset_y <= 4.0 ||
            tile == 13 && offset_y >= 12.0 ||
            tile == 14 && offset_x <= 4.0 ||
            tile == 15 && offset_x >= 12.0
        {
            return true;
        }

    }

    false
}

fn is_roof_tile(config: &ArenaConfig, x: u16, y: u16) -> bool {
    let i = y * config.cols + x;
    let tile = config.tiles[i as usize];

    tile >= 1 && tile <= 4
}
