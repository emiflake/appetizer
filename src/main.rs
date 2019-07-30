extern crate gl;
extern crate glfw;
extern crate image;

extern crate nalgebra;
extern crate nalgebra_glm as glm;

extern crate specs;
#[macro_use]
extern crate specs_derive;

use self::glfw::{Action, Context, Key};
use std::sync::mpsc::Receiver;

#[macro_use]
mod object;
mod macros;

mod components;
mod resources;
mod systems;

use specs::prelude::*;

use components::*;
use resources::*;
use systems::*;

const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

pub fn main() -> Result<(), String> {
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
	glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
	glfw.window_hint(glfw::WindowHint::OpenGlProfile(
		glfw::OpenGlProfileHint::Core,
	));
	#[cfg(target_os = "macos")]
	glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

	let (mut window, events) = glfw
		.create_window(
			SCR_WIDTH,
			SCR_HEIGHT,
			"Appetizer",
			glfw::WindowMode::Windowed,
		)
		.expect("Failed to create GLFW window");

	window.make_current();
	window.set_key_polling(true);
	window.set_framebuffer_size_polling(true);

	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	unsafe {
		gl::Enable(gl::DEPTH_TEST);
	}

	// Initialized everything

	let mut world: World = World::new();
	world.register::<transformation::TransformationComponent>();
	world.register::<model::ModelComponent>();
	world.register::<name::NameComponent>();
	world.register::<texture::TextureComponent>();

	world.insert(delta_time::DeltaTime(0.0));
	world.insert(keystate::Keystate::default());
	world.insert(camera::Camera::default());
	world.insert(texture_map::TextureMap::new());
	world.insert(texture_map::GLTextureMap::new());

	{
		let mut texture_map = world.write_resource::<texture_map::TextureMap>();
		texture_map.load_from_file("textures/wall.jpg".to_string())?;
	}

	world
		.create_entity()
		.with(transformation::TransformationComponent(glm::mat4(
			0.0, 0.0, 0.0, 0.0, //
			0.0, 0.0, 0.0, 0.0, //
			0.0, 0.0, 0.0, 0.0, //
			0.0, 0.0, 0.0, 0.0, //
		)))
		.with(name::NameComponent("Alpha".to_string()))
		.build();

	let mut dispatcher = DispatcherBuilder::new()
		.with_thread_local(render::RenderSystem)
		.with(logger::LoggerSystem, "logger_system", &[])
		.with(gravity::GravitySystem, "gravity_system", &[])
		.build();
	dispatcher.setup(&mut world);

	let mut last_frame = glfw.get_time();

	while !window.should_close() {
		let current_time = glfw.get_time();
		let delta_time = current_time - last_frame;
		{
			let mut delta = world.write_resource::<delta_time::DeltaTime>();
			*delta = delta_time::DeltaTime(delta_time);
		}
		last_frame = current_time;
		dispatcher.dispatch(&world);

		process_events(&mut window, &events);
		unsafe {
			gl::ClearColor(1.0, 0.5, 0.3, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}

		window.swap_buffers();
		glfw.poll_events();
	}

	Ok(())
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
	for (_, event) in glfw::flush_messages(events) {
		match event {
			glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
				gl::Viewport(0, 0, width, height)
			},
			glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
				window.set_should_close(true)
			}
			_ => {}
		}
	}
}
