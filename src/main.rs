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

use std::fs;
use std::io::Cursor;
use std::thread;
use std::time::Instant;

mod object;
#[macro_use]
mod macros;
mod components;
mod obj_parser;
mod profiler;
mod resource_store;
mod resources;
mod systems;
mod world;

use components::*;
use glium::Surface;
use imgui::Context;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use resources::*;
use specs::prelude::*;
use systems::*;

const SCR_WIDTH: f64 = 1280.0;
const SCR_HEIGHT: f64 = 720.0;

pub fn main() -> Result<(), String> {
	let mut event_loop = glutin::EventsLoop::new();
	let wb = glutin::WindowBuilder::new()
		.with_dimensions(glutin::dpi::LogicalSize::new(SCR_WIDTH, SCR_HEIGHT))
		.with_title("Appetizer");
	let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

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

	// PBR TESTING
	println!("Reading PBR textures");
	// BASE COLOR
	let image = image::load(
		Cursor::new(&include_bytes!("../assets/textures/bricks/albedo.png")[..]),
		image::ImageFormat::PNG,
	)
	.unwrap()
	.to_rgba();
	let image_dimensions = image.dimensions();
	let image =
		glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture_base = glium::texture::Texture2d::new(&display, image).unwrap();
	println!("Read base color");

	// METALLIC
	let image = image::load(
		Cursor::new(&include_bytes!("../assets/textures/bricks/metallic.png")[..]),
		image::ImageFormat::PNG,
	)
	.unwrap()
	.to_rgba();
	let image_dimensions = image.dimensions();
	let image =
		glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture_metallic = glium::texture::Texture2d::new(&display, image).unwrap();
	println!("Read metallic color");

	// NORMAL
	let image = image::load(
		Cursor::new(&include_bytes!("../assets/textures/bricks/normal.png")[..]),
		image::ImageFormat::PNG,
	)
	.unwrap()
	.to_rgba();
	let image_dimensions = image.dimensions();
	let image =
		glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture_normal = glium::texture::Texture2d::new(&display, image).unwrap();
	println!("Read normal color");

	// ROUGHNESS
	let image = image::load(
		Cursor::new(&include_bytes!("../assets/textures/bricks/roughness.png")[..]),
		image::ImageFormat::PNG,
	)
	.unwrap()
	.to_rgba();
	let image_dimensions = image.dimensions();
	let image =
		glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture_roughness = glium::texture::Texture2d::new(&display, image).unwrap();
	println!("Read roughness color");

	// AMBIENT OCCLUSION
	let image = image::load(
		Cursor::new(&include_bytes!("../assets/textures/bricks/ambient_occlusion.png")[..]),
		image::ImageFormat::PNG,
	)
	.unwrap()
	.to_rgba();
	let image_dimensions = image.dimensions();
	let image =
		glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture_ao = glium::texture::Texture2d::new(&display, image).unwrap();
	println!("Read roughness color");

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
				if let glutin::Event::WindowEvent {
					event: glutin::WindowEvent::CloseRequested,
					..
				} = event
				{
					closed = true;
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

		let mut target = display.draw();
		target.clear_color_srgb_and_depth((0.1, 0.1, 0.1, 1.0), 24.0);
		// SCENE RENDER
		{
			let trans = world.read_component::<transformation::TransformationComponent>();
			let models = world.read_component::<model::ModelComponent>();
			let materials = world.read_component::<material::MaterialComponent>();
			let lights = world.read_component::<light::LightComponent>();
			let camera = world.read_resource::<camera::Camera>();
			let projection = world.read_resource::<projection::Projection>();

			let mut light_pos = glm::vec3(0.0, 0.0, 0.0);
			let mut light_color = glm::vec3(0.0, 0.0, 0.0);
			for (trans, light) in (&trans, &lights).join() {
				light_pos = trans.get_pos();
				light_color = light.color;
			}

			for (trans, model, _material) in (&trans, &models, &materials).join() {
				let uniforms = uniform! {
					camera: *camera.get_view_matrix().as_ref(),
					projection: *projection.0.as_ref(),
					model: *trans.0.as_ref(),
					light_pos: *light_pos.as_ref(),
					light_color: *light_color.as_ref(),
					// PBR WORKFLOW UNIFORMS
					tex_base: &texture_base,
					tex_ao: &texture_ao,
					tex_normal: &texture_normal,
					tex_rough: &texture_roughness,
					tex_metal: &texture_metallic,
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

		let io = imgui.io_mut();
		last_frame = io.update_delta_time(last_frame);

		let mut ui = imgui.frame();
		profiler.draw_ui(delta_time, &mut ui);

		let draw_data = ui.render();
		renderer.render(&mut target, draw_data).unwrap();
		target.finish().expect("Failed to swap buffers");

		thread::sleep(std::time::Duration::from_millis(16));
	}
	Ok(())
}
