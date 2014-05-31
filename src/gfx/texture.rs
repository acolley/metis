
extern crate gl;

use std::ptr;
use std::vec;
use std::libc::{c_void};

// use stb_image::image;
// use stb_image::image::ImageU8;

pub enum PixelFormat {
	RGB,
	RGBA
}

// abstracts the concept of a Texture on the GPU
pub trait Texture {
	fn get_width(&self) -> i32;
	fn get_height(&self) -> i32;

	fn bind(&self, target: gl::types::GLenum);
	fn unbind(&self);
}

// wraps the concept of an OpenGL texture,
// automatically reserving memory on the GPU
// for it.
pub struct Texture2D {
	width: i32,
	height: i32,
	depth: i32, // how many components there are per pixel
	format: u32, // TODO: wrap this sort of thing with a Rust enum?

	texID: u32 // the opengl texture id assigned to this Texture2D
}

impl Texture2D {
	// create a new blank Texture2D with no data supplied
	pub fn new(width: i32, height: i32, depth: PixelFormat) -> Texture2D {
		let format = match depth {
			RGB => gl::RGB as u32,
			RGBA => gl::RGBA as u32
		};

		// TODO: error checking to make this safe
		// if we do this inside a do task::try block
		// then this should provide some safety...
		// not sure if it works for unsafe code that can
		// dereference NULL pointers resulting in seg faults
		let texID = unsafe {
			let c_id = &mut 1;
			gl::GenTextures(1, ptr::to_mut_unsafe_ptr(c_id));

			gl::BindTexture(gl::TEXTURE_2D, (*c_id as u32));

			gl::TexImage2D(
				gl::TEXTURE_2D,
				0, // highest detail (TODO: generate mip maps)
				format as i32,
				width,
				height ,
				0,
				format,
				gl::UNSIGNED_BYTE,
				(0 as *c_void)
			);

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, (gl::LINEAR as i32));
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, (gl::LINEAR as i32));
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, (gl::CLAMP_TO_EDGE as i32));
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, (gl::CLAMP_TO_EDGE as i32));

			(*c_id as u32).clone()
		};

		Texture2D {
			width: width,
			height: height,
			depth: depth,
			format: format,
			texID: texID
		}
	}

	// create a new Texture2D with supplied image data
	pub fn from_data(width: i32, height: i32, depth: PixelFormat, data: &[u8]) -> Texture2D {
		let tex = Texture2D::new(width, height, depth);

		tex.bind(gl::TEXTURE_2D);

		unsafe {
			gl::TexSubImage2D(
				gl::TEXTURE_2D,
				0, 0, 0,
				width,
				height,
				tex.format,
				gl::UNSIGNED_BYTE,
				vec::raw::to_ptr(data) as *c_void
			);
		}

		tex
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}

	pub fn get_height(&self) -> u32 {
		self.height
	}

	pub fn get_depth(&self) -> u32 {
		self.depth
	}

	pub fn set_data(&self) {

	}

	pub fn get_data(&self) -> &[u8] {
		&[]
	}
}

impl Texture for Texture2D {
	fn get_width(&self) -> i32 {
		self.width
	}

	fn get_height(&self) -> i32 {
		self.height
	}

	#[inline]
	fn bind(&self, target: gl::types::GLenum) {
		gl::BindTexture(target, self.texID);
	}

	#[inline]
	fn unbind(&self) {

	}
}

impl Drop for Texture2D {
	fn drop(&mut self) {
		// TODO: unbind tex2D id memory from GPU
		unsafe {
			gl::DeleteTextures(1, &self.texID);
		}
	}
}
