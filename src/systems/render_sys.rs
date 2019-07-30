use crate::components::{
	model::ModelComponent, shader::ShaderComponent, texture::GLTextureComponent,
	transformation::TransformationComponent,
};
use specs::prelude::*;
use specs::Join;

pub struct RenderSystem;

#[derive(SystemData)]
struct RenderData<'a> {
	pub trans: ReadStorage<'a, TransformationComponent>,
	pub model: ReadStorage<'a, ModelComponent>,
	pub texture: ReadStorage<'a, GLTextureComponent>,
	pub shader: ReadStorage<'a, ShaderComponent>,
}

impl<'a> System<'a> for RenderSystem {
	type SystemData = RenderData<'a>;

	fn run(&mut self, render_data: Self::SystemData) {
		unsafe {
			gl::ClearColor(1.0, 0.5, 0.3, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
		for render_entity in render_data.join() {
			//
		}
	}
}
