use crate::component::object::{Object, AnimationId};

use amethyst::{
    assets::{PrefabLoader, RonFormat, ProgressCounter},
    animation::{AnimationCommand, AnimationSet, AnimationControlSet, get_animation_set, EndControl},
    ecs::{Entities, ReadStorage, WriteStorage, Join},
    prelude::*,
    core::transform::Transform,
    renderer::{Camera, SpriteRender},
    window::ScreenDimensions
};

pub struct GameplayState {
    pub progress_counter: Option<ProgressCounter>
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.delete_all();

//        let test_prefab = world.exec(|loader: PrefabLoader<'_, Position>| {
//            loader.load("prefab/test.ron", RonFormat, ())
//        });
        let bat_prefab = world.exec(|loader: PrefabLoader<'_, Object>| {
            loader.load(
                "prefab/anim_test.ron",
                RonFormat,
                self.progress_counter.as_mut().unwrap())
        });

//        world.create_entity().with(test_prefab).build();
        world.create_entity().with(bat_prefab).build();
        init_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data
        if let Some(ref progress_counter) = self.progress_counter {
            // Checks progress
            if progress_counter.is_complete() {
                let StateData { world, .. } = data;
                // Execute a pass similar to a system
                world.exec(|(entities, animation_sets, mut control_sets): (
                        Entities,
                        ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                        WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
                    )| {
                        // For each entity that has AnimationSet
                        for (entity, animation_set) in (&entities, &animation_sets).join() {
                            // Creates a new AnimationControlSet for the entity
                            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                            // Adds the `Fly` animation to AnimationControlSet and loops infinitely
                            control_set.add_animation(
                                AnimationId::Fly,
                                &animation_set.get(&AnimationId::Fly).unwrap(),
                                EndControl::Loop(None),
                                1.0,
                                AnimationCommand::Start,
                            );
                        }
                    },
                );
                // All data loaded
                self.progress_counter = None;
            }
        }
        Trans::None
    }
}

fn init_camera(world: &mut World) {
    let screen_dimensions = world.read_resource::<ScreenDimensions>().clone();
    let mut transform = Transform::default();
    transform.set_translation_xyz(screen_dimensions.width() / 2., screen_dimensions.height() / 2., 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(screen_dimensions.width(), screen_dimensions.height()))
        .with(transform)
        .build();
}