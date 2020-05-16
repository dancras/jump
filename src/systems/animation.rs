use amethyst::{
    derive::SystemDesc,
    ecs::{ReadStorage, Join, System, SystemData, WriteStorage},
    renderer::{SpriteRender},
};

use crate::jump::{Animated, Moveable};

#[derive(SystemDesc)]
pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Moveable>,
        ReadStorage<'s, Animated>,
    );

    fn run(&mut self, (mut sprite_renders, moveables, animateds): Self::SystemData) {
        for (sprite_render, moveable, _animated) in (&mut sprite_renders, &moveables, &animateds).join() {
            sprite_render.sprite_number =
                if moveable.velocity_y != 0.0 {
                    9
                } else if moveable.velocity_x != 0.0 {
                    7
                } else {
                    5
                };
        }
    }
}
