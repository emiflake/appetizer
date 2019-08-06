extern crate gl;
extern crate glutin;
#[macro_use]
extern crate glium;

extern crate image;

#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

extern crate nalgebra;
extern crate nalgebra_glm as glm;

extern crate specs;
#[macro_use]
extern crate specs_derive;

extern crate shred;
extern crate shred_derive;

use std::time::Instant;

use std::fs;
use std::io::Cursor;
use std::thread;

mod object;
#[macro_use]
mod macros;
mod obj_parser;
mod profiler;

mod components;
mod resources;
mod systems;

mod world;

use specs::prelude::*;

use components::*;
use resources::*;
use systems::*;

use glium::Surface;

use imgui_winit_support::{HiDpiMode, WinitPlatform};

use imgui::Context;

const SCR_WIDTH: f64 = 1280.0;
const SCR_HEIGHT: f64 = 720.0;

/* Holy what the heck! */

pub fn main() -> Result<(), String> {
	let mut event_loop = glutin::EventsLoop::new();
	let wb = glutin::WindowBuilder::new()
		.with_dimensions(glutin::dpi::LogicalSize::new(SCR_WIDTH, SCR_HEIGHT))
		.with_title("Appetizer");
	let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	gl::load_with(|s| display.gl_window().get_proc_address(&s) as _);

	let mut world = world::create_world()?;
	let mut dispatcher = DispatcherBuilder::new()
		// .with_thread_local(render_sys::RenderSystem)
		.with(logger_sys::LoggerSystem, "logger_system", &[])
		.with(camera_sys::CameraSystem, "camera_system", &[])
		// .with(input_sys::InputSystem, "input_system", &[])
		.build();
	dispatcher.setup(&mut world);

	let mut imgui = Context::create();
	imgui.set_ini_filename(None);

	let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display).unwrap();

	let mut platform = WinitPlatform::init(&mut imgui);
	{
		let gl_window = display.gl_window();
		let window = gl_window.window();
		platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
	}

	let vertex_shader = fs::read_to_string("./vertex.vs").expect("Can't read vertex shader");
	let fragment_shader = fs::read_to_string("./fragment.fs").expect("Can't read fragment shader");
	let image = image::load(
		Cursor::new(&include_bytes!("../assets/textures/wall.jpg")[..]),
		image::JPEG,
	)
	.unwrap()
	.to_rgba();
	let image_dimensions = image.dimensions();
	let image =
		glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = glium::texture::Texture2d::new(&display, image).unwrap();

	let program =
		glium::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();

	let mut profiler = profiler::Profiler::new(100);

	let mut last_frame = Instant::now();
	let mut closed = false;
	while !closed {
		let gl_window = display.gl_window();
		let window = gl_window.window();

		let now = Instant::now();
		let delta = now - last_frame;
		let delta_time = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
		profiler.record_delay(delta_time);
		{
			let mut delta = world.write_resource::<delta_time::DeltaTime>();
			*delta = delta_time::DeltaTime(delta_time);
		}

		// EVENT LOOP
		{
			let mut key_state = world.write_resource::<key_state::Keystate>();
			let mut mouse_state = world.write_resource::<mouse_state::MouseState>();
			mouse_state.update_delta();

			event_loop.poll_events(|event| {
				platform.handle_event(imgui.io_mut(), &window, &event);
				mouse_state.handle_event(&event);
				key_state.handle_event(&event);
				match event {
					glutin::Event::WindowEvent { event, .. } => match event {
						glutin::WindowEvent::CloseRequested => closed = true,
						_ => (),
					},
					_ => (),
				}
			});
		}
		let size = window.get_inner_size().expect("Could not get window size");
		{
			let camera = world.read_resource::<camera::Camera>();
			let mut projection = world.write_resource::<projection::Projection>();
			projection.0 = glm::perspective(
				(size.width as f32) / (size.height as f32),
				camera.zoom,
				0.1,
				10000.0,
			);
		}
		dispatcher.dispatch(&world);

		// IMGUI PREPARE
		let io = imgui.io_mut();
		last_frame = io.update_delta_time(last_frame);
		let mut ui = imgui.frame();
		profiler.draw_ui(delta_time, &mut ui);

		let mut target = display.draw();
		target.clear_color_srgb_and_depth((0.0, 0.0, 0.0, 1.0), 24.0);
		// SCENE RENDER
		{
			let trans = world.read_component::<transformation::TransformationComponent>();
			let models = world.read_component::<model::ModelComponent>();
			let materials = world.read_component::<material::MaterialComponent>();
			let camera = world.read_resource::<camera::Camera>();
			let projection = world.read_resource::<projection::Projection>();

			for (trans, model, material) in (&trans, &models, &materials).join() {
				let uniforms = uniform! {
					camera: *camera.get_view_matrix().as_ref(),
					projection: *projection.0.as_ref(),
					model: *trans.0.as_ref(),
					our_texture: &texture,
				};
				let vertex_buffer = glium::VertexBuffer::new(&display, &model.vertices).unwrap();
				let params = glium::DrawParameters {
					depth: glium::Depth {
						test: glium::draw_parameters::DepthTest::IfLess,
						write: true,
						..Default::default()
					},
					..Default::default()
				};
				target
					.draw(&vertex_buffer, &model.indices, &program, &uniforms, &params)
					.unwrap();
			}
		}

		// IMGUI RENDER
		let draw_data = ui.render();
		renderer.render(&mut target, draw_data).unwrap();
		target.finish().expect("Failed to swap buffers");

		thread::sleep(std::time::Duration::from_millis(16));
	}
	Ok(())
}
