#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;

#[derive(Copy, Clone, Debug)]
pub struct TextureHandle(usize);

pub struct TextureMap {
	pub textures: Vec<image::RgbImage>,
}

impl TextureMap {
	pub fn new() -> Self {
		TextureMap {
			textures: Vec::new(),
		}
	}

	pub fn load_image_from_file(
		&mut self,
		filename: &str,
		format: image::ImageFormat,
	) -> std::result::Result<TextureHandle, String> {
		let mut f = File::open(filename)
			.map_err(|_| format!("Could not open file of image {}	", filename))?;
		let reader = BufReader::new(&mut f);
		let img = image::load(reader, format).map_err(|_| "Could not load image as PNG")?;
		let index = self.textures.len();
		self.textures.push(img.to_rgb());
		Ok(TextureHandle(index))
	}

	pub fn load_image_from_file_png(
		&mut self,
		filename: &str,
	) -> std::result::Result<TextureHandle, String> {
		self.load_image_from_file(filename, image::ImageFormat::PNG)
	}

	pub fn load_image_from_file_jpg(
		&mut self,
		filename: &str,
	) -> std::result::Result<TextureHandle, String> {
		self.load_image_from_file(filename, image::ImageFormat::JPEG)
	}

	pub fn load_image_from_file_bmp(
		&mut self,
		filename: &str,
	) -> std::result::Result<TextureHandle, String> {
		self.load_image_from_file(filename, image::ImageFormat::BMP)
	}

	pub fn get_image_by_handle(
		&self,
		handle: TextureHandle,
	) -> std::result::Result<&image::RgbImage, String> {
		match self.textures.get(handle.0).as_ref() {
			Some(img_ref) => Ok(img_ref),
			None => Err(format!(
				"Could not get image from that handle ({})!",
				handle.0
			)),
		}
	}
}
