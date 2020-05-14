use crate::utils::c::{cstr_with_size, str2cstr};
use crate::utils::Identifiable;

use gl::types::{GLchar, GLint, GLuint};
use std::ptr;

/// https://www.khronos.org/opengl/wiki/Shader
#[allow(dead_code)]
pub enum ShaderType {
    Vertex,
    Fragment,
    TesselationControl,
    TesselationEvaluation,
    Geometry,
    Compute,
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn compile(src: &'static str, shader_type: ShaderType) -> Result<Self, String> {
        let id: GLuint = unsafe {
            match shader_type {
                ShaderType::Vertex => gl::CreateShader(gl::VERTEX_SHADER),
                ShaderType::Fragment => gl::CreateShader(gl::FRAGMENT_SHADER),
                ShaderType::TesselationControl => gl::CreateShader(gl::TESS_CONTROL_SHADER),
                ShaderType::TesselationEvaluation => gl::CreateShader(gl::TESS_EVALUATION_SHADER),
                ShaderType::Geometry => gl::CreateShader(gl::GEOMETRY_SHADER),
                ShaderType::Compute => gl::CreateShader(gl::COMPUTE_SHADER),
            }
        };

        let src = str2cstr(src);

        unsafe {
            gl::ShaderSource(id, 1, &src.as_ptr(), ptr::null());
            gl::CompileShader(id);
        }

        if let Some(err) = handle_error(id) {
            return Err(err);
        }

        Ok(Self { id })
    }

    pub fn attach(&self, program: gl::types::GLuint) {
        unsafe {
            gl::AttachShader(program, self.id());
        }
    }

    pub fn detach(&self, program: gl::types::GLuint) {
        unsafe {
            gl::DetachShader(program, self.id());
        }
    }
}

impl Identifiable for Shader {
    type Id = gl::types::GLuint;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
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

    let error = cstr_with_size(length as usize);

    unsafe {
        gl::GetShaderInfoLog(id, length, ptr::null_mut(), error.as_ptr() as *mut GLchar);
    }

    return Some(error.to_string_lossy().into_owned());
}
