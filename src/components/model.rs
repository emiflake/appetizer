use specs::{Component, VecStorage};

use crate::object::VertexArray;

// Stores the VAO
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ModelComponent {
	pub vertices: Vec<VertexArray>,
	pub indices: glium::index::NoIndices,
}
