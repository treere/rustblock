use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::DisplayConfig,
};

use crate::component::Ball;

pub struct BounceWall;

impl<'s> System<'s> for BounceWall {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        Read<'s, DisplayConfig>,
    );

    fn run(&mut self, (mut balls, transforms, conf): Self::SystemData) {
        let (width, height) = {
            let (w, h) = conf.dimensions.clone().unwrap();
            (w as f32, h as f32)
        };

        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_pos = transform.translation();

            if ball_pos.y <= ball.radius || height - ball_pos.y <= ball.radius {
                ball.vel[1] = -ball.vel[1];
            }
            if ball_pos.x <= ball.radius || width - ball_pos.x <= ball.radius {
                ball.vel[0] = -ball.vel[0];
            }
        }
    }
}
