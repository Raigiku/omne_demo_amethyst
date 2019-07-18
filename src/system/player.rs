use amethyst::{
    ecs::prelude::{Join, WriteStorage, ReadStorage, Read, System},
    input::{InputHandler, StringBindings}
};

use crate::component::{MovableComponent, PlayerComponent};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, MovableComponent>,
        ReadStorage<'s, PlayerComponent>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut movables, players, input): Self::SystemData) {
        for (movable, _) in (&mut movables, &players).join() {
            let horizontal_direction = input.axis_value("horizontal");
            let vertical_direction = input.axis_value("vertical");

            if let Some(horizontal_direction) = horizontal_direction {
                movable.direction_x = horizontal_direction;
            }
            if let Some(vertical_direction) = vertical_direction {
                movable.direction_y = vertical_direction;
            }
        }
    }
}
