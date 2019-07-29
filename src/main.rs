extern crate gl;
extern crate glfw;
extern crate image;

extern crate nalgebra;
extern crate nalgebra_glm as glm;

use self::gl::types::*;
use self::glfw::{Action, Context, Key};
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

mod camera;
#[macro_use]
mod macros;
mod obj_parser;
mod object;
mod rasterizable;
mod shader;
mod texture_map;
mod world;

use camera::{Camera, CameraDirection};
use rasterizable::Rasterizable;
use shader::Shader;
use world::World;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main() -> Result<(), String> {
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
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

	let cube = obj_parser::parse("objs/teapot.obj".to_string()).unwrap();

	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
	let shader = Shader::new("vertex.vs", "fragment.fs").unwrap();

	// Preload the texture
	let wall = image::open("textures/wall.jpg").expect("Could not load textures/wall.jpg");
	// Necessary for interaction with OpenGL, prebake it.
	let wall_data = wall.raw_pixels();

	let camera = Camera::new(glm::vec3(0.0, 0.0, 0.0));
	let mut world = World { camera };

	// Necessary to make the world not look dumb.
	unsafe {
		gl::Enable(gl::DEPTH_TEST);
	}

	// Delta checking variables
	let mut last_frame: f32 = 0.0;
	let mut last_pos = (0.0, 0.0);
	while !window.should_close() {
		let current_frame = glfw.get_time() as f32;
		let delta_time = current_frame - last_frame;
		last_frame = current_frame;

		let (mouse_x, mouse_y) = window.get_cursor_pos();
		let (delta_x, delta_y) = (last_pos.0 - mouse_x, last_pos.1 - mouse_y);
		last_pos = (mouse_x, mouse_y);

		process_input(
			&mut window,
			&mut world,
			delta_time,
			(delta_x as f32, delta_y as f32),
		);
		process_events(&mut window, &events);

		let (window_width, window_height) = window.get_size();
		let projection = glm::perspective(
			(window_width as f32) / (window_height as f32),
			world.camera.zoom,
			0.1,
			10000.0,
		);

		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

			use crate::object::ObjSettings;
			let wall_settings = ObjSettings {
				texture: &wall,
				raw_pixels: &wall_data,
				shader: &shader,
			};

			let raster_settings = crate::rasterizable::RasterSettings {
				projection,
				specifics: wall_settings,
			};

			cube.rasterize(&world, &raster_settings);
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

fn process_input(
	window: &mut glfw::Window,
	mut world: &mut World,
	delta_time: f32,
	(delta_x, delta_y): (f32, f32),
) {
	world.camera.speed = if window.get_key(Key::LeftShift) == Action::Press {
		100.0
	} else {
		1.0
	};
	if window.get_key(Key::W) == Action::Press {
		world.camera.do_move(CameraDirection::Forward, delta_time)
	}
	if window.get_key(Key::A) == Action::Press {
		world.camera.do_move(CameraDirection::Left, delta_time)
	}
	if window.get_key(Key::S) == Action::Press {
		world.camera.do_move(CameraDirection::Backward, delta_time)
	}
	if window.get_key(Key::D) == Action::Press {
		world.camera.do_move(CameraDirection::Right, delta_time)
	}

	window.set_cursor_mode(glfw::CursorMode::Disabled);
	world
		.camera
		.do_rotate(glm::vec2(-delta_x / 10.0, delta_y / 10.0));
}
