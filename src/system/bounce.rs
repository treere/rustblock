use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::DisplayConfig,
};

use super::bounce_util::{bounce, Collision};
use crate::component::{Ball, Bounced, Cube};
pub struct Bounce;

impl<'s> System<'s> for Bounce {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Cube>,
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Bounced>,
        Read<'s, DisplayConfig>,
    );

    fn run(&mut self, (entities, cube, mut balls, transforms, mut bounc, conf): Self::SystemData) {
        let (width, height) = {
            let (w, h) = conf.dimensions.unwrap();
            (w as f32, h as f32)
        };

        for (ball, transform) in (&mut balls, &transforms).join() {
            for (cub, transformb, entity) in (&cube, &transforms, &entities).join() {
                if let Some(vertical) = bounce(transformb, transform, &ball.ball, &cub.0) {
                    match vertical {
                        Collision::Vertical => ball.vel[1] = -ball.vel[1],
                        Collision::Horizontal => ball.vel[0] = -ball.vel[0],
                    };
                    bounc.insert(entity, Bounced).unwrap();
                }
            }

            let ball_pos = transform.translation();

            let radius = ball.ball.radius();
            if ball_pos.x <= radius || width - ball_pos.x <= radius {
                ball.vel[0] = -ball.vel[0];
            }

            if height - ball_pos.y <= radius {
                ball.vel[1] = -ball.vel[1];
            }
        }
    }
}
