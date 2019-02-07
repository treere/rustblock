use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::level::component::Paddle;
use crate::level::SCREEN_WIDTH;

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = input.axis_value("move");

            if let Some(mv_amount) = movement {
                let scaled_amount = paddle.speed * mv_amount as f32;
                let paddle_x = transform.translation().x;
                transform.set_x(
                    (paddle_x + scaled_amount)
                        .min(SCREEN_WIDTH - paddle.width)
                        .max(0.),
                );
            }
        }
    }
}
