extern crate gl;
extern crate glfw;
extern crate image;

extern crate nalgebra;
extern crate nalgebra_glm as glm;

use self::gl::types::*;
use self::glfw::{Action, Context, Key};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

mod shader;
mod texture_map;
use shader::Shader;

use image::GenericImageView;

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

	// glfw window creation
	// --------------------
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

	let mut texture_map = texture_map::TextureMap::new();

	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
	let shader = Shader::new("vertex.vs", "fragment.fs").unwrap();

	let wall = image::open("textures/wall.jpg").expect("Could not load textures/wall.jpg");
	let wall_data = wall.raw_pixels();

	let (vao, texture) = unsafe {
		// set up vertex data (and buffer(s)) and configure vertex attributes
		// ------------------------------------------------------------------
		// HINT: type annotation is crucial since default for float literals is f64
		let vertices: [f32; 20] = [
			0.5, 0.5, 0.0, 1.0, 1.0, // top right
			0.5, -0.5, 0.0, 1.0, 0.0, // bottom right
			-0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
			-0.5, 0.5, 0.0, 0.0, 1.0, // top left
		];

		let indexes: [u32; 6] = [
			0, 1, 3, // first triangle
			1, 2, 3, // second triangle
		];
		let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
		gl::GenVertexArrays(1, &mut vao);
		gl::GenBuffers(1, &mut vbo);
		gl::GenBuffers(1, &mut ebo);
		// bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
		gl::BindVertexArray(vao);

		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
		gl::BufferData(
			gl::ELEMENT_ARRAY_BUFFER,
			(indexes.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
			&indexes[0] as *const u32 as *const c_void,
			gl::STATIC_DRAW,
		);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
			&vertices[0] as *const f32 as *const c_void,
			gl::STATIC_DRAW,
		);

		let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;

		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
		gl::EnableVertexAttribArray(0);
		gl::VertexAttribPointer(
			1,
			2,
			gl::FLOAT,
			gl::FALSE,
			stride,
			(3 * mem::size_of::<GLfloat>()) as *const c_void,
		);
		gl::EnableVertexAttribArray(1);

		let mut texture = 0;
		gl::GenTextures(1, &mut texture);
		gl::BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
		// set texture filtering parameters
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			gl::RGB as i32,
			wall.width() as i32,
			wall.height() as i32,
			0,
			gl::RGB,
			gl::UNSIGNED_BYTE,
			&wall_data[0] as *const u8 as *const c_void,
		);
		gl::GenerateMipmap(gl::TEXTURE_2D);

		(vao, texture)
	};

	while !window.should_close() {
		process_events(&mut window, &events);

		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
			shader.use_program();

			gl::BindTexture(gl::TEXTURE_2D, texture);

			let time = glfw.get_time() as f32;
			let green = time.sin() / 2.0 + 0.5;
			let our_color = CString::new("our_color").unwrap();
			let vertex_color_loc = gl::GetUniformLocation(shader.get_id(), our_color.as_ptr());
			gl::Uniform1f(vertex_color_loc, green);

			gl::BindVertexArray(vao);
			gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
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
