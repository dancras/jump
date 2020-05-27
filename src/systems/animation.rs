use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Read, ReadStorage, Join, System, SystemData, WriteStorage},
    renderer::{
        palette::Srgba,
        resources::Tint,
        SpriteRender
    },
};
use ezing;

use crate::jump::{Animated, Moveable};

#[derive(SystemDesc)]
pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s, Moveable>,
        ReadStorage<'s, Animated>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut tints, moveables, animateds, time): Self::SystemData) {
        for (sprite_render, tint, moveable, _animated) in (&mut sprite_renders, &mut tints, &moveables, &animateds).join() {
            sprite_render.sprite_number =
                if moveable.velocity_y != 0.0 {
                    9
                } else if moveable.velocity_x != 0.0 {
                    7
                } else {
                    5
                };

            if moveable.evanescent {
                let time_passed = time.absolute_real_time_seconds() - moveable.evanescent_start_time;
                let progress = (time_passed / 0.5) as f32;
                let eased_progress = ezing::sine_inout(progress);

                tint.0 = Srgba::new(1.0, 1.0, 1.0, eased_progress);
            } else {
                tint.0 = Srgba::new(1.0, 1.0, 1.0, 1.0);
            }
        }
    }
}
