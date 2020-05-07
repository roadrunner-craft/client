use crate::utils::Bindable;

use gl::types::{GLint, GLsizei, GLuint};
use image::DynamicImage;
use std::ffi::c_void;
use std::path::Path;
use std::ptr;

#[derive(Debug)]
pub struct TextureArray {
    id: GLuint,
    size: u32,
}

impl TextureArray {
    pub fn new(size: u32, layer_count: u32) -> Self {
        let mut id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, id);

            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as GLint,
            );

            gl::TexImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                gl::RGBA as GLint,
                size as GLsizei,
                size as GLsizei,
                layer_count as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            )
        }

        Self { id, size }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn add_file(&self, path: &Path, layer: u32) {
        if let Some(path) = path.to_str() {
            match image::open(path) {
                Err(err) => {
                    println!("<texture-array> Could not load image {}: {}", path, err);
                    // TODO: add default image
                }
                Ok(img) => {
                    let img = match img {
                        DynamicImage::ImageRgba8(img) => img,
                        img => img.to_rgba(),
                    };

                    if img.width() != self.size || img.height() != self.size {
                        println!("<texture-array> Image aspect ratio must be 1: {}", path);
                        // TODO: add default image
                    }

                    self.add_texture(&img.into_raw(), layer);
                }
            }
        }
    }

    pub fn add_texture(&self, img: &Vec<u8>, layer: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id);

            gl::TexSubImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                0,
                0,
                layer as GLint,
                self.size as GLsizei,
                self.size as GLsizei,
                1,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const c_void,
            );
        }
    }
}

impl Bindable for TextureArray {
    fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id)
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, 0)
        }
    }
}

impl Drop for TextureArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) }
    }
}
