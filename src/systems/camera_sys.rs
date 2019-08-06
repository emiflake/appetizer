use crate::components::*;
use crate::resources::*;
use specs::prelude::*;

use camera::CameraDirection;

pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
	type SystemData = (
		Write<'a, camera::Camera>,
		Read<'a, delta_time::DeltaTime>,
		Read<'a, key_state::Keystate>,
		Read<'a, mouse_state::MouseState>,
	);

	fn run(&mut self, (mut cam, delta_time, key_state, mouse_state): Self::SystemData) {
		cam.speed = if key_state.is_key_down(glutin::VirtualKeyCode::LShift) {
			100.0
		} else {
			1.0
		};
		if key_state.is_key_down(glutin::VirtualKeyCode::W) {
			cam.do_move(CameraDirection::Forward, delta_time.0);
		}
		if key_state.is_key_down(glutin::VirtualKeyCode::S) {
			cam.do_move(CameraDirection::Backward, delta_time.0);
		}
		if key_state.is_key_down(glutin::VirtualKeyCode::A) {
			cam.do_move(CameraDirection::Left, delta_time.0);
		}
		if key_state.is_key_down(glutin::VirtualKeyCode::D) {
			cam.do_move(CameraDirection::Right, delta_time.0);
		}

		if mouse_state.is_button_down(glutin::MouseButton::Right) {
			cam.do_rotate(glm::vec2(
				mouse_state.delta.x * delta_time.0 * 10.0,
				-mouse_state.delta.y * delta_time.0 * 10.0,
			));
		}
	}
}
