use std::collections::HashSet;

pub struct MouseState {
	pub position: glm::Vec2,
	pub delta: glm::Vec2,
	pub is_locked: bool,
	pub buttons: HashSet<glutin::MouseButton>,
}

impl Default for MouseState {
	fn default() -> Self {
		Self {
			position: glm::vec2(0.0, 0.0),
			delta: glm::vec2(0.0, 0.0),
			is_locked: false,
			buttons: HashSet::new(),
		}
	}
}

impl MouseState {
	pub fn set_button_down(&mut self, button: glutin::MouseButton) {
		self.buttons.insert(button);
	}

	pub fn set_button_up(&mut self, button: glutin::MouseButton) {
		self.buttons.remove(&button);
	}

	pub fn is_button_down(&self, button: glutin::MouseButton) -> bool {
		self.buttons.contains(&button)
	}
}
