extern crate gl;
extern crate glfw;

use self::gl::types::*;
use self::glfw::{Action, Context, Key};
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

mod shader;
use shader::Shader;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main() {
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

	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
	let shader = Shader::new("vertex.vs", "fragment.fs").unwrap();

	let (vao, _ebo) = unsafe {
		// set up vertex data (and buffer(s)) and configure vertex attributes
		// ------------------------------------------------------------------
		// HINT: type annotation is crucial since default for float literals is f64
		let vertices: [f32; 12] = [
			0.5, 0.5, 0.0, // top right
			0.5, -0.5, 0.0, // bottom right
			-0.5, -0.5, 0.0, // bottom left
			-0.5, 0.5, 0.0, // top left
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

		gl::VertexAttribPointer(
			0,
			3,
			gl::FLOAT,
			gl::FALSE,
			3 * mem::size_of::<GLfloat>() as GLsizei,
			ptr::null(),
		);
		gl::EnableVertexAttribArray(0);

		// note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);

		// You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
		// vaos requires a call to glBindVertexArray anyways so we generally don't unbind vaos (nor vbos) when it's not directly necessary.
		gl::BindVertexArray(0);

		// uncomment this call to draw in wireframe polygons.
		// gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

		(vao, ebo)
	};

	while !window.should_close() {
		process_events(&mut window, &events);

		unsafe {
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
			shader.use_program();

			gl::BindVertexArray(vao);
			gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
		}

		window.swap_buffers();
		glfw.poll_events();
	}
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
