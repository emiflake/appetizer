use glm::Mat4;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TransformationComponent(pub Mat4);
