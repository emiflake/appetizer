use specs::prelude::*;

#[derive(Debug)]
pub enum Light {
	PointLight {
		ambient: glm::Vec3,
		diffuse: glm::Vec3,
		specular: glm::Vec3,

		constant: f32,
		linear: f32,
		quadratic: f32,
	},
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct LightComponent(pub Light);
