use gl;
use gl::types::*;

/// Rust-friendly wrappers around the process of compiling
/// GLSL shaders and creating a Program from these.
/// The design should make use of Rust's memory safety features
/// to keep a shader uniquely owned as when it goes out of scope
/// and is cleaned up the shader is removed from the GPU
/// and we don't want to have a double-free error occur

fn compile_shader(src: &str, ty: GLenum) -> Result<GLuint> {
    let id = gl::CreateShader(ty);
    unsafe {
        src.with_c_str(|ptr| gl::ShaderSource(id, 1, &ptr, ptr::null()));
        gl::CompileShader(id);

        let mut status = gl::FALSE as GLuint;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLuint) {
            let mut len: GLuint = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = slice::from_elem(len as uint - 1, 0u8);
            gl::GetShaderInfoLog(id, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
            return Err(str::from_utf8_owned(buf).expect("ShaderInfoLog not valid UTF-8"));
        }
    }
    Ok(id)
}

struct ShaderVertex(GLuint);

// Taken from https://github.com/bjz/gl-rs/blob/master/src/examples/triangle.rs

// for compilation and creation of shaders and programs maybe it's
// best to have the module work as a factory with compile functions

impl ShaderVertex {
    pub fn new(src: &str) -> Result<ShaderVertex> {
        match compile_shader(src, gl::VERTEX_SHADER) {
            Ok(id) => Ok(ShaderVertex(id)),
            Err(s) => Err(s)
        }
    }
}

impl Drop for ShaderVertex {
    pub fn drop(&mut self) {
        let ShaderVertex(id) = self;
        gl::DeleteShader(id);
    }
}

struct ShaderFragment(GLuint);

impl ShaderFragment {
    pub fn new(src: &str) -> Result<ShaderFragment> {
        match compile_shader(src, gl::FRAGMENT_SHADER) {
            Ok(id) => Ok(ShaderFragment(id)),
            Err(s) => Err(s)
        }
    }
}

impl Drop for ShaderFragment {
    pub fn drop(&mut self) {
        let ShaderFragment(id) = self;
        gl::DeleteShader(id);
    }
}

pub struct ShaderProgram {
    id: GLuint,
    vs: ShaderVertex,
    fs: ShaderFragment
}

impl ShaderProgram {
    pub fn new(vert_src: &str, frag_src: &str) -> Result<ShaderProgram> {
        let vs = match ~ShaderVertex::new(vert_src) {
            Ok(v) => v,
            Err(s) => return Err(s)
        };
        let fs = match ~ShaderFragment::new(frag_src) {
            Ok(f) => f,
            Err(s) => return Err(s)
        };
        let ShaderVertex(vert_id) = vs;
        let ShaderFragment(frag_id) = fs;

        let id = gl::CreateProgram();
        gl::AttachShader(id, vert_id);
        gl::Attachshader(id, frag_id);
        gl::LinkProgram(id);

        unsafe {
            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = slice::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(id, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
                return Err(str::from_utf8_owned(buf).expect("ProgramInfoLog not valid utf8"));
            }
        }
        Ok(ShaderProgram { id: id, vs: vs, fs: fs })
    }

    pub fn begin(&self) {
        
    }
}

impl Drop for ShaderProgram {
    pub fn drop(&mut self) {
        gl::DeleteProgram(self.id);
    }
}