use std::collections::HashSet;
use glutin::ElementState;

pub struct MouseState {
	pub position: glm::Vec2,
	pub previous_position: glm::Vec2,
	pub delta: glm::Vec2,
	pub buttons: HashSet<glutin::MouseButton>,
}

impl Default for MouseState {
	fn default() -> Self {
		Self {
			position: glm::vec2(0.0, 0.0),
			previous_position: glm::vec2(0.0, 0.0),
			delta: glm::vec2(0.0, 0.0),
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

	pub fn update_delta(&mut self) {
		self.delta = self.position - self.previous_position;
		self.previous_position = self.position;
	}

	pub fn handle_event(&mut self, event: &glutin::Event) {
		if let glutin::Event::WindowEvent { event, .. } = event {
			match event {
				glutin::WindowEvent::MouseInput { state, button, .. } => match state {
					ElementState::Pressed => self.set_button_down(*button),
					ElementState::Released => self.set_button_up(*button),
				},
				glutin::WindowEvent::CursorMoved { position, .. } => {
					self.position = glm::vec2(position.x as f32, position.y as f32);
				},
				_ => {}
			}
		}
	}
}
