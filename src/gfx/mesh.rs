use cgmath::vector::{Vector2, Vector3};

pub struct Mesh {
	vertices: ~[Vector3],
	uv: ~[Vector2],
	indices: ~[int]
}

impl Mesh {
	pub fn new() -> Mesh {
		Mesh {
			vertices: ~[],
			uv: ~[],
			indices: ~[]
		}
	}

	pub fn set_vertices(&mut self, vertices: ~[Vector3]) {
		self.vertices = vertices;
	}

	pub fn set_uv(&mut self, uv: ~[Vector2]) {
		self.uv = uv;
	}

	pub fn set_indices(&mut self, indices: ~[int]) {
		self.indices = indices;
	}

	/// Commit the data to the GPU
	pub fn apply(&self) {

	}
}