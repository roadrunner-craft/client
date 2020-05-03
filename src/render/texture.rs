use crate::utils::traits::Bindable;

use gl::types::GLuint;
use image::DynamicImage;
use std::ffi::c_void;
use std::path::Path;

pub struct Texture {
    pub id: GLuint,
}

impl Texture {
    pub fn new() -> Texture {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("res/textures/block/dirt.png");
        let path = path.to_str().unwrap();

        match image::open(path) {
            Err(err) => {
                println!("Could not load image {}: {}", path, err);
                return Texture { id: 0 }; // Texture::default();
            }
            Ok(img) => {
                let img = match img {
                    DynamicImage::ImageRgba8(img) => img,
                    img => img.to_rgba(),
                };

                unsafe {
                    let mut id: GLuint = 0;
                    gl::GenTextures(1, &mut id);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, id);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGBA as i32,
                        img.width() as i32,
                        img.height() as i32,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        img.into_raw().as_ptr() as *const c_void,
                    );

                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

                    // TODO: assign to uniform if there are more than one texture

                    Texture { id }
                }
            }
        }
    }
}

//impl Default for Texture {
// TODO: implement a default texture
//}

impl Bindable for Texture {
    fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        //       unsafe { gl::DeleteTextures(1, &self.id) }
    }
}
