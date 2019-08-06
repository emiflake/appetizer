#![allow(dead_code)]

use specs::prelude::*;
use glium::program::Program;
use std::sync::Arc;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct ShaderComponent {
	pub program: Arc<Program>
}
