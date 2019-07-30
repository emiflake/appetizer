use crate::components::transformation::TransformationComponent;
use specs::*;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, TransformationComponent>);

    fn run(&mut self, trans: Self::SystemData) {
        for _pos in trans.join() {}
    }
}
