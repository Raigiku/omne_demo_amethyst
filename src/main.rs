mod component;
mod prefab;
mod render;
mod resource;
mod state;
mod system;

use amethyst::{
    assets::{PrefabLoaderSystem, Processor},
    audio::Source,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        sprite::SpriteSheet, sprite_visibility::SpriteVisibilitySortingSystem,
        types::DefaultBackend, RenderingSystem,
    },
    ui::UiBundle,
    utils::application_root_dir,
    window::WindowBundle,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config = assets_dir.join("config/display_properties.ron");

    let render_graph = render::RenderGraph::default();
    let render_system = RenderingSystem::<DefaultBackend, _>::new(render_graph);

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(assets_dir.join("config/input_pc.ron"))?,
        )?
        .with(PrefabLoaderSystem::<prefab::Scene1>::default(), "", &[])
        .with(Processor::<SpriteSheet>::new(), "", &[])
        .with(Processor::<Source>::new(), "", &[])
        .with(
            SpriteVisibilitySortingSystem::new(),
            "",
            &["transform_system"],
        )
        .with(system::Movement, "movement_system", &[])
        .with(system::Player, "player_system", &["input_system"])
        .with_thread_local(render_system);

    let mut game = Application::new(assets_dir, state::MainMenu, game_data)?;
    game.run();

    Ok(())
}
