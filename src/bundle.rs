use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::DispatcherBuilder,
    prelude::*,
};

use crate::level;
use crate::system;

#[derive(Default)]
pub struct LevelBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for LevelBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            system::PaddleSystem.pausable(level::GameState::Running),
            "paddle_system",
            &[],
        );
        builder.add(
            system::Bounce.pausable(level::GameState::Running),
            "bounce",
            &["paddle_system"],
        );
        builder.add(
            system::BouncedBlock.pausable(level::GameState::Running),
            "bounced_block",
            &["bounce"],
        );
        builder.add(
            system::BelowZero.pausable(level::GameState::Running),
            "below_zero",
            &["bounce"],
        );
        Ok(())
    }
}
