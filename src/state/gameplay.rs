use amethyst::{
    assets::{PrefabLoader, RonFormat, ProgressCounter, Handle, Prefab},
    prelude::*,
    core::transform::Transform,
    renderer::{Camera},
    window::ScreenDimensions
};
use derive_new::new;

use crate::prefab::Scene1;

#[derive(new)]
pub struct GameplayState {
    #[new(default)]
    pub progress_counter: ProgressCounter,
    #[new(default)]
    pub prefab_handle: Option<Handle<Prefab<Scene1>>>
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let scene1_prefab = world.exec(|loader: PrefabLoader<'_, Scene1>| {
            loader.load(
                "prefab/scene1.ron",
                RonFormat,
                &mut self.progress_counter
            )
        });

        world.delete_all();
        world.create_entity().with(scene1_prefab.clone()).build();
        self.init_camera(world);

        self.prefab_handle = Some(scene1_prefab);
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
