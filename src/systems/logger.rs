use crate::components::name::NameComponent;
use crate::resources::{camera::Camera, delta_time::DeltaTime, texture_map::TextureMap};

use specs::prelude::*;

pub struct LoggerSystem;

impl<'a> System<'a> for LoggerSystem {
	type SystemData = (
		ReadStorage<'a, NameComponent>,
		Read<'a, DeltaTime>,
		Read<'a, Camera>,
		Read<'a, TextureMap>,
	);

	fn run(&mut self, (names, _delta_time, _camera, texture_map): Self::SystemData) {
		println!(
			"# of textures loaded: {}",
			texture_map.texture_handles.len()
		);
		for NameComponent(_name) in names.join() {}
	}
}
