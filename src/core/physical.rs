
use cgmath::vector::{Vector3};

use super::component::{Component};

pub struct Physical {
	pub velocity: Vector3
}

impl Physical {
	fn apply_impulse(force: Vector3) {
		
	}
}

impl Component for Physical {
	
}