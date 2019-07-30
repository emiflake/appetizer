use crate::components::*;
use crate::resources::*;
use glfw::Key;
use specs::prelude::*;

use camera::CameraDirection;

pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
	type SystemData = (
		Write<'a, camera::Camera>,
		Read<'a, delta_time::DeltaTime>,
		Read<'a, keystate::Keystate>,
		Read<'a, mousestate::MouseState>,
	);

	fn run(&mut self, (mut cam, delta_time, keystate, mousestate): Self::SystemData) {
		cam.speed = if keystate.is_key_down(Key::LeftShift) {
			100.0
		} else {
			1.0
		};
		if keystate.is_key_down(Key::W) {
			cam.do_move(CameraDirection::Forward, delta_time.0);
		}
		if keystate.is_key_down(Key::S) {
			cam.do_move(CameraDirection::Backward, delta_time.0);
		}
		if keystate.is_key_down(Key::A) {
			cam.do_move(CameraDirection::Left, delta_time.0);
		}
		if keystate.is_key_down(Key::D) {
			cam.do_move(CameraDirection::Right, delta_time.0);
		}

		if mousestate.is_locked {
			cam.do_rotate(glm::vec2(
				-mousestate.delta.x * delta_time.0 * 50.0,
				mousestate.delta.y * delta_time.0 * 50.0,
			));
		}
	}
}
