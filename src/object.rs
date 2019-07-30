use gl::types::*;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use crate::components::model::ModelComponent;

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
	pub position: glm::Vec3,
	pub normal: glm::Vec3,
	pub uv: glm::Vec2,
}

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
	fn get_indices(&self) -> Vec<u32> {
		let mut v = Vec::new();
		for i in &self.triangle_indices {
			v.push(i.0);
			v.push(i.1);
			v.push(i.2);
		}
		v
	}
	#[allow(dead_code)]
	fn get_vertices(&self) -> Vec<f32> {
		let mut v = Vec::new();
		for vertex in &self.vertexes {
			v.push(vertex.position.x);
			v.push(vertex.position.y);
			v.push(vertex.position.z);
			v.push(vertex.normal.x);
			v.push(vertex.normal.y);
			v.push(vertex.normal.z);
			v.push(vertex.uv.x);
			v.push(vertex.uv.y);
		}
		v
	}
	pub fn get_component(&self) -> ModelComponent {
		unsafe {
			let vertices = self.get_vertices();

			let (mut vbo, mut vao) = (0, 0);
			gl::GenVertexArrays(1, &mut vao);
			gl::GenBuffers(1, &mut vbo);

			gl::BindVertexArray(vao);

			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
				&vertices[0] as *const f32 as *const c_void,
				gl::STATIC_DRAW,
			);

			// Dependent on the size of each vertex, currently: pos(3) + normal(3) = uv(2) = 8
			let stride_elems = 8usize;
			let stride = (stride_elems as i32) * mem::size_of::<GLfloat>() as GLsizei;

			gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(
				1,
				3,
				gl::FLOAT,
				gl::FALSE,
				stride,
				(3 * mem::size_of::<GLfloat>()) as *const c_void,
			);
			gl::EnableVertexAttribArray(1);
			gl::VertexAttribPointer(
				2,
				2,
				gl::FLOAT,
				gl::FALSE,
				stride,
				(6 * mem::size_of::<GLfloat>()) as *const c_void,
			);
			gl::EnableVertexAttribArray(2);

			ModelComponent { vao }
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



// 			// Texture portion
// 			let mut texture = 0; // Texture handle
// 			let texture_img = raster_settings.specifics.texture;
// 			let texture_raw = raster_settings.specifics.raw_pixels;
// 			gl::GenTextures(1, &mut texture);
// 			gl::BindTexture(gl::TEXTURE_2D, texture);
// 			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
// 			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
// 			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
// 			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
// 			gl::TexImage2D(
// 				gl::TEXTURE_2D,
// 				0,
// 				gl::RGB as i32,
// 				texture_img.width() as i32,
// 				texture_img.height() as i32,
// 				0,
// 				gl::RGB,
// 				gl::UNSIGNED_BYTE,
// 				&texture_raw[0] as *const u8 as *const c_void,
// 			);
// 			// https://en.wikipedia.org/wiki/Mipmap
// 			// OpenGL does it automatically for us
// 			// TODO: Precompute it
// 			gl::GenerateMipmap(gl::TEXTURE_2D);

// 			// Render portion
// 			gl::BindTexture(gl::TEXTURE_2D, texture);

// 			gl::BindVertexArray(vao);
// 			gl::DrawArrays(
// 				gl::TRIANGLES,
// 				0,
// 				(vertices.len() / stride_elems).try_into().unwrap(),
// 			);
