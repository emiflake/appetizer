use crate::components::model::ModelComponent;

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
	pub tangent: [f32; 3],
	pub bitangent: [f32; 3],
}

implement_vertex!(VertexArray, position, normal, uv, tangent, bitangent);

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
				uv: [vertex.uv.x, vertex.uv.y],
				tangent: [0.0; 3],
				bitangent: [0.0; 3],
			});
		}

		for triangle_index in &self.triangle_indices {
			let i = (
				triangle_index.0 as usize,
				triangle_index.1 as usize,
				triangle_index.2 as usize,
			);
			let (a, b, c) = (
				&self.vertexes[i.0],
				&self.vertexes[i.1],
				&self.vertexes[i.2],
			);

			let edge1 = b.position - a.position;
			let edge2 = c.position - a.position;
			let delta_uv1 = b.uv - a.uv;
			let delta_uv2 = c.uv - a.uv;

			let f = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv2.x * delta_uv1.y);

			let tangent = glm::vec3(
				f * (delta_uv2.y * edge1.x - delta_uv1.y * edge2.x),
				f * (delta_uv2.y * edge1.y - delta_uv1.y * edge2.y),
				f * (delta_uv2.y * edge1.z - delta_uv1.y * edge2.z),
			)
			.normalize();

			let bitangent = glm::vec3(
				f * (-delta_uv2.y * edge1.x + delta_uv1.y * edge2.x),
				f * (-delta_uv2.y * edge1.y + delta_uv1.y * edge2.y),
				f * (-delta_uv2.y * edge1.z + delta_uv1.y * edge2.z),
			)
			.normalize();

			v[i.0].tangent = [tangent.x, tangent.y, tangent.z];
			v[i.1].tangent = [tangent.x, tangent.y, tangent.z];
			v[i.2].tangent = [tangent.x, tangent.y, tangent.z];
			v[i.0].bitangent = [bitangent.x, bitangent.y, bitangent.z];
			v[i.1].bitangent = [bitangent.x, bitangent.y, bitangent.z];
			v[i.2].bitangent = [bitangent.x, bitangent.y, bitangent.z];
		}

		ModelComponent {
			vertices: v,
			indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
		}
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
