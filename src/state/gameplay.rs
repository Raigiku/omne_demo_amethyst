use amethyst::{
    assets::{PrefabLoader, RonFormat, ProgressCounter, Handle, Prefab},
    ecs::{WriteStorage, ReadStorage, Join},
    prelude::*,
    core::transform::Transform,
    renderer::{Camera},
    window::ScreenDimensions
};
use derive_new::new;

use crate::prefab::Scene1Prefab;
use crate::component::PlayerComponent;

#[derive(new)]
pub struct GameplayState {
    #[new(value = "Some(Default::default())")]
    pub progress_counter: Option<ProgressCounter>,
    #[new(default)]
    pub prefab_handle: Option<Handle<Prefab<Scene1Prefab>>>
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let scene1_prefab = world.exec(|loader: PrefabLoader<'_, Scene1Prefab>| {
            loader.load(
                "prefab/scene1.ron",
                RonFormat,
                self.progress_counter.as_mut().unwrap()
            )
        });

        world.delete_all();
        world.create_entity().with(scene1_prefab.clone()).build();

        self.init_camera(world);
        self.prefab_handle = Some(scene1_prefab);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // if let Some(ref progress_counter) = self.progress_counter {
        //     if progress_counter.is_complete() {
        //         data.world.exec(
        //             |(mut transforms, players): (WriteStorage<Transform>, ReadStorage<PlayerComponent>)| {
        //                 for (transform, _) in (&mut transforms, &players).join() {
        //                     transform.prepend_translation_x(40.0);
        //                 }
        //             }
        //         );
        //         self.progress_counter = None;
        //     }
        // }
        Trans::None
    }
}

impl GameplayState {
    fn init_camera(&self, world: &mut World) {
        let screen_dimensions = world.read_resource::<ScreenDimensions>().clone();
        let mut transform = Transform::default();
        transform.set_translation_xyz(screen_dimensions.width() / 2., screen_dimensions.height() / 2., 1.);

        world
            .create_entity()
            .with(Camera::standard_2d(screen_dimensions.width(), screen_dimensions.height()))
            .with(transform)
            .build();
    }
}
