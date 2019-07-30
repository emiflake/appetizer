use crate::components::transformation::TransformationComponent;
use specs::*;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
	type SystemData = (ReadStorage<'a, TransformationComponent>);

	fn run(&mut self, trans: Self::SystemData) {
		unsafe {
			gl::ClearColor(1.0, 0.5, 0.3, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
		for _pos in trans.join() {}
	}
}
