use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MaterialComponent {
	pub ambient: glm::Vec3,
	pub diffuse: glm::Vec3,
	pub specular: glm::Vec3,
	pub shininess: f32,
}
