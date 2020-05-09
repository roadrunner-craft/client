use math::noise::LayeredNoise;
use std::ops::Range;

pub struct HeightMap {
    height: f64,
    range: Range<u8>,
    noise: LayeredNoise,
}

impl HeightMap {
    pub fn new(range: Range<u8>, seed: u32) -> Self {
        Self {
            height: (range.end - range.start) as f64,
            range,
            noise: LayeredNoise::new(6, 75.0, 0.40, 1.87, seed),
        }
    }

    pub fn get_height(&self, x: i64, z: i64) -> f64 {
        self.noise.get(x, z) * self.height + self.range.start as f64
    }
}
