use glm;

pub struct Camera {
	pub position: glm::Vec3,
	pub front: glm::Vec3,
	pub up: glm::Vec3,
	pub right: glm::Vec3,
	pub world_up: glm::Vec3,

	pub yaw: f32,
	pub pitch: f32,

	pub speed: f32,
	pub zoom: f32,
}

pub enum CameraDirection {
	Forward,
	Backward,
	Left,
	Right,
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			position: glm::Vec3::new(0.0, 0.0, 0.0),
			front: glm::Vec3::new(0.0, 0.0, 0.0),
			up: glm::Vec3::new(0.0, 0.0, 0.0),
			right: glm::Vec3::new(0.0, 0.0, 0.0),
			world_up: glm::Vec3::new(0.0, 1.0, 0.0),
			yaw: -90.0,
			pitch: 0.0,
			speed: 2.5,
			zoom: 45.0,
		}
	}
}

impl Camera {
	pub fn new(position: glm::Vec3) -> Self {
		let mut camera = Self {
			position,
			..Default::default()
		};

		camera.update_camera_vectors();
		camera
	}

	// Project the view forwards,
	// self.front is essentially the point we're looking it.
	pub fn get_view_matrix(&self) -> glm::Mat4 {
		glm::look_at(&self.position, &(self.position + self.front), &self.up)
	}

	// Primitive movement for keyboards, (dumb)
	pub fn do_move(&mut self, d: CameraDirection, dt: f32) {
		let velocity = self.speed * dt;
		match d {
			CameraDirection::Forward => self.position += self.front * velocity,
			CameraDirection::Backward => self.position -= self.front * velocity,
			CameraDirection::Right => self.position += self.right * velocity,
			CameraDirection::Left => self.position -= self.right * velocity,
		}
	}

	pub fn do_move_relative(&mut self, delta: glm::Vec3, dt: f32) {
		self.position += self.front * delta.z + self.up * delta.y + self.right * delta.x;
	}

	// Rotate Yaw and Pitch.
	pub fn do_rotate(&mut self, offset: glm::Vec2) {
		self.yaw += offset.x;
		self.pitch += offset.y;

		if self.pitch >= 90.0 {
			self.pitch = 89.9;
		} else if self.pitch <= -90.0 {
			self.pitch = -89.9;
		}

		self.update_camera_vectors();
	}

	pub fn update_camera_vectors(&mut self) {
		self.front = glm::vec3(
			self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
			self.pitch.to_radians().sin(),
			self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
		)
		.normalize();

		self.right = self.front.cross(&self.world_up).normalize();
		self.up = self.right.cross(&self.front).normalize();
	}
}
