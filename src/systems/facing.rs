use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{ReadStorage, Join, System, SystemData, WriteStorage};
use amethyst::renderer::{SpriteRender};

use crate::jump::{Facing};

#[derive(SystemDesc)]
pub struct FacingSystem;

impl<'s> System<'s> for FacingSystem {
    type SystemData = (
        WriteStorage<'s, Facing>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut facings, mut sprite_renders, transforms): Self::SystemData) {
        for (facing, sprite_render, transform) in (&mut facings, &mut sprite_renders, &transforms).join() {

            let new_x = transform.isometry().translation.x;

            if new_x > facing.previous_x && facing.left {
                facing.left = false;
                sprite_render.sprite_number -= 1;
            } else if new_x < facing.previous_x && !facing.left {
                facing.left = true;
                sprite_render.sprite_number += 1;
            }

            facing.previous_x = new_x;
        }
    }
}
