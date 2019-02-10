use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::level::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::level::component::Ball;

pub struct BounceWall;

impl<'s> System<'s> for BounceWall {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_pos = transform.translation();

            if ball_pos.y <= ball.radius || SCREEN_HEIGHT - ball_pos.y <= ball.radius {
                ball.vel[1] = -ball.vel[1];
            }
            if ball_pos.x <= ball.radius || SCREEN_WIDTH - ball_pos.x <= ball.radius {
                ball.vel[0] = -ball.vel[0];
            }
        }
    }
}