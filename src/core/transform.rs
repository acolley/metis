
use cgmath::vector::{Vector3};
use cgmath::quaternion::{Quaternion};

use super::component::{Component};
// 

pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Quaternion<f32>
}

impl Component for Transform {
	fn get_component_type(&self) -> u64 { 0 }
}