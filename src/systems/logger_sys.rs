use crate::components::{
	light::LightComponent, name::NameComponent, transformation::TransformationComponent,
};
use crate::resources::{delta_time::DeltaTime, time::CurrentTime};

use specs::prelude::*;

pub struct LoggerSystem;

impl<'a> System<'a> for LoggerSystem {
	type SystemData = (
		ReadStorage<'a, NameComponent>,
		ReadStorage<'a, LightComponent>,
		WriteStorage<'a, TransformationComponent>,
		Read<'a, DeltaTime>,
		Read<'a, CurrentTime>,
	);

	fn run(&mut self, (names, lights, mut trans, _delta_time, current_time): Self::SystemData) {
		for (name, _light, trans) in (&names, &lights, &mut trans).join() {
			println!("{:?}", current_time.0);
			if name.0 == "Random Light" {
				trans.set_pos(glm::vec3(
					(current_time.0.cos() * 20.0) as f32,
					50.0,
					(current_time.0.sin() * 20.0) as f32,
				))
			}
		}
	}
}
