use specs::{Component, VecStorage};
use glm::Mat4;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TransformationComponent(pub Mat4);