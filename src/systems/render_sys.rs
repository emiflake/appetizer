use crate::components::{
	light::{Light, LightComponent},
	material::MaterialComponent,
	model::ModelComponent,
	shader::ShaderComponent,
	texture::GLTextureComponent,
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
	pub material: ReadStorage<'a, MaterialComponent>,
	pub lights: ReadStorage<'a, LightComponent>,
}

impl<'a> System<'a> for RenderSystem {
	type SystemData = RenderData<'a>;

	fn run(&mut self, render_data: Self::SystemData) {
		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}

		let camera_mat = render_data.camera.get_view_matrix();

		for (trans, model, shader, texture, material) in (
			&render_data.trans,
			&render_data.model,
			&render_data.shader,
			&render_data.texture,
			&render_data.material,
		)
			.join()
		{
			unsafe {
				if let Some(handle) = render_data.gltexture_map.get_texture(texture.0) {
					shader.use_program();
					shader.set_mat4(c_str!("model"), &trans.0);
					shader.set_mat4(c_str!("projection"), &render_data.projection.0);

					for (trans, light) in (&render_data.trans, &render_data.lights).join() {
						let pos = trans.get_pos();

						match light.0 {
							Light::PointLight {
								ambient,
								diffuse,
								specular,
								constant,
								linear,
								quadratic,
								..
							} => {
								shader.set_vector3(c_str!("point_light.position"), &pos);
								shader.set_vector3(c_str!("point_light.ambient"), &ambient);
								shader.set_vector3(c_str!("point_light.diffuse"), &diffuse);
								shader.set_vector3(c_str!("point_light.specular"), &specular);
								shader.set_float(c_str!("point_light.constant"), constant);
								shader.set_float(c_str!("point_light.linear"), linear);
								shader.set_float(c_str!("point_light.quadratic"), quadratic);
							}
						}
					}

					shader.set_vector3(c_str!("material.ambient"), &material.ambient);
					shader.set_vector3(c_str!("material.diffuse"), &material.diffuse);
					shader.set_vector3(c_str!("material.specular"), &material.specular);
					shader.set_float(c_str!("material.shininess"), material.shininess);

					shader.set_mat4(c_str!("camera"), &camera_mat);
					shader.set_vector3(c_str!("camera_pos"), &render_data.camera.position);

					gl::BindTexture(gl::TEXTURE_2D, handle);

					gl::BindVertexArray(model.vao);
					gl::DrawArrays(gl::TRIANGLES, 0, model.length as i32);
				}
			}
		}
	}
}
