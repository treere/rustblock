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
mod dispatcher;
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

    let game_data = dispatcher::CustomGameDataBuilder::default()
        .with_base_bundle(RenderBundle::new(pipe, Some(display_config.clone())))?
        .with_base_bundle(TransformBundle::new())?
        .with_base_bundle(input_bundle)?
        .with_base_bundle(UiBundle::<String, String>::new())?
        .with_running(system::MoveBallSysytem, "move_ball", &[])
        .with_running(system::PaddleSystem, "paddle_system", &[])
        .with_running(system::BouncePaddle, "bounce_paddle", &["move_ball"])
        .with_running(system::BounceBlock, "bounce_block", &["move_ball"])
        .with_running(system::BouncedBlock, "bounced_block", &["bounce_block"])
        .with_running(
            system::BounceWall,
            "bounce_wall",
            &["move_ball", "bounce_paddle", "bounced_block"],
        );

    let mut game = Application::build("./", loading::Loading)?
        .with_resource(display_config)
        .with_resource(config::PaddleConfig::default())
        .with_resource(config::BallConfig::default())
        .with_resource(config::BlockConfig::default())
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(1)),
            30,
        )
        .build(game_data)?;

    game.run();

    Ok(())
}
