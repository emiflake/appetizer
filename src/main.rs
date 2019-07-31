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
	window.set_mouse_button_polling(true);
	window.set_framebuffer_size_polling(true);

	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	let shader = shader::ShaderComponent::new("vertex.vs", "fragment.fs")
		.map_err(|e| format!("Shader error: {:?}", e))?;

	let teapot = obj_parser::parse("objs/teapot.obj".to_string())
		.map_err(|e| format!("Parser error: {:?}", e))?;
	let cube = obj_parser::parse("objs/cube.obj".to_string())
		.map_err(|e| format!("Parser error: {:?}", e))?;

	unsafe {
		gl::Enable(gl::DEPTH_TEST);
	}

	// Initialized everything
	let mut world: World = World::new();

	world.register::<transformation::TransformationComponent>();
	world.register::<model::ModelComponent>();
	world.register::<name::NameComponent>();
	world.register::<texture::GLTextureComponent>();
	world.register::<shader::ShaderComponent>();
	world.register::<material::MaterialComponent>();
	world.register::<light::LightComponent>();

	world.insert(delta_time::DeltaTime(0.0));
	world.insert(key_state::Keystate::default());
	world.insert(camera::Camera::default());
	world.insert(texture_map::TextureMap::new());
	world.insert(texture_map::GLTextureMap::new());
	world.insert(projection::Projection::default());
	world.insert(mouse_state::MouseState::default());
	world.insert(time::CurrentTime::default());

	{
		let mut camera = world.write_resource::<camera::Camera>();
		camera.update_camera_vectors();
	}
	let white_texture_handle = {
		let mut texture_map = world.write_resource::<texture_map::TextureMap>();
		texture_map.load_from_file("textures/white.png".to_string())?
	};
	let white_gltexture_handle = {
		let texture_map = world.read_resource::<texture_map::TextureMap>();
		let mut gltexture_map = world.write_resource::<texture_map::GLTextureMap>();

		gltexture_map.load_from_map(&texture_map, white_texture_handle)?
	};
	let wall_texture_handle = {
		let mut texture_map = world.write_resource::<texture_map::TextureMap>();
		texture_map.load_from_file("textures/wall.jpg".to_string())?
	};
	let wall_gltexture_handle = {
		let texture_map = world.read_resource::<texture_map::TextureMap>();
		let mut gltexture_map = world.write_resource::<texture_map::GLTextureMap>();

		gltexture_map.load_from_map(&texture_map, wall_texture_handle)?
	};

	world
		.create_entity()
		.with(transformation::TransformationComponent(glm::mat4(
			1.0, 0.0, 0.0, 0.0, //
			0.0, 1.0, 0.0, 0.0, //
			0.0, 0.0, 1.0, 0.0, //
			0.0, 0.0, 0.0, 1.0, //
		)))
		.with(material::MaterialComponent {
			ambient: glm::vec3(0.1, 0.1, 0.1),
			diffuse: glm::vec3(0.5, 0.5, 0.5),
			specular: glm::vec3(0.8, 0.8, 0.8),
			shininess: 32.0,
		})
		.with(shader)
		.with(teapot.get_component())
		.with(texture::GLTextureComponent(wall_gltexture_handle))
		.with(name::NameComponent("Alpha".to_string()))
		.build();

	world
		.create_entity()
		.with(transformation::TransformationComponent::from_pos(
			glm::vec3(0.0, 1000.0, 0.0),
		))
		.with(shader)
		.with(material::MaterialComponent {
			ambient: glm::vec3(1.0, 0.5, 0.5),
			diffuse: glm::vec3(0.0, 0.0, 0.0),
			specular: glm::vec3(0.0, 0.0, 0.0),
			shininess: 32.0,
		})
		.with(cube.get_component())
		.with(texture::GLTextureComponent(white_gltexture_handle))
		.with(light::LightComponent(light::Light::PointLight {
			ambient: glm::vec3(1.0, 1.0, 1.0),
			diffuse: glm::vec3(1.0, 1.0, 1.0),
			specular: glm::vec3(1.0, 1.0, 1.0),
			constant: 1.0,
			linear: 0.09,
			quadratic: 0.032,
		}))
		.with(name::NameComponent("Random Light".to_string()))
		.build();

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
