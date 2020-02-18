use crate::math::matrix::m4;
use crate::math::vector::v3;
use crate::render::shaders::{FragmentShader, VertexShader};
use crate::utils::c::cstr_of_size;
use crate::utils::c::str2cstr;

use gl::types::{GLchar, GLint, GLuint};
use std::mem;

#[derive(Debug, Copy, Clone)]
pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn create_and_link(vertex: VertexShader, fragment: FragmentShader) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(id, vertex.id());
            gl::AttachShader(id, fragment.id());
        }

        unsafe {
            gl::LinkProgram(id);
        }

        if let Some(err) = handle_error(id) {
            return Err(err);
        }

        unsafe {
            gl::DetachShader(id, vertex.id());
            gl::DetachShader(id, fragment.id());

            vertex.delete();
            fragment.delete();
        }

        Ok(ShaderProgram { id })
    }

    #[allow(dead_code)]
    fn get_uniform_location(&self, name: &str) -> GLint {
        let s = str2cstr(name);
        unsafe { gl::GetUniformLocation(self.id, s.as_ptr()) }
    }

    #[allow(dead_code)]
    pub fn set_uniform_texture(&self, name: &str, value: GLuint) {
        unsafe { gl::Uniform1i(self.get_uniform_location(name), value as i32) }
    }

    #[allow(dead_code)]
    pub fn set_uniform_float(&self, name: &str, value: f32) {
        unsafe { gl::Uniform1f(self.get_uniform_location(name), value) }
    }

    #[allow(dead_code)]
    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        unsafe { gl::Uniform1i(self.get_uniform_location(name), value as i32) }
    }

    #[allow(dead_code)]
    pub fn set_uniform_v3(&self, name: &str, value: v3) {
        unsafe { gl::Uniform3f(self.get_uniform_location(name), value.x, value.y, value.z) }
    }

    #[allow(dead_code)]
    pub fn set_uniform_m4(&self, name: &str, value: m4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location(name),
                1,
                gl::TRUE,
                mem::transmute(&value.0[0]),
            )
        }
    }

    pub fn enable(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> GLuint {
        self.id
    }
}

fn handle_error(id: GLuint) -> Option<String> {
    let mut success: GLint = 1;

    unsafe {
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
    }

    if success != 0 {
        return None;
    }

    let mut length: GLint = 0;
    unsafe {
        gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut length);
    }

    let error = cstr_of_size(length as usize);

    unsafe {
        gl::GetProgramInfoLog(
            id,
            length,
            std::ptr::null_mut(),
            error.as_ptr() as *mut GLchar,
        );
    }

    return Some(error.to_string_lossy().into_owned());
}
