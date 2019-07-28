pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub uv: glm::Vec2,
}

pub struct Object {
    pub positions: Vec<glm::Vec3>,
    pub normals: Vec<glm::Vec3>,
	pub uvs: Vec<glm::Vec2>,
    pub triangle_indices: Vec<glm::Vec3>,
}

impl Default for Object {
    fn default() -> Self {
        Self {
			positions: Vec::new(),
			normals: Vec::new(),
			uvs: Vec::new(),
            triangle_indices: Vec::new(),
        }
    }
}
