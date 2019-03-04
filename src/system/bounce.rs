use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

use ncollide2d::{math, query};

use crate::component::{Ball, Bounced, Cube, Direction, Paddle};
use crate::resources::WindowSize;

pub struct Bounce;

impl<'s> System<'s> for Bounce {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Cube>,
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Bounced>,
        Read<'s, WindowSize>,
        ReadStorage<'s, Paddle>,
        Read<'s, Time>,
        WriteStorage<'s, Direction>,
    );

    fn run(
        &mut self,
        (
            entities,
            cube,
            balls,
            mut transforms,
            mut bounc,
            world_size,
            paddles,
            time,
            mut directions,
        ): Self::SystemData,
    ) {
        let WindowSize { width, .. } = *world_size;

        // Move all
        for (transform, dir) in (&mut transforms, &directions).join() {
            let delta = time.delta_seconds();
            transform.move_global(delta * dir.0);
        }

        // Block paddle movement
        for (cub, _paddle, transform) in (&cube, &paddles, &mut transforms).join() {
            let left_border = cub.obj.half_extents()[0];
            let right_border = width - cub.obj.half_extents()[0];
            let paddle_x = transform.translation().x;
            transform.set_x(paddle_x.min(right_border).max(left_border));
        }

        // Manage collisions.
        for (ball, ball_pos, dir, ()) in (&balls, &transforms, &mut directions, !&cube).join() {
            let ball_pos = ball_pos.translation();

            let isoball = math::Isometry::from_parts(
                math::Translation::from(math::Vector::new(ball_pos.x, ball_pos.y)),
                math::Rotation::identity(),
            );

            for (cub, cub_pos, entity, ()) in (&cube, &transforms, &entities, !&balls).join() {
                let block_pos = cub_pos.translation();

                let isoblock = math::Isometry::from_parts(
                    math::Translation::from(math::Vector::new(block_pos.x, block_pos.y)),
                    math::Rotation::identity(),
                );

                match query::contact(&isoblock, &cub.obj, &isoball, &ball.obj, 0.2) {
                    Some(collision) => {
                        let normal = collision.normal.into_inner();

                        if normal.y.abs() > 0.9 {
                            dir.0[1] = -dir.0[1]
                        } else {
                            dir.0[0] = -dir.0[0]
                        }
                        bounc.insert(entity, Bounced).unwrap();
                    }
                    _ => (),
                }
            }
        }
    }
}
