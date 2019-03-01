use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadStorage, System},
};

pub struct BelowZero;

impl<'s> System<'s> for BelowZero {
    type SystemData = (Entities<'s>, ReadStorage<'s, Transform>);

    fn run(&mut self, (entities, transforms): Self::SystemData) {
        for (e, transform) in (&entities, &transforms).join() {
            let ball_pos = transform.translation();

            if ball_pos.y <= -10.0 {
                entities.delete(e).unwrap();
            }
        }
    }
}
