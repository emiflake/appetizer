use crate::components::{
	light::LightComponent, name::NameComponent, transformation::TransformationComponent,
};
use crate::resources::{delta_time::DeltaTime, time::CurrentTime};

use specs::prelude::*;
use std::time::SystemTime;

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
		let now = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();
		let current_time = now.as_millis() as f64;
		let theta = current_time / 1000.0;

		for (name, _light, trans) in (&names, &lights, &mut trans).join() {
			if name.0 == "Random Light" {
				trans.set_pos(glm::vec3(
					(theta.cos() * 20.0) as f32,
					10.0,
					(theta.sin() * 20.0) as f32,
				))
			}
		}
	}
}
