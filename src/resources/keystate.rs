pub struct Keystate {
	pub keys: [bool; 512],
}

impl Default for Keystate {
	fn default() -> Self {
		Self { keys: [false; 512] }
	}
}
