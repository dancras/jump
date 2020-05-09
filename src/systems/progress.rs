use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{ReadStorage, Join, System, SystemData, WriteStorage},
    renderer::{Camera},
};
use crate::jump::{Moveable};

#[derive(SystemDesc)]
pub struct ProgressSystem;

impl<'s> System<'s> for ProgressSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Moveable>
    );

    fn run(&mut self, (mut transforms, cameras, moveables): Self::SystemData) {

        let mut player_y = 0.0;

        for (transform, _moveable) in (&transforms, &moveables).join() {
            player_y = transform.isometry().translation.y;
        }

        for (transform, _camera) in (&mut transforms, &cameras).join() {
            transform.set_translation_y(player_y);
        }
    }
}
