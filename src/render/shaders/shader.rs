use crate::utils::c::cstr_of_size;
use gl::types::{GLchar, GLint, GLuint};
use std::ffi::CString;

#[derive(Debug, Copy, Clone)]
struct Shader {}

impl Shader {
    fn compile(id: GLuint, src: &'static str) -> Result<GLuint, String> {
        let src = &CString::new(src).unwrap();

        unsafe {
            gl::ShaderSource(id, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        if let Some(err) = handle_error(id) {
            return Err(err);
        }

        return Ok(id);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VertexShader {
    id: GLuint,
}

impl VertexShader {
    pub fn compile(src: &'static str) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        Shader::compile(id, src)?;
        return Ok(Self { id });
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FragmentShader {
    id: GLuint,
}

impl FragmentShader {
    pub fn compile(src: &'static str) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        Shader::compile(id, src)?;
        return Ok(Self { id });
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}

fn handle_error(id: GLuint) -> Option<String> {
    let mut success: GLint = 1;

    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success != 0 {
        return None;
    }

    let mut length: GLint = 0;
    unsafe {
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length);
    }

    let error = cstr_of_size(length as usize);

    unsafe {
        gl::GetShaderInfoLog(
            id,
            length,
            std::ptr::null_mut(),
            error.as_ptr() as *mut GLchar,
        );
    }

    return Some(error.to_string_lossy().into_owned());
}
