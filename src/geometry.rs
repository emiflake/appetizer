pub struct Vec3 {
	x: f32,
	y: f32,
	z: f32,
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}
}

pub struct Triangle<T> {
	a: T,
	b: T,
	c: T,
}
