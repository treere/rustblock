use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::DisplayConfig,
};

use crate::component::Ball;

pub struct BounceWall;

impl<'s> System<'s> for BounceWall {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        Read<'s, DisplayConfig>,
    );

    fn run(&mut self, (ent, mut balls, transforms, conf): Self::SystemData) {
        let (width, height) = {
            let (w, h) = conf.dimensions.unwrap();
            (w as f32, h as f32)
        };

        for (e, ball, transform) in (&ent, &mut balls, &transforms).join() {
            let ball_pos = transform.translation();

            let radius = ball.ball.radius();
            if ball_pos.x <= radius || width - ball_pos.x <= radius {
                ball.vel[0] = -ball.vel[0];
            }

            if height - ball_pos.y <= radius {
                ball.vel[1] = -ball.vel[1];
            } else if ball_pos.y <= -10.0 {
                ent.delete(e).unwrap();
            }
        }
    }
}
