extern crate gl;
extern crate glfw;
extern crate image;

extern crate nalgebra;
extern crate nalgebra_glm as glm;

extern crate specs;
#[macro_use]
extern crate specs_derive;

extern crate shred;
#[macro_use]
extern crate shred_derive;

use self::glfw::{Action, Context, Key};
use std::sync::mpsc::Receiver;

use std::ffi::CStr;
use std::thread;

mod object;
#[macro_use]
mod macros;
mod obj_parser;

mod components;
mod resources;
mod systems;

mod world;

use specs::prelude::*;

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
	window.set_mouse_button_polling(true);
	window.set_framebuffer_size_polling(true);

	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	unsafe {
		gl::Enable(gl::DEPTH_TEST);
	}

	let mut world = world::create_world()?;

	let mut dispatcher = DispatcherBuilder::new()
		.with_thread_local(render_sys::RenderSystem)
		.with(logger_sys::LoggerSystem, "logger_system", &[])
		.with(camera_sys::CameraSystem, "camera_system", &[])
		.with(input_sys::InputSystem, "input_system", &[])
		.build();

	dispatcher.setup(&mut world);

	let mut last_frame = glfw.get_time();

	let mut last_pos = (0.0, 0.0);

	while !window.should_close() {
		let current_time = glfw.get_time();
		let delta_time = (current_time - last_frame) as f32;
		{
			let mut delta = world.write_resource::<delta_time::DeltaTime>();
			*delta = delta_time::DeltaTime(delta_time);
		}
		{
			let mut t = world.write_resource::<time::CurrentTime>();
			*t = time::CurrentTime(current_time);
		}
		last_frame = current_time;

		let (mouse_x, mouse_y) = window.get_cursor_pos();
		let (delta_x, delta_y) = (last_pos.0 - mouse_x, last_pos.1 - mouse_y);
		last_pos = (mouse_x, mouse_y);
		{
			let mut mouse_state = world.write_resource::<mouse_state::MouseState>();
			mouse_state.position = glm::vec2(mouse_x as f32, mouse_y as f32);
			mouse_state.delta = glm::vec2(delta_x as f32, delta_y as f32);

			window.set_cursor_mode(if mouse_state.is_locked {
				glfw::CursorMode::Disabled
			} else {
				glfw::CursorMode::Normal
			});

			// Process the key_state for future ussage
			let mut key_state = world.write_resource::<key_state::Keystate>();
			process_events(&mut window, &events, &mut key_state, &mut mouse_state);
		}

		let (window_width, window_height) = window.get_size();
		{
			let camera = world.read_resource::<camera::Camera>();
			let mut projection = world.write_resource::<projection::Projection>();
			projection.0 = glm::perspective(
				(window_width as f32) / (window_height as f32),
				camera.zoom,
				0.1,
				10000.0,
			);
		}

		// Finally, let's dispatch on the world
		dispatcher.dispatch(&world);

		window.swap_buffers();
		thread::sleep(std::time::Duration::from_millis(16));
		glfw.poll_events();
	}

	Ok(())
}

fn process_events(
	window: &mut glfw::Window,
	events: &Receiver<(f64, glfw::WindowEvent)>,
	key_state: &mut key_state::Keystate,
	mouse_state: &mut mouse_state::MouseState,
) {
	for (_, event) in glfw::flush_messages(events) {
		match event {
			glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
				gl::Viewport(0, 0, width, height);
			},
			glfw::WindowEvent::Key(key, _, Action::Release, _) => {
				key_state.set_key_up(key);
			}
			glfw::WindowEvent::Key(key, _, Action::Press, _) => {
				key_state.set_key_down(key);
				if key == Key::Escape {
					// TODO: maybe integrate into some sort of system?
					window.set_should_close(true);
				}
			}
			glfw::WindowEvent::MouseButton(button, Action::Press, _) => {
				mouse_state.set_button_down(button);
			}
			glfw::WindowEvent::MouseButton(button, Action::Release, _) => {
				mouse_state.set_button_up(button);
			}
			_ => {}
		}
	}
}
