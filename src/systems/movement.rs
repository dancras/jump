use amethyst::core::{Transform};
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

const INITIAL_VELOCITY: f32 = 20.0;
const ACCELERATION: f32 = 120.0;
const FRICTION: f32 = 100.0;
const GRAVITY: f32 = 98.0;

use crate::jump::{Moveable};

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Moveable>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut moveables, mut transforms, input, time): Self::SystemData) {
        for (moveable, transform) in (&mut moveables, &mut transforms).join() {
            let movement = input.axis_value("horizontal");

            if let Some(horizontal) = movement {
                if horizontal != 0.0 {

                    if moveable.velocity_x == 0.0 {
                        moveable.velocity_x = INITIAL_VELOCITY * horizontal;
                    } else {
                        moveable.velocity_x += horizontal * ACCELERATION * time.delta_seconds();
                    }
                }
            }

            if moveable.velocity_x > 0.0 {
                moveable.velocity_x -= FRICTION * time.delta_seconds();

                if moveable.velocity_x < 0.0 {
                    moveable.velocity_x = 0.0;
                }

            } else if moveable.velocity_x < 0.0 {
                moveable.velocity_x += FRICTION * time.delta_seconds();

                if moveable.velocity_x > 0.0 {
                    moveable.velocity_x = 0.0;
                }
            }

            if moveable.velocity_y == 0.0 && input.action_is_down("jump") == Some(true) {
                moveable.velocity_y += 80.0;
            }

            moveable.velocity_y -= GRAVITY * time.delta_seconds();

            let magnitude_y = moveable.velocity_y * time.delta_seconds();
            transform.prepend_translation_y(magnitude_y);

            let magnitude_x = moveable.velocity_x * time.delta_seconds();
            transform.prepend_translation_x(magnitude_x);

        }
    }
}
