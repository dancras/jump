use amethyst::{
    core::{
        timing::Time,
        Transform
    },
    derive::SystemDesc,
    ecs::{Read, ReadStorage, Join, System, SystemData, Write, WriteStorage},
    renderer::{Camera},
};
use ezing;
use crate::{
    config::{ArenaConfig},
    jump::{Progress, Moveable}
};

const PROGRESS_DELAY: f64 = 0.4;
const EASE_DURATION: f64 = 0.6;

#[derive(SystemDesc)]
pub struct ProgressSystem;

impl<'s> System<'s> for ProgressSystem {
    type SystemData = (
        Write<'s, Progress>,
        WriteStorage<'s, Transform>,
        Read<'s, ArenaConfig>,
        Read<'s, Time>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Moveable>,
    );

    fn run(&mut self, (mut progress, mut transforms, arena_config, time, cameras, moveables): Self::SystemData) {

        let mut player_y = 0.0;

        for (transform, _moveable) in (&transforms, &moveables).join() {
            player_y = transform.isometry().translation.y;
        }

        let mut target_y = player_y;

        if target_y < arena_config.arena_width() * 0.5 {
            target_y = arena_config.arena_width() * 0.5;
        }

        let mut camera_y = 0.0;

        for (transform, _camera) in (&transforms, &cameras).join() {
            camera_y = transform.isometry().translation.y;
        }

        if target_y != progress.previous_y {
            progress.previous_y = target_y;
            progress.previous_y_start_time = time.absolute_real_time_seconds();
        } else if
            target_y != camera_y &&
            !(progress.is_easing && target_y == progress.ease_end_y) &&
            progress.previous_y_start_time + PROGRESS_DELAY < time.absolute_real_time_seconds() &&
            progress.ease_start_time + EASE_DURATION < time.absolute_real_time_seconds()
        {
            progress.is_easing = true;
            progress.ease_start_time = time.absolute_real_time_seconds();
            progress.ease_start_y = camera_y;
            progress.ease_end_y = target_y;
            progress.ease_magnitude_y = target_y - camera_y;
        }

        let ease_end_time = progress.ease_start_time + EASE_DURATION;

        if progress.is_easing {

            let mut ease_progress = ezing::sine_inout((time.absolute_real_time_seconds() - progress.ease_start_time) / EASE_DURATION);

            if ease_end_time <= time.absolute_real_time_seconds() {
                ease_progress = 1.0;
                progress.is_easing = false;
            }
            let new_camera_y = progress.ease_start_y + progress.ease_magnitude_y * ease_progress as f32;

            for (transform, _camera) in (&mut transforms, &cameras).join() {
                transform.set_translation_y(new_camera_y);
            }
        }
    }
}
