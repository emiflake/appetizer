use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::object::{Object, Vertex, VertexIndex};

#[derive(Debug)]
pub enum ParseError {
	FileReadError,
	LineReadError,
	StrParseError,
	MalformedVertex(usize),
}

pub fn str_to_float(s: &str) -> Result<f32, ParseError> {
	s.parse::<f32>().map_err(|_| ParseError::StrParseError)
}

pub fn parse(path: String) -> Result<Object, ParseError> {
	let file = File::open(path).map_err(|_| ParseError::FileReadError)?;
	let reader = BufReader::new(file);

	let mut positions: Vec<glm::Vec3> = Vec::new();
	let mut normals: Vec<glm::Vec3> = Vec::new();
	let mut uvs: Vec<glm::Vec2> = Vec::new();
	let mut obj = Object::default();

	for (line_no, line) in reader.lines().enumerate() {
		let unwrapped_line = line.map_err(|_| ParseError::LineReadError)?;
		let line_bits: Vec<&str> = unwrapped_line.split_whitespace().collect();

		if line_bits.is_empty() {
			// Empty line; skip
			continue;
		}

		match line_bits[0usize] {
			"v" => {
				positions.push(glm::vec3(
					str_to_float(line_bits[1usize])?,
					str_to_float(line_bits[2usize])?,
					str_to_float(line_bits[3usize])?,
				));
			}
			"vn" => {
				normals.push(glm::vec3(
					str_to_float(line_bits[1usize])?,
					str_to_float(line_bits[2usize])?,
					str_to_float(line_bits[3usize])?,
				));
			}
			"vt" => {
				uvs.push(glm::vec2(
					str_to_float(line_bits[1usize])?,
					str_to_float(line_bits[2usize])?,
				));
			}

			"f" => {
				let mut vertex_idxs = Vec::new();
				for vfinder in line_bits[1usize..].iter() {
					let data: Vec<&str> = vfinder.split('/').collect();
					if data.len() == 3 {
						if let [pos, uv, norm] = data.as_slice() {
							let vertex = Vertex {
								position: positions[pos
									.parse::<usize>()
									.map_err(|_| ParseError::MalformedVertex(line_no))?
									- 1],
								uv: uvs[uv
									.parse::<usize>()
									.map_err(|_| ParseError::MalformedVertex(line_no))?
									- 1],
								normal: normals[norm
									.parse::<usize>()
									.map_err(|_| ParseError::MalformedVertex(line_no))?
									- 1],
							};
							obj.vertexes.push(vertex);
							vertex_idxs.push(obj.vertexes.len() as u32 - 1);
						}
					} else {
						return Err(ParseError::MalformedVertex(line_no));
					}
				}
				if vertex_idxs.len() != 3 {
					unreachable!();
				}
				obj.triangle_indices.push(VertexIndex(
					vertex_idxs[0],
					vertex_idxs[1],
					vertex_idxs[2],
				));
			}
			_ => {}
		}
	}

	Ok(obj)
}
