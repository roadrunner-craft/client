use crate::render::texture::{Texture, TextureType};
use crate::render::ui::Rect;

use rusttype::{Font as FontType, Point, Scale};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct FontCharacter {
    texture: Texture,
    width: u32,
    height: u32,
    side_bearing: f32,
    advance: f32,
}

pub struct Font {
    size: f32,
    line_gap: f32,
    chars: HashMap<char, FontCharacter>,
}

impl Font {
    pub fn new(path: &Path, size: f32) -> Option<Self> {
        let data = fs::read(path.to_str()?).ok()?;
        let font = FontType::try_from_bytes(data.as_slice())?;

        let scale = Scale::uniform(size);
        let v_metrics = font.v_metrics(scale);

        let mut chars = HashMap::new();

        if let Some(font_char) = Font::generate_glyph(&font, '\u{0}', scale) {
            chars.insert('\u{0}', font_char);
        }

        for i in 0x20..0x7f_u8 {
            if let Some(font_char) = Font::generate_glyph(&font, i as char, scale) {
                chars.insert(i as char, font_char);
            }
        }

        Some(Self {
            size,
            line_gap: v_metrics.line_gap,
            chars,
        })
    }

    fn generate_glyph(font: &FontType, c: char, scale: Scale) -> Option<FontCharacter> {
        let glyph = font.glyph(c).scaled(scale);
        let h_metrics = glyph.h_metrics();

        let positioned_glyph = glyph.positioned(Point { x: 0.0, y: 0.0 });
        let bbox = positioned_glyph.pixel_bounding_box()?;

        let width = bbox.width() as u32;
        let height = bbox.height() as u32;

        let mut image = Vec::new();
        image.resize((width * height) as usize, 0);

        positioned_glyph.draw(|x, y, v| image[(y * width + x) as usize] = (v * 255.0) as u8);

        let texture = Texture::from_image(&image, width, height, TextureType::GREYSCALE, 5);

        Some(FontCharacter {
            texture,
            height,
            width,
            side_bearing: h_metrics.left_side_bearing,
            advance: h_metrics.advance_width,
        })
    }

    pub fn iter_for<'a>(&'a self, string: String, width: f32) -> FontIterator<'a> {
        FontIterator {
            font: self,
            string,
            width,
            x: 0.0,
            y: 0.0,
            index: 0,
        }
    }

    pub fn chars<'a>(&'a self, c: char) -> Option<&'a FontCharacter> {
        self.chars.get(&c)
    }
}

pub struct FontIterator<'a> {
    font: &'a Font,
    string: String,
    width: f32,
    x: f32,
    y: f32,
    index: usize,
}

impl<'a> Iterator for FontIterator<'a> {
    type Item = (Rect, &'a Texture);

    fn next(&mut self) -> Option<Self::Item> {
        // find font info for character
        let c = self.string.chars().nth(self.index)?;
        let font_char = self
            .font
            .chars(c)
            .unwrap_or_else(|| self.font.chars('\u{0}').unwrap());

        self.index += 1;

        let rect = Rect::new(
            self.x + font_char.side_bearing,
            self.y,
            font_char.width as f32,
            font_char.height as f32,
        );

        let result = (rect, &font_char.texture);

        self.x += font_char.advance;

        Some(result)
    }
}
