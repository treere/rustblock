use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::level::component::{Ball, Paddle};

pub struct BouncePaddle;


impl<'s> System<'s> for BouncePaddle {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            for (paddle, transformp) in (&paddles, &transforms).join() {
                bounce(transformp, transform, ball, paddle);
            }
        }
    }
}

fn bounce(paddle_transform: &Transform, ball_transform: &Transform, ball: &mut Ball, paddle: &Paddle) {
    let paddle_pos = paddle_transform.translation();
    let ball_pos = ball_transform.translation();
    let y = ball_pos.y;
    let x = ball_pos.x;
    let r = ball.radius;
    let bot_value = paddle_pos.y - r;
    let top_value = paddle_pos.y + paddle.height + r;
    let left_value = paddle_pos.x - r;
    let right_value = paddle_pos.x + paddle.width + r;
    let p = ball_pos;
    let v = ball.vel;

    let bot = {
        y > bot_value
    };
    let top = {
        y < top_value
    };
    let left = {
        x > left_value
    };
    let right = {
        x < right_value
    };

    if bot & &top & &left && right {
        let a = left_value;
        let b = right_value;
        let x1 = {
            let y = bot_value;
            let x = p[0] + v[0] * (y - p[1]) / v[1];
            ((x - p[0]).abs() < r) && (a < x) && (x < b)
        };
        let x2 = {
            let y = top_value;
            let x = p[0] + v[0] * (y - p[1]) / v[1];
            ((x - p[0]).abs() < r) && (a < x) && (x < b)
        };

        if x1 || x2 {
            ball.vel[1] = -ball.vel[1];
        } else {
            ball.vel[0] = -ball.vel[0];
        }
    }
}