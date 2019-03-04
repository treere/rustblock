extern crate amethyst;
extern crate ncollide2d;
extern crate serde;

use std::path::Path;
use std::time::Duration;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
    LoggerConfig, StdoutLog,
};

mod component;
mod config;
mod intro;
mod level;
mod loading;
mod pause;
mod resources;
mod system;
mod util;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        stdout: StdoutLog::Plain,
        ..Default::default()
    });

    let app_root = application_root_dir();

    let input_config = format!("{}/resources/input_config.ron", app_root);
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(Path::new(&input_config))?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new())
            .with_pass(DrawUi::new()),
    );

    let display_config = {
        let display_config = format!("{}/resources/display_config.ron", app_root);
        DisplayConfig::load(display_config)
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config.clone())))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(
            system::PaddleSystem.pausable(level::GameState::Running),
            "paddle_system",
            &[],
        )
        .with(
            system::Bounce.pausable(level::GameState::Running),
            "bounce",
            &["paddle_system"],
        )
        .with(
            system::BouncedBlock.pausable(level::GameState::Running),
            "bounced_block",
            &["bounce"],
        )
        .with(
            system::BelowZero.pausable(level::GameState::Running),
            "below_zero",
            &["bounce"],
        );

    let mut game = Application::build("./", loading::Loading)?
        .with_resource(display_config)
        .with_resource(config::PaddleConfig::default())
        .with_resource(config::BallConfig::default())
        .with_resource(config::BlockConfig::default())
        .build(game_data)?;

    game.run();

    Ok(())
}
