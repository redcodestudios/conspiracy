mod conspiracy;

use std::time::Duration;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::{transform::TransformBundle},
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use conspiracy::*;

const BACKGROUND_COLOUR: [f32; 4] = [0.25, 0.25, 0.25, 0.0];

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets/");

    let key_bindings_path = {
        if cfg!(feature = "sdl_controller") {
            app_root.join("config/input_controller.ron")
        } else {
            app_root.join("config/input.ron")
        }
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
            )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear(BACKGROUND_COLOUR),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(assets_dir, Conspiracy::default(), game_data)?;

    game.run();
    Ok(())
}
