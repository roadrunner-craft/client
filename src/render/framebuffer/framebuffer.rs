use crate::utils::Bindable;

use gl::types::GLuint;

pub trait Target {}

#[derive(Default)]
pub struct ColorFrameBuffer {
    id: GLuint,
    texture: Texture,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);
        }

        Self { id }
    }
}

impl Bindable for FrameBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}
