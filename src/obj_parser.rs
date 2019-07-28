use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::object::Object;

pub enum ParseError {
	FileReadError,
	LineReadError,
	StrParseError,
}

pub fn str_to_float(s: &str) -> Result<f32, ParseError> {
	s.parse::<f32>().map_err(|_| ParseError::StrParseError)
}

pub fn parse(path: String) -> Result<Object, ParseError> {
	let file = File::open(path).map_err(|_| ParseError::FileReadError)?;
	let reader = BufReader::new(file);

	let mut obj = Object::default();

	for line in reader.lines() {
		let unwrapped_line = line.map_err(|_| ParseError::LineReadError)?;
		let line_bits: Vec<&str> = unwrapped_line.split_whitespace().collect();

		if line_bits.len() < 1 {
			continue;
		}

		match line_bits[0usize] {
			"v" => {
				obj.positions.push(glm::vec3(
					str_to_float(line_bits[1usize])?,
					str_to_float(line_bits[2usize])?,
					str_to_float(line_bits[3usize])?,
				));
			}
			"vn" => {
				obj.normals.push(glm::vec3(
					str_to_float(line_bits[1usize])?,
					str_to_float(line_bits[2usize])?,
					str_to_float(line_bits[3usize])?,
				));
			}
			"vt" => {
				obj.uvs.push(glm::vec2(
					str_to_float(line_bits[1usize])?,
					str_to_float(line_bits[2usize])?,
				));
			}

			"f" => {
				//   1/2/3 4/5/6 7/8/9

				let fs: Vec<Vec<Result<usize, ParseError>>> = line_bits[1usize..]
					.iter()
					.map(|bit| {
						bit.split("/")
							.collect::<Vec<&str>>()
							.iter()
							.map(|i| i.parse::<usize>().map_err(|_| ParseError::StrParseError))
					})
					.collect();

					/* 1/2/3 4/5/6 7/8/9 */
				if let [a, b, c] = fs {
					obj.triangle_indices.push(a[0], b[0], c[0]);
				}
			}
			_ => {}
		}
	}

	unimplemented!()
}

/*
1/2/3 4/5/6 7/8/9

[
	[1,2,3],
	[4,5,6],
	[7,8,9]
]

 */
