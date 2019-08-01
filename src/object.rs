use crate::components::model::ModelComponent;
use gl::types::*;
use std::ffi::c_void;
use std::mem;
use std::ptr;

use glium::implement_vertex;

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
	pub position: glm::Vec3,
	pub normal: glm::Vec3,
	pub uv: glm::Vec2,
}

#[derive(Copy, Clone, Debug)]
pub struct VertexArray {
	pub position: [f32; 3],
	pub normal: [f32; 3],
	pub uv: [f32; 2],
}

implement_vertex!(VertexArray, position, normal, uv);

#[derive(Debug)]
#[repr(C)]
pub struct VertexIndex(pub u32, pub u32, pub u32);

#[derive(Debug)]
#[repr(C)]
pub struct Object {
	pub vertexes: Vec<Vertex>,
	pub triangle_indices: Vec<VertexIndex>,
	pub model: glm::Mat4,
}

impl Object {
	#[allow(dead_code)]
	pub fn get_component(&self) -> ModelComponent {
		let mut v = Vec::new();
		for vertex in &self.vertexes {
			v.push(VertexArray {
				position: [vertex.position.x, vertex.position.y, vertex.position.z],
				normal: [vertex.normal.x, vertex.normal.y, vertex.normal.z],
				uv: [vertex.uv.x, vertex.uv.y,],
			});
		}
		ModelComponent{ vertices: v, indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)}
	}
}

impl Default for Object {
	fn default() -> Self {
		Self {
			vertexes: Vec::new(),
			triangle_indices: Vec::new(),
			model: glm::mat4(
				1.0, 0.0, 0.0, 0.0, //
				0.0, 1.0, 0.0, 0.0, //
				0.0, 0.0, 1.0, 0.0, //
				0.0, 0.0, 0.0, 1.0, //
			),
		}
	}
}