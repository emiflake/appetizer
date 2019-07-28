trait Rasterizable<T> {
    fn rasterize(&self, state: &T);
}

pub struct RasterCollection<'a, T>(Vec<Box<dyn Rasterizable<T> + 'a>>);

impl<T> Rasterizable<T> for RasterCollection<'_, T> {
    fn rasterize(&self, state: &T) {
        for object in &self.0 {
            object.rasterize(state);
        }
    }
}
