extern crate amethyst;

use std::path::Path;
use std::time::Duration;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{
        DrawFlat,
        PosTex,
    },
    utils::application_root_dir,
};

use level::Level;

mod level;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();
    let config = format!("{}/resources/display_config.ron", app_root);

    let input_config = format!("{}/resources/input_config.ron", app_root);
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(Path::new(&input_config))?;

    let game_data = GameDataBuilder::default()
        .with_basic_renderer(config, DrawFlat::<PosTex>::new(), true)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(level::system::MoveBallSysytem, "move_ball", &[])
        .with(level::system::PaddleSystem, "paddle_system", &[])
        .with(level::system::BounceBall, "bounce_ball", &["move_ball"]);

    let mut game = Application::build("./", Level)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60,
        )
        .build(game_data)?;

    game.run();

    Ok(())
}
