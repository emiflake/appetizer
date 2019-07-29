pub trait Rasterizable<'a, T, S> {
    fn rasterize(&self, state: &T, raster_settings: &RasterSettings<S>);
}

pub struct RasterCollection<'a, T, S>(Vec<Box<dyn Rasterizable<'a, T, S> + 'a>>);

impl<'a, T, S> Rasterizable<'a, T, S> for RasterCollection<'_, T, S> {
    fn rasterize(&self, state: &T, raster_settings: &RasterSettings<S>) {
        for object in &self.0 {
            object.rasterize(state, raster_settings);
        }
    }
}

pub struct RasterSettings<T> {
	pub projection: glm::Mat4,
	pub specifics: T,
}
