use crate::resources::*;
use specs::prelude::*;

pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
	type SystemData = (
		Write<'a, camera::Camera>,
		Read<'a, delta_time::DeltaTime>,
		Read<'a, key_state::Keystate>,
		Read<'a, mouse_state::MouseState>,
	);

	fn run(&mut self, (mut cam, delta_time, key_state, mouse_state): Self::SystemData) {
		let speed = if key_state.is_key_down(glutin::VirtualKeyCode::LShift) {
			100.0
		} else {
			1.0
		};

		if key_state.is_key_down(glutin::VirtualKeyCode::W) {
			cam.do_move(glm::vec3(0.0, 0.0, speed), delta_time.0);
		}
		if key_state.is_key_down(glutin::VirtualKeyCode::S) {
			cam.do_move(glm::vec3(0.0, 0.0, -speed), delta_time.0);
		}
		if key_state.is_key_down(glutin::VirtualKeyCode::A) {
			cam.do_move(glm::vec3(-speed, 0.0, 0.0), delta_time.0);
		}
		if key_state.is_key_down(glutin::VirtualKeyCode::D) {
			cam.do_move(glm::vec3(speed, 0.0, 0.0), delta_time.0);
		}

		if mouse_state.is_button_down(glutin::MouseButton::Right) {
			cam.do_rotate(glm::vec2(
				mouse_state.delta.x * delta_time.0 * 10.0,
				-mouse_state.delta.y * delta_time.0 * 10.0,
			));
		}
	}
}
