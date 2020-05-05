use amethyst::core::{Transform};
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

const VELOCITY: f32 = 20.0;

use crate::jump::{Moveable};

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Moveable>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, moveables, input, time): Self::SystemData) {
        for (_moveable, transform) in (&moveables, &mut transforms).join() {
            let movement = input.axis_value("horizontal");

            if let Some(horizontal) = movement {
                if horizontal != 0.0 {
                    let magnitude = horizontal * VELOCITY * time.delta_seconds();
                    transform.prepend_translation_x(magnitude);
                }
            }
        }
    }
}
