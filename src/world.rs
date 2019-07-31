use specs::prelude::*;

use crate::components::*;
use crate::obj_parser;
use crate::resources::*;

fn register_components(mut world: &mut World) {
	world.register::<transformation::TransformationComponent>();
	world.register::<model::ModelComponent>();
	world.register::<name::NameComponent>();
	world.register::<texture::GLTextureComponent>();
	world.register::<shader::ShaderComponent>();
	world.register::<material::MaterialComponent>();
	world.register::<light::LightComponent>();
}

fn insert_resources(mut world: &mut World) {
	world.insert(delta_time::DeltaTime(0.0));
	world.insert(key_state::Keystate::default());
	world.insert(camera::Camera::default());
	world.insert(texture_map::TextureMap::new());
	world.insert(texture_map::GLTextureMap::new());
	world.insert(projection::Projection::default());
	world.insert(mouse_state::MouseState::default());
	world.insert(time::CurrentTime::default());
}

pub fn create_world() -> Result<World, String> {
	// Initialized everything
	let mut world: World = World::new();

	register_components(&mut world);
	insert_resources(&mut world);

	let shader = shader::ShaderComponent::new("vertex.vs", "fragment.fs")
		.map_err(|e| format!("Shader error: {:?}", e))?;

	let teapot = obj_parser::parse("objs/teapot.obj".to_string())
		.map_err(|e| format!("Parser error: {:?}", e))?;
	let cube = obj_parser::parse("objs/cube.obj".to_string())
		.map_err(|e| format!("Parser error: {:?}", e))?;

	{
		let mut camera = world.write_resource::<camera::Camera>();
		camera.update_camera_vectors();
	}
	let white_texture_handle = {
		let mut texture_map = world.write_resource::<texture_map::TextureMap>();
		texture_map.load_from_file("assets/textures/white.png".to_string())?
	};
	let white_gltexture_handle = {
		let texture_map = world.read_resource::<texture_map::TextureMap>();
		let mut gltexture_map = world.write_resource::<texture_map::GLTextureMap>();

		gltexture_map.load_from_map(&texture_map, white_texture_handle)?
	};
	let wall_texture_handle = {
		let mut texture_map = world.write_resource::<texture_map::TextureMap>();
		texture_map.load_from_file("assets/textures/wall.jpg".to_string())?
	};
	let wall_gltexture_handle = {
		let texture_map = world.read_resource::<texture_map::TextureMap>();
		let mut gltexture_map = world.write_resource::<texture_map::GLTextureMap>();

		gltexture_map.load_from_map(&texture_map, wall_texture_handle)?
	};

	world
		.create_entity()
		.with(transformation::TransformationComponent(glm::mat4(
			1.0, 0.0, 0.0, 0.0, //
			0.0, 1.0, 0.0, 0.0, //
			0.0, 0.0, 1.0, 0.0, //
			0.0, 0.0, 0.0, 1.0, //
		)))
		.with(material::MaterialComponent {
			ambient: glm::vec3(0.1, 0.1, 0.1),
			diffuse: glm::vec3(0.5, 0.5, 0.5),
			specular: glm::vec3(0.8, 0.8, 0.8),
			shininess: 32.0,
		})
		.with(shader)
		.with(teapot.get_component())
		.with(texture::GLTextureComponent(wall_gltexture_handle))
		.with(name::NameComponent("Alpha".to_string()))
		.build();

	world
		.create_entity()
		.with(transformation::TransformationComponent::from_pos(
			glm::vec3(0.0, 1000.0, 0.0),
		))
		.with(shader)
		.with(material::MaterialComponent {
			ambient: glm::vec3(1.0, 0.5, 0.5),
			diffuse: glm::vec3(0.0, 0.0, 0.0),
			specular: glm::vec3(0.0, 0.0, 0.0),
			shininess: 32.0,
		})
		.with(cube.get_component())
		.with(texture::GLTextureComponent(white_gltexture_handle))
		.with(light::LightComponent(light::Light::PointLight {
			ambient: glm::vec3(1.0, 1.0, 1.0),
			diffuse: glm::vec3(1.0, 1.0, 1.0),
			specular: glm::vec3(1.0, 1.0, 1.0),
			constant: 1.0,
			linear: 0.09,
			quadratic: 0.032,
		}))
		.with(name::NameComponent("Random Light".to_string()))
		.build();

	Ok(world)
}
