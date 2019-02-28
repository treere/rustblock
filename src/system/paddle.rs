use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::DisplayConfig,
};

use crate::component::Paddle;

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, DisplayConfig>,
    );

    fn run(&mut self, (mut transforms, paddles, input, conf): Self::SystemData) {
        let width = conf.dimensions.unwrap().0 as f32;
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = input.axis_value("move");

            if let Some(mv_amount) = movement {
                let scaled_amount = paddle.speed * mv_amount as f32;
                let paddle_x = transform.translation().x;
                transform.set_x(
                    (paddle_x + scaled_amount)
                        .min(width - paddle.paddle.half_extents()[0] * 2.0)
                        .max(0.),
                );
            }
        }
    }
}
