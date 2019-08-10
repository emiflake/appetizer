use specs::prelude::*;

use crate::components::*;
use crate::obj_parser;
use crate::resources::*;

fn register_components(world: &mut World) {
	world.register::<transformation::TransformationComponent>();
	world.register::<model::ModelComponent>();
	world.register::<name::NameComponent>();
	world.register::<material::MaterialComponent>();
	world.register::<light::LightComponent>();
}

fn insert_resources(world: &mut World) {
	world.insert(delta_time::DeltaTime(0.0));
	world.insert(key_state::Keystate::default());
	world.insert(camera::Camera {
		position: glm::vec3(0.0, 0.0, -10.0),
		..camera::Camera::default()
	});
	world.insert(projection::Projection::default());
	world.insert(mouse_state::MouseState::default());
	world.insert(time::CurrentTime::default());
}

pub fn create_world() -> Result<World, String> {
	let mut world: World = World::new();

	register_components(&mut world);
	insert_resources(&mut world);

	let plane = obj_parser::parse("objs/plane.obj".to_string())
		.map_err(|e| format!("Parser error: {:?}", e))?;

	{
		let mut camera = world.write_resource::<camera::Camera>();
		camera.update_camera_vectors();
	}

	world
		.create_entity()
		.with(transformation::TransformationComponent(
			glm::mat4(
				1.0, 0.0, 0.0, 0.0,
				0.0, 1.0, 0.0, 0.0,
				0.0, 0.0, 1.0, 0.0,
				0.0, 0.0, 0.0, 1.0
			),
		))
		.with(plane.get_component())
		.with(name::NameComponent("Alpha".to_string()))
		.build();

	world
		.create_entity()
		.with(transformation::TransformationComponent::from_pos(
			glm::vec3(0.0, 0.0, 0.0),
		))
		.with(light::LightComponent {
			color: glm::vec3(300.0, 300.0, 300.0),
		})
		.with(name::NameComponent("Random Light".to_string()))
		.build();

	Ok(world)
}
