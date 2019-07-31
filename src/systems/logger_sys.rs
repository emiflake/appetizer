use crate::components::{
	light::LightComponent, name::NameComponent, transformation::TransformationComponent,
};
use crate::resources::{
	camera::Camera, delta_time::DeltaTime, keystate::Keystate, texture_map::TextureMap,
	time::CurrentTime,
};

use glfw::Key;
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
		for (name, _light, mut trans) in (&names, &lights, &mut trans).join() {
			if name.0 == "Random Light" {
				trans.set_pos(glm::vec3(
					(current_time.0.cos() * 100.0) as f32,
					50.0,
					(current_time.0.sin() * 100.0) as f32,
				))
			}
		}
	}
}
