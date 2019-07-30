use crate::components::{
	model::ModelComponent, shader::ShaderComponent, texture::GLTextureComponent,
	transformation::TransformationComponent,
};

use crate::resources::{camera::Camera, projection::Projection, texture_map::GLTextureMap};
use specs::prelude::*;
use specs::Join;
use std::ffi::CStr;

pub struct RenderSystem;

#[derive(SystemData)]
pub struct RenderData<'a> {
	pub trans: ReadStorage<'a, TransformationComponent>,
	pub model: ReadStorage<'a, ModelComponent>,
	pub texture: ReadStorage<'a, GLTextureComponent>,
	pub shader: ReadStorage<'a, ShaderComponent>,
	pub gltexture_map: Read<'a, GLTextureMap>,
	pub camera: Read<'a, Camera>,
	pub projection: Read<'a, Projection>,
}

impl<'a> System<'a> for RenderSystem {
	type SystemData = RenderData<'a>;

	fn run(&mut self, render_data: Self::SystemData) {
		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
		for (trans, model, shader, texture) in (
			&render_data.trans,
			&render_data.model,
			&render_data.shader,
			&render_data.texture,
		)
			.join()
		{
			unsafe {
				if let Some(handle) = render_data.gltexture_map.get_texture(texture.0) {
					shader.use_program();
					shader.set_mat4(c_str!("model"), &trans.0);
					shader.set_mat4(c_str!("projection"), &render_data.projection.0);

					shader.set_vec3(c_str!("light_pos"), 0.0, 50.0, 50.0);
					shader.set_vec3(c_str!("light_color"), 1.0, 1.0, 1.0);

					shader.set_vector3(c_str!("camera_pos"), &render_data.camera.position);
					shader.set_vector3(
						c_str!("camera_tgt"),
						&(render_data.camera.front + render_data.camera.position),
					);
					shader.set_vector3(c_str!("camera_up"), &render_data.camera.up);

					gl::BindTexture(gl::TEXTURE_2D, handle);

					gl::BindVertexArray(model.vao);
					gl::DrawArrays(gl::TRIANGLES, 0, model.length as i32);
				}
			}
		}
	}
}
