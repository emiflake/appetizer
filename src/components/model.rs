use specs::{Component, VecStorage};

// Stores the VAO
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ModelComponent {
	pub vao: u32,
	pub length: i32,
}
