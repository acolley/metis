
use std::hash::Hash;

// use std::any::{Any};
// use std::cast::{transmute};

// use super::component::{Component};

// #[deriving(Clone)]
// pub struct Entity {
// 	id: uint,
// 	aspect_id: uint,
// 	components: Vec<~Component>
// }

#[deriving(Clone, Eq, Hash, TotalEq)]
pub struct Entity(pub u64);

// impl Entity {
// 	// TODO: take a World as argument
// 	pub fn new(id: uint) -> Entity {
// 		Entity { 
// 			id: id,
// 			aspect_id: 0,
// 			components: Vec::new()
// 		}
// 	}

// 	pub fn get_aspect_id(&self) -> uint {
// 		self.aspect_id
// 	}

// 	pub fn add_component<T: Component>(&mut self, component: T) {
// 		self.components.push(component.clone() as ~Component);

// 		self.aspect_id |= component.get_component_type();
// 	}

// 	pub fn get_component<'a, T: Component>(&'a self, component_type: uint) -> Option<&'a ~T> {
// 		for component in self.components.iter() {
// 			if component.get_component_type() | component_type != 0 {
// 				unsafe { 
// 					return Some(transmute(component)); 
// 				}
// 			}
// 		}

// 		None
// 	}

// 	pub fn get_component_mut<'a, T: Component>(&'a mut self, component_type: uint) -> Option<&'a mut ~T> {
// 		for component in self.components.mut_iter() {
// 			if component.get_component_type() | component_type != 0 {
// 				unsafe {
// 					return Some(transmute(component));
// 				}
// 			}
// 		}

// 		None
// 	}
// }

// impl Eq for Entity {
// 	fn eq(&self, other: &Entity) -> bool {
// 		self.id == other.id
// 	}
// }