use crate::ops::Bindable;
use crate::render::texture::Texture;

use std::fs;
use std::path::Path;
use std::pin::Pin;
use std::slice;
use ttf_parser::Font as FontType;

struct FontCharacter {
    texture: Texture,
    size: (u16, u16),
    bearing: (u16, u16),
    advance: u16,
}

struct OwnedFont<'font> {
    data: Vec<u8>,
    font: Option<FontType<'font>>,
}

impl OwnedFont<'_> {
    fn new() -> Self {
        let font = 

        let mut p = Box::pin(font);

        unsafe {
            let slice: &'static [u8] = slice::from_raw_parts(p.data.as_ptr(), p.data.len());
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut p);
            let mut_inner = mut_ref.get_unchecked_mut();
            mut_inner.font = Some(FontType::from_data(slice, 0)?);
        }

        Some(p)
    }

    fn as_font(self: &'a OwnedFont) -> &FontType<'a> {
        match self.font.as_ref() {
            Some(f) => f,
            None => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

pub struct Font<'font> {
    font: OwnedFont,
}

impl<'font> Font<'font> {
    pub fn new(path: &Path, size: f32) -> Option<Pin<Box<Self>>> {
    }
}
