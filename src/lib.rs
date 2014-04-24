#![crate_id="metis#0.1"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![comment = "A game engine"]

#[feature(globs)]

extern crate cgmath;
//extern crate glfw;

extern crate collections;

pub mod core;
