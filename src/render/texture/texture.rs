use crate::ops::Bindable;

use gl::types::{GLint, GLsizei, GLuint};
use image::DynamicImage;
use std::ffi::c_void;
use std::path::Path;

static mut DEFAULT_TEXTURE_ID: GLuint = 0;
static mut DEFAULT_TEXTURE_SIZE: u32 = 0;

pub enum TextureType {
    GREYSCALE,
    RGBA,
}

#[derive(Clone, Default)]
pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
    unit: GLuint,
}

impl Texture {
    pub fn new(path: &Path, unit: GLuint) -> Option<Self> {
        let path = path.to_str()?;
        let img = match image::open(path).ok()? {
            DynamicImage::ImageRgba8(img) => img,
            img => img.to_rgba(),
        };

        let width = img.width();
        let height = img.height();

        Some(Self::from_image(
            &img.into_raw(),
            width,
            height,
            TextureType::RGBA,
            unit,
        ))
    }

    pub fn from_image(
        img: &Vec<u8>,
        width: u32,
        height: u32,
        texture_type: TextureType,
        unit: GLuint,
    ) -> Self {
        Texture {
            id: Texture::generate_texture(img, width, height, texture_type, unit),
            width,
            height,
            unit,
        }
    }

    fn generate_texture(
        img: &Vec<u8>,
        width: u32,
        height: u32,
        texture_type: TextureType,
        unit: GLuint,
    ) -> GLuint {
        unsafe {
            let mut id: GLuint = 0;
            gl::GenTextures(1, &mut id);

            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, id);

            let format = match texture_type {
                TextureType::GREYSCALE => gl::RED,
                TextureType::RGBA => gl::RGBA,
            };

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                format,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const c_void,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            id
        }
    }

    pub fn unit(&self) -> GLuint {
        self.unit
    }
}

impl Bindable for Texture {
    fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id)
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.unit);
            gl::BindTexture(gl::TEXTURE_2D, 0)
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            if self.id != DEFAULT_TEXTURE_ID {
                gl::DeleteTextures(1, &self.id)
            }
        }
    }
}
