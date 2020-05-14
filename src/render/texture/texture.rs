use crate::ops::Bindable;

use gl::types::{GLint, GLsizei, GLuint};
use image::DynamicImage;
use std::ffi::c_void;
use std::path::Path;

static mut DEFAULT_TEXTURE_ID: GLuint = 0;
static mut DEFAULT_TEXTURE_SIZE: u32 = 0;

pub struct Texture {
    id: GLuint,
    size: u32,
    unit: GLuint,
}

impl Texture {
    pub fn new(path: &Path, unit: GLuint) -> Self {
        match path.to_str() {
            None => Texture::default(),
            Some(path) => match image::open(path) {
                Err(err) => {
                    println!("<texture> Could not load image {}: {}", path, err);
                    return Self::default();
                }
                Ok(img) => {
                    let img = match img {
                        DynamicImage::ImageRgba8(img) => img,
                        img => img.to_rgba(),
                    };

                    let width = img.width();
                    if width != img.height() {
                        println!("<texture> Image aspect ratio must be 1: {}", path);
                        return Self::default();
                    }

                    return Self::from_image(&img.into_raw(), width, unit);
                }
            },
        }
    }

    pub fn from_image(img: &Vec<u8>, size: u32, unit: GLuint) -> Self {
        Texture {
            id: Texture::generate_texture(img, size, unit),
            size,
            unit,
        }
    }

    fn generate_texture(img: &Vec<u8>, size: u32, unit: GLuint) -> GLuint {
        unsafe {
            let mut id: GLuint = 0;
            gl::GenTextures(1, &mut id);
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                size as GLsizei,
                size as GLsizei,
                0,
                gl::RGBA,
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

    fn unit(&self) -> GLuint {
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

impl Default for Texture {
    fn default() -> Self {
        unsafe {
            if DEFAULT_TEXTURE_ID != 0 {
                return Texture {
                    id: DEFAULT_TEXTURE_ID,
                    size: DEFAULT_TEXTURE_SIZE,
                    unit: 0,
                };
            }

            // TODO: change this for default texture
            let root = env!("CARGO_MANIFEST_DIR");
            let path = Path::new(root).join("res/textures/block/dirt.png");

            let t = Texture::new(&path, 0);
            DEFAULT_TEXTURE_ID = t.id;
            DEFAULT_TEXTURE_SIZE = t.size;
            t
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
