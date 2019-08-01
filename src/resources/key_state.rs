use std::collections::HashSet;

#[derive(Default)]
pub struct Keystate {
	pub keys: HashSet<glutin::VirtualKeyCode>,
}

impl Keystate {
	pub fn set_key_down(&mut self, key: glutin::VirtualKeyCode) {
		self.keys.insert(key);
	}

	pub fn set_key_up(&mut self, key: glutin::VirtualKeyCode) {
		self.keys.remove(&key);
	}

	pub fn is_key_down(&self, key: glutin::VirtualKeyCode) -> bool {
		self.keys.contains(&key)
	}
}
