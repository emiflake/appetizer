use crate::resources::mousestate;
use specs::prelude::*;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
	type SystemData = Write<'a, mousestate::MouseState>;

	fn run(&mut self, mut mousestate: Self::SystemData) {
		mousestate.is_locked = mousestate.is_button_down(glfw::MouseButton::Button2);
	}
}
