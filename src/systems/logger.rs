use crate::components::name::NameComponent;
use crate::resources::{
	camera::Camera, delta_time::DeltaTime, keystate::Keystate, texture_map::TextureMap,
};

use glfw::Key;
use specs::prelude::*;

pub struct LoggerSystem;

impl<'a> System<'a> for LoggerSystem {
	type SystemData = (
		ReadStorage<'a, NameComponent>,
		Read<'a, DeltaTime>,
		Read<'a, Keystate>,
	);

	fn run(&mut self, (names, _delta_time, keystate): Self::SystemData) {
		println!("{:?}", keystate.is_key_down(Key::Space));
		for NameComponent(_name) in names.join() {}
	}
}
