use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::level::component::Ball;

pub struct MoveBallSysytem;

impl<'s> System<'s> for MoveBallSysytem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut transforms, time): Self::SystemData) {
        for (ball, transform) in (&balls, &mut transforms).join() {
            let delta = time.delta_seconds();
            transform.move_global(delta * ball.vel);
        }
    }
}
