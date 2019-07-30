use image::GenericImageView;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::c_void;

#[derive(Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub struct GLTextureHandle(u32);

#[derive(Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub struct TextureHandle(u32);

// TODO: Use HandleMap structure
#[derive(Default)]
pub struct TextureMap {
    pub texture_counter: u32,
    pub texture_handles: BTreeMap<TextureHandle, Box<RawTexture>>,
}

impl TextureMap {
    pub fn new() -> Self {
        Self {
            texture_counter: 0,
            texture_handles: BTreeMap::new(),
        }
    }

    pub fn load_from_file(&mut self, path: String) -> Result<TextureHandle, String> {
        let raw_texture = Box::new(RawTexture::load_from_file(path)?);
        let handle_counter = self.texture_counter;
        self.texture_counter += 1;
        let handle = TextureHandle(handle_counter);
        self.texture_handles.insert(handle, raw_texture);
        Ok(handle)
    }
    pub fn get(&self, handle: TextureHandle) -> Result<&Box<RawTexture>, String> {
        if let Some(texture) = self.texture_handles.get(&handle) {
            Ok(texture)
        } else {
            Err("Could not find that texture.".to_string())
        }
    }
}

pub struct RawTexture {
    pub img: image::DynamicImage,
    pub pixels: Vec<u8>,
}

impl RawTexture {
    pub fn load_from_file(path: String) -> Result<Self, String> {
        let img = image::open(path).map_err(|_| "Could not open texture")?;
        let pixels = img.raw_pixels();
        Ok(Self { img, pixels })
    }
}

#[derive(Default)]
pub struct GLTextureMap {
    pub gl_handles: BTreeSet<GLTextureHandle>,
}

impl GLTextureMap {
    pub fn new() -> Self {
        Self {
            gl_handles: BTreeSet::new(),
        }
    }
    pub fn load_from_map(
        &mut self,
        map: &TextureMap,
        handle: TextureHandle,
    ) -> Result<GLTextureHandle, String> {
        let texture = map.get(handle)?;
        Ok(self.load_texture(&texture))
    }
    pub fn load_texture(&mut self, raw_tex: &RawTexture) -> GLTextureHandle {
        let generated_handle = unsafe {
            let mut texture = 0; // GL Texture handle
            let texture_img = &raw_tex.img;
            let texture_raw = &raw_tex.pixels;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                texture_img.width() as i32,
                texture_img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &texture_raw[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            GLTextureHandle(texture)
        };
        self.gl_handles.insert(generated_handle);
        generated_handle
    }
    pub fn get_texture(&self, gl_texture_handle: GLTextureHandle) -> Option<u32> {
        if self.gl_handles.contains(&gl_texture_handle) {
            Some(gl_texture_handle.0)
        } else {
            None
        }
    }
}
