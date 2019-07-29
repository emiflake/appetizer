use specs::{Component, VecStorage};
use crate::object::Object;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ModelComponent(pub Object);