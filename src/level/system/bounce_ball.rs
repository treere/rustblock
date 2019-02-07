use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::level::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::level::component::{Ball, Paddle};

pub struct BounceBall;

impl<'s> System<'s> for BounceBall {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_pos = transform.translation();

            if ball_pos.y <= ball.radius || SCREEN_HEIGHT - ball_pos.y <= ball.radius {
                ball.vel_y = -ball.vel_y;
            }
            if ball_pos.x <= ball.radius || SCREEN_WIDTH - ball_pos.x <= ball.radius {
                ball.vel_x = -ball.vel_x;
            }
            for (paddle, transform) in (&paddles, &transforms).join() {
                let paddle_pos = transform.translation();
                if ball_pos.x > paddle_pos.x
                    && ball_pos.x < paddle_pos.x + paddle.width
                    && ball_pos.y < paddle_pos.y + paddle.height + ball.radius {
                    ball.vel_y = -ball.vel_y;
                }
            }
        }
    }
}