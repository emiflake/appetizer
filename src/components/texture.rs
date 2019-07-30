use specs::{Component, VecStorage};

use crate::resources::texture_map::GLTextureHandle;

// Store OpenGL Texture handles only
// Actual textures are stored in TextureMap,
// As well as references to the handles
#[derive(Component)]
#[storage(VecStorage)]
pub struct GLTextureComponent(pub GLTextureHandle);
