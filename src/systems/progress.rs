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
use crate::jump::{Progress, Moveable};

const PROGRESS_DELAY: f64 = 0.4;
const EASE_DURATION: f64 = 0.6;

#[derive(SystemDesc)]
pub struct ProgressSystem;

impl<'s> System<'s> for ProgressSystem {
    type SystemData = (
        Write<'s, Progress>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Moveable>,
    );

    fn run(&mut self, (mut progress, mut transforms, time, cameras, moveables): Self::SystemData) {

        let mut player_y = 0.0;

        for (transform, _moveable) in (&transforms, &moveables).join() {
            player_y = transform.isometry().translation.y;
        }

        let mut camera_y = 0.0;

        for (transform, _camera) in (&transforms, &cameras).join() {
            camera_y = transform.isometry().translation.y;
        }

        if player_y != progress.previous_y {
            progress.previous_y = player_y;
            progress.previous_y_start_time = time.absolute_real_time_seconds();
        } else if
            progress.previous_y_start_time + PROGRESS_DELAY < time.absolute_real_time_seconds() &&
            progress.ease_start_time + EASE_DURATION < time.absolute_real_time_seconds()
        {
            progress.ease_start_time = time.absolute_real_time_seconds();
            progress.ease_start_y = camera_y;
            progress.ease_magnitude_y = player_y - camera_y;
        }

        let ease_end_time = progress.ease_start_time + EASE_DURATION;

        if ease_end_time >= time.absolute_real_time_seconds() {
            let ease_progress = ezing::sine_inout((time.absolute_real_time_seconds() - progress.ease_start_time) / EASE_DURATION);
            let new_camera_y = progress.ease_start_y + progress.ease_magnitude_y * ease_progress as f32;

            for (transform, _camera) in (&mut transforms, &cameras).join() {
                transform.set_translation_y(new_camera_y);
            }
        }
    }
}
