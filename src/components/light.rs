use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct LightComponent {
	pub color: glm::Vec3,
}
