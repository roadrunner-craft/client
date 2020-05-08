use noise::{NoiseFn, Perlin, Seedable};

#[derive(Copy, Clone, Debug)]
struct NoiseOptions {
    /// The number of noise layer to generate
    octaves: u32,
    /// The scale of the noise
    scale: f64,
    /// A number between 0.0 and 1.0 that determines how much each octave contributes to the final
    /// image (Changes amplitude)
    persistance: f64,
    /// A number >= 1 that determines how much detail is added or removed at each octave. (Changes
    /// frequency)
    lacunarity: f64,
    /// The seed used by the noise function
    seed: u32,
}

pub struct Noise {
    noise: Perlin,
    options: NoiseOptions,
}

impl Noise {
    pub fn new(octaves: u32, scale: f64, persistance: f64, lacunarity: f64, seed: u32) -> Self {
        Self {
            noise: Perlin::new().set_seed(seed),
            options: NoiseOptions {
                octaves,
                scale,
                persistance,
                lacunarity,
                seed,
            },
        }
    }

    pub fn get(&self, x: i64, y: i64) -> f64 {
        let mut acc: f64 = 0.0;
        let mut acc_amplitude: f64 = 0.0;

        let mut frequency: f64 = 1.0;
        let mut amplitude: f64 = 1.0;

        for _ in 0..self.options.octaves {
            let sample_x = x as f64 * frequency / self.options.scale;
            let sample_y = y as f64 * frequency / self.options.scale;

            let value = self.noise.get([sample_x, sample_y]);
            acc += value * amplitude;
            acc_amplitude += amplitude;

            frequency *= self.options.lacunarity;
            amplitude *= self.options.persistance;
        }

        (acc + acc_amplitude) / (2.0 * acc_amplitude)
    }
}
