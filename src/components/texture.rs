use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Texture;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TextureComponent(pub Texture);
