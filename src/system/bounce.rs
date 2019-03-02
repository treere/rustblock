use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

use ncollide2d::{math, query, shape};

use crate::component::{Ball, Bounced, Cube, Direction, Paddle};
use crate::resources::WindowSize;

pub enum Collision {
    Vertical,
    Horizontal,
}

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
        let WindowSize { width, height } = *world_size;

        // Move all
        for (transform, dir) in (&mut transforms, &directions).join() {
            let delta = time.delta_seconds();
            transform.move_global(delta * dir.0);
        }

        // Block paddle movement
        for (cub, _paddle, transform) in (&cube, &paddles, &mut transforms).join() {
            let left_border = 0.;
            let right_border = width - cub.0.half_extents()[0] * 2.0;
            let paddle_x = transform.translation().x;
            transform.set_x(paddle_x.min(right_border).max(left_border));
        }

        // Manage collisions.
        for (ball, ball_pos, dir) in (&balls, &transforms, &mut directions).join() {
            for (cub, cub_pos, entity) in (&cube, &transforms, &entities).join() {
                if let Some(vertical) = bounce(cub_pos, ball_pos, &ball.ball, &cub.0) {
                    match vertical {
                        Collision::Vertical => dir.0[1] = -dir.0[1],
                        Collision::Horizontal => dir.0[0] = -dir.0[0],
                    };
                    bounc.insert(entity, Bounced).unwrap();
                }
            }

            let ball_pos = ball_pos.translation();

            let radius = ball.ball.radius();
            if ball_pos.x <= radius || width - ball_pos.x <= radius {
                dir.0[0] = -dir.0[0];
            }

            if height - ball_pos.y <= radius {
                dir.0[1] = -dir.0[1];
            }
        }
    }
}

pub fn bounce(
    block_transform: &Transform,
    ball_transform: &Transform,
    ball: &shape::Ball<f32>,
    block: &shape::Cuboid<f32>,
) -> Option<Collision> {
    let block_pos = block_transform.translation();
    let ball_pos = ball_transform.translation();

    let isoball = math::Isometry::from_parts(
        math::Translation::from(math::Vector::new(ball_pos.x, ball_pos.y)),
        math::Rotation::identity(),
    );

    let half_width = block.half_extents()[0];
    let half_height = block.half_extents()[1];
    let isoblock = math::Isometry::from_parts(
        math::Translation::from(math::Vector::new(
            block_pos.x + half_width,
            block_pos.y + half_height,
        )),
        math::Rotation::identity(),
    );

    if let Some(collision) = query::contact(&isoblock, block, &isoball, ball, 0.2) {
        let normal = collision.normal.into_inner();

        if normal.y.abs() > 0.9 {
            return Some(Collision::Vertical);
        } else {
            return Some(Collision::Horizontal);
        }
    }

    None
}
