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

        let mut delta = time.delta_seconds();

        if delta > 0.02 {
            delta = 0.02;
        }

        for (moveable, transform) in (&mut moveables, &mut transforms).join() {
            let movement = input.axis_value("horizontal");

            if let Some(horizontal) = movement {
                if horizontal != 0.0 {

                    if moveable.velocity_x == 0.0 {
                        moveable.velocity_x = INITIAL_VELOCITY * horizontal;
                    } else {
                        moveable.velocity_x += horizontal * ACCELERATION * delta;
                    }
                }
            }

            if moveable.velocity_x > 0.0 {
                moveable.velocity_x -= FRICTION * delta;

                if moveable.velocity_x < 0.0 {
                    moveable.velocity_x = 0.0;
                }

            } else if moveable.velocity_x < 0.0 {
                moveable.velocity_x += FRICTION * delta;

                if moveable.velocity_x > 0.0 {
                    moveable.velocity_x = 0.0;
                }
            }

            if input.action_is_down("jump") == Some(true) {
                if moveable.velocity_y == 0.0 {
                    moveable.velocity_y += 40.0;
                    moveable.jump_boost_delay = time.absolute_real_time_seconds() + 0.05;
                } else if moveable.jump_boost_delay < time.absolute_real_time_seconds() && time.absolute_real_time_seconds() < moveable.jump_boost_delay + 0.05  {
                    moveable.velocity_y += 20.0;

                    moveable.jump_boost_delay =
                        if moveable.velocity_y > 60.0 {
                            time.absolute_real_time_seconds() + 1000.0
                        } else {
                            time.absolute_real_time_seconds() + 0.05
                        }
                }
            }

            moveable.velocity_y -= GRAVITY * delta;

            let magnitude_y = moveable.velocity_y * delta;
            transform.prepend_translation_y(magnitude_y);

            let magnitude_x = moveable.velocity_x * delta;
            transform.prepend_translation_x(magnitude_x);

        }
    }
}
