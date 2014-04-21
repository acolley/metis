
pub trait Component {
	// the component is a bit flag that uniquely
	// identifies the component among other components
	// TODO: make this global (in the World struct?)
	// using a u64 for type means the maximum number of
	// unique components is 64 (maybe 65 if 0 is also used)
	fn get_component_type(&self) -> u64;
	// fn start(&mut self);
	// fn update(&mut self);
}