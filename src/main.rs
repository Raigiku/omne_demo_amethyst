mod render;
mod state;
mod component;

use state::MainMenuState;
use component::position::Position;
use crate::component::Object;
use crate::component::object::AnimationId;

use amethyst::{
    assets::{Processor, PrefabLoaderSystem},
    animation::{AnimationBundle},
    audio::Source,
    core::transform::TransformBundle,
    input::{StringBindings, InputBundle},
    prelude::*,
    renderer::{
        sprite_visibility::SpriteVisibilitySortingSystem,
        sprite::{SpriteSheet, SpriteRender},
        types::DefaultBackend,
        RenderingSystem,
    },
    ui::UiBundle,
    utils::application_root_dir,
    window::{WindowBundle, DisplayConfig}
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources_dir = app_root.join("resources");
    let icon_dir = resources_dir.join("image/omne_icon.png");

    let mut display_config = DisplayConfig::default();
    display_config.title = String::from("Omne");
    display_config.dimensions = Some((1024, 768));
    display_config.icon.replace(icon_dir);

    let render_graph = render::RenderGraph::default();
    let render_system = RenderingSystem::<DefaultBackend, _>::new(render_graph);

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config(display_config))?
        .with(
            PrefabLoaderSystem::<Object>::default(),
            "scene_loader",
            &[]
        )
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation"
        ))?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"])
        )?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &["transform_system"],
        )
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with(PrefabLoaderSystem::<Position>::default(), "", &[])
        .with_thread_local(render_system);

    let mut game = Application::new(resources_dir, MainMenuState, game_data)?;
    game.run();

    Ok(())
}
