use amethyst::{
    core::{Float, Transform},
    ecs::prelude::{Join, WriteStorage, ReadStorage, System}
};

use crate::component::MovableComponent;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, MovableComponent>
    );

    fn run(&mut self, (mut transforms, movables): Self::SystemData) {
        for (transform, movable) in (&mut transforms, &movables).join() {
            transform.prepend_translation_x(Float::from(movable.direction_x * movable.speed));
            transform.prepend_translation_y(Float::from(movable.direction_y * movable.speed));
        }
    }
}
