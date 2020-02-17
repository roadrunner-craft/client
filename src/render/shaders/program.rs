use crate::render::shaders::{FragmentShader, VertexShader};
use crate::utils::c::cstr_of_size;
use gl::types::{GLchar, GLint, GLuint};

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

    pub fn enable(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

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
