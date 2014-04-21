
LIB_DIR = lib

GLFW = lib/glfw-rs
GLFW_LIB = lib/glfw-rs/lib

CGMATH = lib/cgmath-rs
CGMATH_LIB = lib/cgmath-rs/lib

all: deps lib

lib:
	rustc src/lib.rs --crate-type=lib -L $(GLFW_LIB) -L $(CGMATH_LIB) --out-dir $(LIB_DIR)

deps:
	make lib $(GLFW)
	make lib $(CGMATH)
