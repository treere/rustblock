use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use super::bounce_util::{bounce, Collision};
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
                match bounce(transformp, transform, &ball.ball, &paddle.paddle) {
                    Some(Collision::Vertical) => ball.vel[1] = -ball.vel[1],
                    Some(Collision::Horizontal) => ball.vel[0] = -ball.vel[0],
                    _ => (),
                }
            }
        }
    }
}
