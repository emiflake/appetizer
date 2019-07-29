use crate::components::name::NameComponent;
use crate::resources::delta_time::DeltaTime;

use specs::prelude::*;

pub struct LoggerSystem;

impl<'a> System<'a> for LoggerSystem {
	type SystemData = (ReadStorage<'a, NameComponent>, Read<'a, DeltaTime>);

	fn run(&mut self, (names, _delta_time): Self::SystemData) {
		for NameComponent(_name) in names.join() {}
	}
}
