use crate::render::display::FrameBuffer;

pub trait PostProcessingEffect {
    fn prepare(&self, src: &FrameBuffer);
}
