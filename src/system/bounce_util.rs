use amethyst::core::transform::Transform;
use ncollide2d::{math, query, shape};

pub enum Collision {
    Vertical,
    Horizontal,
}

pub fn bounce(
    block_transform: &Transform,
    ball_transform: &Transform,
    radius: f32,
    width: f32,
    height: f32,
) -> Option<Collision> {
    let block_pos = block_transform.translation();
    let ball_pos = ball_transform.translation();

    let ball2d = shape::Ball::new(radius);
    let isoball = math::Isometry::from_parts(
        math::Translation::from(math::Vector::new(ball_pos.x, ball_pos.y)),
        math::Rotation::identity(),
    );
    let block2d = shape::Cuboid::new(math::Vector::new(0.5 * width, 0.5 * height));
    let isoblock = math::Isometry::from_parts(
        math::Translation::from(math::Vector::new(
            block_pos.x + width * 0.5,
            block_pos.y + height * 0.5,
        )),
        math::Rotation::identity(),
    );

    if let Some(collision) = query::contact(&isoblock, &block2d, &isoball, &ball2d, 0.2) {
        let normal = collision.normal.into_inner();

        if normal.y.abs() > 0.9 {
            return Some(Collision::Vertical);
        } else {
            return Some(Collision::Horizontal);
        }
    }

    None
}
