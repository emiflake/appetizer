use crate::object::Object;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ModelComponent(pub Object);
