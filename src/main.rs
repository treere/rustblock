extern crate amethyst;

use std::time::Duration;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
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

    let game_data = GameDataBuilder::default()
        .with_basic_renderer(config, DrawFlat::<PosTex>::new(), true)?
        .with_bundle(TransformBundle::new())?
        .with(level::system::MoveBallSysytem, "move_ball", &[])
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
