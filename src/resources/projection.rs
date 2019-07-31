pub struct Projection(pub glm::Mat4);

impl Default for Projection {
	fn default() -> Self {
		Self(glm::mat4(
			1.0, 0.0, 0.0, 0.0, //
			0.0, 1.0, 0.0, 0.0, //
			0.0, 0.0, 1.0, 0.0, //
			0.0, 0.0, 0.0, 1.0, //
		))
	}
}
