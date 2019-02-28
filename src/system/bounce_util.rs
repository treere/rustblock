use amethyst::core::transform::Transform;
use ncollide2d::{math, query, shape};

pub enum Collision {
    Vertical,
    Horizontal,
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
    //    let block2d = shape::Cuboid::new(math::Vector::new(0.5 * width, 0.5 * height));
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
