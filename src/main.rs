use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod wizciv;
mod systems;
mod hex_grid;

use crate::wizciv::WizCiv;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let bindings_config_path = app_root.join("config").join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
            .with_plugin(RenderFlat2D::default()),
            )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config_path)?)?
        .with(systems::MoveCameraSystem, "move_camera_system", &["input_system"])
        .with(systems::TileSelectSystem::new(), "tile_select_system", &["input_system"])
        .with(systems::UnitHighlightSystem, "unit_highlight_system", &["tile_select_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, WizCiv, game_data)?;
    game.run();

    Ok(())
}

