use amethyst::{
    core::{Float, Transform},
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::component;

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, component::Movable>,
    );

    fn run(&mut self, (mut transforms, movables): Self::SystemData) {
        for (transform, movable) in (&mut transforms, &movables).join() {
            transform.prepend_translation_x(Float::from(movable.direction_x * movable.speed));
            transform.prepend_translation_y(Float::from(movable.direction_y * movable.speed));
        }
    }
}
