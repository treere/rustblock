use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::DisplayConfig,
};

use crate::component::{Cube, Paddle};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        ReadStorage<'s, Cube>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, DisplayConfig>,
    );

    fn run(&mut self, (cube, mut transforms, paddles, input, conf): Self::SystemData) {
        let width = conf.dimensions.unwrap().0 as f32;
        for (cub, paddle, transform) in (&cube, &paddles, &mut transforms).join() {
            let movement = input.axis_value("move");

            if let Some(mv_amount) = movement {
                transform.move_global(paddle.vel * mv_amount as f32);

                let left_border = 0.;
                let right_border = width - cub.0.half_extents()[0] * 2.0;
                let paddle_x = transform.translation().x;
                transform.set_x(paddle_x.min(right_border).max(left_border));
            }
        }
    }
}
