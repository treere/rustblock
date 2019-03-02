use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::component::{Direction, Paddle};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Direction>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (paddles, mut directions, input): Self::SystemData) {
        for (dir, paddle) in (&mut directions, &paddles).join() {
            let movement = input.axis_value("move");

            if let Some(mv_amount) = movement {
                dir.0[0] = paddle.speed * mv_amount as f32;
            }
        }
    }
}
