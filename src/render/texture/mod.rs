mod texture;
mod texture_array;

pub use self::texture::{Texture, TextureType};
pub use self::texture_array::TextureArray;

use gl::types::GLuint;
pub type TextureUnit = GLuint;
