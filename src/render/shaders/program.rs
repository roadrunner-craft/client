use crate::render::shaders::{Shader, ShaderType};
use crate::utils::c::{cstr_with_size, str2cstr};
use crate::utils::Identifiable;

use gl::types::{GLchar, GLint, GLuint};
use math::matrix::Matrix4;
use math::vector::{Vector2, Vector3};
use std::mem;
use std::ptr;
use std::vec::Vec;

#[derive(Debug)]
pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn new(vertex_src: &'static str, fragment_src: &'static str) -> Result<Self, String> {
        let vertex = Shader::compile(vertex_src, ShaderType::Vertex);
        let fragment = Shader::compile(fragment_src, ShaderType::Fragment);

        let shaders = vec![vertex?, fragment?];
        ShaderProgram::create_and_link(&shaders)
    }

    fn create_and_link(shaders: &Vec<Shader>) -> Result<Self, String> {
        let id: GLuint = unsafe { gl::CreateProgram() };

        for shader in shaders {
            shader.attach(id);
        }

        unsafe {
            gl::LinkProgram(id);
        }

        if let Some(err) = handle_error(id) {
            return Err(err);
        }

        for shader in shaders {
            shader.detach(id);
        }

        Ok(ShaderProgram { id })
    }

    fn get_uniform_location(&self, name: &str) -> GLint {
        let s = str2cstr(name);
        unsafe { gl::GetUniformLocation(self.id, s.as_ptr()) }
    }

    pub fn set_uniform_texture(&self, name: &str, value: GLuint) {
        unsafe { gl::Uniform1i(self.get_uniform_location(name), value as i32) }
    }

    pub fn set_uniform_u32(&self, name: &str, value: u32) {
        unsafe { gl::Uniform1ui(self.get_uniform_location(name), value) }
    }

    pub fn set_uniform_float(&self, name: &str, value: f32) {
        unsafe { gl::Uniform1f(self.get_uniform_location(name), value) }
    }

    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        unsafe { gl::Uniform1i(self.get_uniform_location(name), value as i32) }
    }

    pub fn set_uniform_v2(&self, name: &str, value: Vector2) {
        unsafe { gl::Uniform2f(self.get_uniform_location(name), value.x, value.y) }
    }

    pub fn set_uniform_v3(&self, name: &str, value: Vector3) {
        unsafe { gl::Uniform3f(self.get_uniform_location(name), value.x, value.y, value.z) }
    }

    pub fn set_uniform_m4(&self, name: &str, value: &Matrix4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location(name),
                1,
                gl::TRUE,
                mem::transmute(&value[0]),
            )
        }
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) }
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

    let error = cstr_with_size(length as usize);

    unsafe {
        gl::GetProgramInfoLog(id, length, ptr::null_mut(), error.as_ptr() as *mut GLchar);
    }

    return Some(error.to_string_lossy().into_owned());
}

impl Identifiable for ShaderProgram {
    type Id = gl::types::GLuint;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
