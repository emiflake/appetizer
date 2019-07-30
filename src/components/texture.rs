use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]

// Store OpenGL Texture handles only
// Actual textures are stored in TextureMap,
// As well as references to the handles
pub struct TextureComponent(pub usize);
