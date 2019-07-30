use crate::components::*;
use specs::{Join, System, WriteStorage};

pub struct GravitySystem;

impl<'a> System<'a> for GravitySystem {
    type SystemData = (WriteStorage<'a, transformation::TransformationComponent>);

    fn run(&mut self, mut data: Self::SystemData) {
        for transform in (&mut data).join() {
            transform.0[0] += 1.0
        }
    }
}
