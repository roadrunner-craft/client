use crate::render::display::FrameBuffer;

pub trait PostProcessingEffect {
    fn apply(&self, src: &FrameBuffer, dst: &FrameBuffer);
}
