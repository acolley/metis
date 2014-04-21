use super::entity::Entity;
use super::world::World;

pub trait System {
	fn get_aspect(&self) -> u64;
	fn update(&mut self, entity: Entity);
}