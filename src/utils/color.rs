use math::vector::Vector4;

#[derive(Copy, Clone, Debug, Hash)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 0xff }
    }

    #[inline]
    pub fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// color from hex code with the RGB format
    /// ex: 0x00FF00 is green
    #[inline]
    #[allow(dead_code)]
    pub fn from_hex(x: u32) -> Self {
        Self::new(
            (x >> 16 & 0xff) as u8,
            (x >> 8 & 0xff) as u8,
            (x & 0xff) as u8,
        )
    }

    /// color from hex code with the RGBA format
    /// ex: 0xFF00007f is red with half opacity
    #[inline]
    #[allow(dead_code)]
    pub fn from_hex_alpha(x: u32) -> Self {
        Self::new_alpha(
            (x >> 24 & 0xff) as u8,
            (x >> 16 & 0xff) as u8,
            (x >> 8 & 0xff) as u8,
            (x & 0xff) as u8,
        )
    }

    pub fn as_vec(&self) -> Vector4 {
        Vector4 {
            x: self.r as f32 / 255.0,
            y: self.g as f32 / 255.0,
            z: self.b as f32 / 255.0,
            w: self.a as f32 / 255.0,
        }
    }

    #[inline]
    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    #[inline]
    pub fn clear() -> Self {
        Self::new_alpha(0, 0, 0, 0)
    }
}
