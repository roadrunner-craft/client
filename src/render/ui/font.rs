use crate::render::texture::Texture;

use rusttype::{Font as FontType, Point, Scale};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct FontCharacter {
    texture: Texture,
    width: f32,
    height: f32,
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

        let bbox = glyph.exact_bounding_box()?;
        let positioned_glyph = glyph.positioned(Point { x: 0.0, y: 0.0 });

        let width = bbox.width();
        let height = bbox.height();

        //let mut image = Vec::new();

        debug!("{} -> {}x{} {:?}", c, width, height, h_metrics);

        positioned_glyph.draw(|x, y, v| debug!("{}, {}, {}", x, y, v));

        //let texture = Texture::from_image();
        let texture = Texture::default();

        Some(FontCharacter {
            texture,
            height,
            width,
            side_bearing: h_metrics.left_side_bearing,
            advance: h_metrics.advance_width,
        })
    }
}
