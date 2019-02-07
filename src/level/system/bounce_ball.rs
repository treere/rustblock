use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::level::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::level::component::Ball;

pub struct BounceBall;

impl<'s> System<'s> for BounceBall {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>
    );

    fn run(&mut self, (mut balls, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let position = transform.translation();

            if position.y <= ball.radius || SCREEN_WIDTH - position.y <= ball.radius {
                ball.vel_y = -ball.vel_y;
                println!("HIT Y!");
            }
            if position.x <= ball.radius || SCREEN_HEIGHT - position.x <= ball.radius {
                ball.vel_x = -ball.vel_x;
                println!("HIT Y!");
            }
        }
    }
}