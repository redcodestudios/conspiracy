mod conspiracy;

use std::time::Duration;

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
    input::{InputBundle, Bindings},
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use conspiracy::*;

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
        .add_bundle(
            InputBundle::new().with_bindings_from_file(key_bindings_path)?,
        )
        .add_bundle(UiBundle::<u32>::default())
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear(BACKGROUND_COLOUR),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        );

    let mut game = Application::new(assets_dir, Conspiracy::default(), dispatcher)?;

    game.run();
    Ok(())
}
