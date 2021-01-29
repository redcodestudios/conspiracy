mod conspiracy;
mod systems;

use amethyst::{
    prelude::*,
    assets::{LoaderBundle},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        rendy::hal::command::ClearColor,
    },
    core::{transform::TransformBundle},
    input::{InputBundle},
    ui::{RenderUi, UiBundle},
    animation::{AnimationBundle},
    renderer::{SpriteRender},
    utils::application_root_dir,
};

use conspiracy::*;
use systems::player::PlayerSystem;
use systems::animation::AnimationSystem;

const BACKGROUND_COLOUR: ClearColor = ClearColor {
    float32: [0.0, 0.0, 0.0, 1.0],
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets/");

    let key_bindings_path = app_root.join("config/input.ron");

    let mut dispatcher = DispatcherBuilder::default();
    dispatcher
        .add_bundle(LoaderBundle)
        .add_bundle(TransformBundle)
        .add_bundle(AnimationBundle::<BiluAnimations, SpriteRender>::default())
        .add_bundle(
            InputBundle::new().with_bindings_from_file(key_bindings_path)?,
        )
        .add_system(Box::new(PlayerSystem))
        .add_system(Box::new(AnimationSystem))
        .add_bundle(UiBundle::<u32>::default())
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear(BACKGROUND_COLOUR),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        );

    let game = Application::new(assets_dir, Conspiracy::default(), dispatcher)?;

    game.run();
    Ok(())
}
