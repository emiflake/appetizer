use std::any::Any;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub enum ResourceLoadError {
	AcquisitionError { path: String },
	TypeMismatchError { path: String },
}

pub trait LoadableResource: Any {
	type LoadError;

	fn load(path: &String) -> Result<Box<Self>, Self::LoadError>;
}

pub struct ResourceStore {
	pub resources: HashMap<String, Box<dyn Any>>,
}

impl ResourceStore {
	pub fn new() -> Self {
		Self {
			resources: HashMap::new(),
		}
	}

	pub fn get<T: LoadableResource>(&mut self, path: String) -> Result<&T, ResourceLoadError> {
		if self.resources.contains_key(&path) {
			println!("Cache hit");
			return match self.resources.get(&path) {
				Some(resource) => resource
					.downcast_ref::<T>()
					.ok_or(ResourceLoadError::TypeMismatchError { path }),
				None => unreachable!(),
			};
		}

		let loaded = T::load(&path)
			.map_err(|_| ResourceLoadError::AcquisitionError { path: path.clone() })?; // TODO
		self.resources.insert(path.clone(), loaded);
		Ok(self
			.resources
			.get(&path)
			.unwrap()
			.downcast_ref::<T>()
			.unwrap())
	}
}

use image::{ImageBuffer, Rgb};
use std::fs::File;
use std::io::BufReader;

impl LoadableResource for ImageBuffer<Rgb<u8>, Vec<u8>> {
	type LoadError = String;

	fn load(path: &String) -> Result<Box<Self>, Self::LoadError> {
		let mut f =
			File::open(path).map_err(|_| format!("Could not open file of image {}	", path))?;
		let reader = BufReader::new(&mut f);
		let img = image::load(reader, image::ImageFormat::PNG)
			.map_err(|_| "Could not load image as PNG")?;

		Ok(Box::new(img.to_rgb()))
	}
}
