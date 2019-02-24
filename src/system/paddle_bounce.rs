use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use super::bounce_util::bounce;
use crate::component::{Ball, Paddle};

pub struct BouncePaddle;

impl<'s> System<'s> for BouncePaddle {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            for (paddle, transformp) in (&paddles, &transforms).join() {
                match bounce(
                    transformp,
                    transform,
                    ball.radius,
                    paddle.width,
                    paddle.height,
                ) {
                    (false, _) => (),
                    (true, vertical) => {
                        if vertical {
                            ball.vel[1] = -ball.vel[1];
                        } else {
                            ball.vel[0] = -ball.vel[0];
                        }
                    }
                }
            }
        }
    }
}
