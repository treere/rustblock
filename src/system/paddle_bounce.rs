use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use super::bounce_util::{bounce, Collision};
use crate::component::{Ball, Cube, Paddle};

pub struct BouncePaddle;

impl<'s> System<'s> for BouncePaddle {
    type SystemData = (
        ReadStorage<'s, Cube>,
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (cube, mut balls, paddles, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            for (cub, _paddle, transformp) in (&cube, &paddles, &transforms).join() {
                match bounce(transformp, transform, &ball.ball, &cub.0) {
                    Some(Collision::Vertical) => ball.vel[1] = -ball.vel[1],
                    Some(Collision::Horizontal) => ball.vel[0] = -ball.vel[0],
                    _ => (),
                }
            }
        }
    }
}
