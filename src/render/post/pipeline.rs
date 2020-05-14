use crate::ops::{Bindable, Drawable};
use crate::render::display::FrameBuffer;
use crate::render::mesh::TextureQuad;
use crate::render::post::effects::*;
use crate::render::post::PostProcessingEffect;

#[allow(dead_code)]
pub enum PostProcessingEffectType {
    Identity,
    StaticWave,
    Inverted,
}

pub struct PostProcessingPipeline {
    effects: Vec<Box<dyn PostProcessingEffect>>,
    screen: FrameBuffer,
    swap1: FrameBuffer,
    swap2: FrameBuffer,
    quad: TextureQuad,
}

impl PostProcessingPipeline {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            effects: Vec::new(),
            swap1: FrameBuffer::new(width, height, 2, false),
            swap2: FrameBuffer::new(width, height, 1, false),
            screen: FrameBuffer::empty(),
            quad: TextureQuad::new(),
        }
    }

    pub fn add(&mut self, effect: PostProcessingEffectType) {
        self.effects.push(match effect {
            PostProcessingEffectType::Identity => Box::new(IdentityPostProcessing::new()),
            PostProcessingEffectType::StaticWave => Box::new(StaticWavePostProcessing::new()),
            PostProcessingEffectType::Inverted => Box::new(InvertedPostProcessing::new()),
        });
    }

    pub fn apply(&self, input: &FrameBuffer) {
        let len = self.effects.len();

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }

        for i in 0..len {
            let source: &FrameBuffer;
            let mut target: &FrameBuffer;

            if i == 0 {
                source = input;
                target = &self.swap2;
            } else if i % 2 == 0 {
                source = &self.swap1;
                target = &self.swap2;
            } else {
                source = &self.swap2;
                target = &self.swap1;
            }

            if i == len - 1 {
                target = &self.screen;
            }

            self.effects[i].prepare(source);

            source.bind_texture();
            target.bind();
            target.clear(true, false, false);
            self.quad.draw();
        }
    }
}
