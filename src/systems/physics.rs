use crate::components::FallingObject;
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{prelude::*, Join, Read, System, WriteStorage},
};
use nalgebra::Vector2;

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, FallingObject>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut falling_objects, time): Self::SystemData) {
        let frame_delta_s = time.fixed_time().as_secs_f32();
        for (transform, falling_obj) in (&mut transforms, &mut falling_objects).join() {
            falling_obj.v += falling_obj.g * frame_delta_s * Vector2::new(0.0, 1.0);

            let translation_offset = falling_obj.v * frame_delta_s;

            let translation = transform.translation_mut();
            translation.x += translation_offset.x;
            translation.y += translation_offset.y;
        }
    }
}
