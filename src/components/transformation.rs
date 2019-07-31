use glm::Mat4;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TransformationComponent(pub Mat4);

impl TransformationComponent {
	pub fn from_pos(pos: glm::Vec3) -> Self {
		Self(glm::mat4(
			1.0, 0.0, 0.0, pos[0], //
			0.0, 1.0, 0.0, pos[1], //
			0.0, 0.0, 1.0, pos[2], //
			0.0, 0.0, 0.0, 1.0, //
		))
	}

	pub fn set_pos(&mut self, pos: glm::Vec3) {
		self.0[12] = pos[0];
		self.0[13] = pos[1];
		self.0[14] = pos[2];
	}

	pub fn get_pos(&self) -> glm::Vec3 {
		glm::vec4_to_vec3(&glm::column(&self.0, 3))
	}
}
