
use collections::hashmap::HashMap;

use super::entity::Entity;
use super::component::{Component, ComponentType, ComponentTypeManager};
use super::system::System;

pub struct World {
	current_id: u64,
    comp_type_manager: ComponentTypeManager,
	entities: HashMap<Entity, Vec<~Component>>,
	recycled: Vec<Entity>,
	systems: Vec<~System>,
	pub time_delta: f32
}

impl World {
	pub fn new() -> World {
		World {
			current_id: 0,
            comp_type_manager: ComponentTypeManager::new(),
			entities: HashMap::new(),
			recycled: Vec::new(),
			systems: Vec::new(),
			time_delta: 0f32
		}
	}

	fn get_entity_aspect(&self, entity: Entity) -> u64 {
		let mut aspect: u64 = 0;
		for component in self.entities.get(&entity).iter() {
			aspect |= component.get_component_type();
		}
		aspect
	}

	/// Create an entity, returning it as the result.
	/// Will reuse old removed entities.
	/// Maximum possible number of entities is 2^64.
	pub fn create_entity(&mut self) -> Entity {
		let entity = match self.recycled.pop() {
			Some(e) => e,
			None => { self.current_id += 1; Entity(self.current_id - 1) }
		};
		self.entities.insert(entity, Vec::new());
		entity
	}

	/// Remove an entity and its components.
	pub fn remove_entity(&mut self, entity: Entity) {
		self.entities.remove(&entity);
		self.recycled.push(entity);
	}

	/// Add the given component to the given Entity.
	pub fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: ~T) {
		let components = self.entities.get_mut(&entity);
		components.push(component as ~Component);
	}

	pub fn add_system<T: System + 'static>(&mut self, system: ~T) {
		self.systems.push(system as ~System)
	}

	pub fn update(&mut self) {
		for system in self.systems.mut_iter() {
			// TODO: make searching for the entity set that
			// matches the system's aspect more efficient by
			// storing the set when new entities are added
			// at the end of the frame
			for &entity in self.entities.keys() {
                // TODO: implement this using an Aspect type
				// get the entity's aspect
                //let aspect = self.entities.get(&entity).iter().fold(ComponentType(0u64), |acc,x| acc | self.comp_type_manager.get_type_for_instance(*x));
				let aspect = self.entities.get(&entity).iter().fold(0u64, |acc,x| acc | x.get_component_type());

                // if system.aspect().matches(&entity) etc...

				if aspect | system.get_aspect() != 0 {
					system.update(entity);
				}
			}
		}
	}
}
