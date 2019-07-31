use crate::resources::mouse_state;
use specs::prelude::*;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
	type SystemData = Write<'a, mouse_state::MouseState>;

	fn run(&mut self, mut mouse_state: Self::SystemData) {
		mouse_state.is_locked = mouse_state.is_button_down(glfw::MouseButton::Button2);
	}
}
