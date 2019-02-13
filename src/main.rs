extern crate amethyst;
extern crate serde;

use std::path::Path;
use std::time::Duration;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage},
    ui::DrawUi,
    utils::application_root_dir,
};

use level::config::*;
use level::Level;

mod level;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

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
        .with(level::system::MoveBallSysytem, "move_ball", &[])
        .with(level::system::PaddleSystem, "paddle_system", &[])
        .with(level::system::BouncePaddle, "bounce_paddle", &["move_ball"])
        .with(level::system::BounceBlock, "bounce_block", &["move_ball"])
        .with(level::system::BounceWall, "bounce_wall", &["move_ball"])
        .with(
            level::system::BouncedBlock,
            "bounced_block",
            &["bounce_block"],
        );

    let mut game = Application::build("./", Level)?
        .with_resource(display_config)
        .with_resource(PaddleConfig::default())
        .with_resource(BallConfig::default())
        .with_resource(BlockConfig::default())
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(1)),
            30,
        )
        .build(game_data)?;

    game.run();

    Ok(())
}
