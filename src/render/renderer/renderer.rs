use crate::game::entity::Player;
use crate::input::InputHandler;
use crate::ops::Bindable;
use crate::render::camera::Camera;
use crate::render::display::FrameBuffer;
use crate::render::post::{PostProcessingEffectType, PostProcessingPipeline};
use crate::render::renderer::{ChunkRenderer, PlayerRenderer};

use math::vector::Vector3;

use core::world::World;

pub const SKY_COLOR: Vector3 = Vector3 {
    x: 0.455,
    y: 0.678,
    z: 0.984,
};

pub struct Renderer {
    framebuffer: FrameBuffer,
    player_renderer: PlayerRenderer,
    chunk_renderer: ChunkRenderer,
    post_pipeline: PostProcessingPipeline,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut post_pipeline = PostProcessingPipeline::new(width, height);
        post_pipeline.add(PostProcessingEffectType::FXAA);

        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Self {
            framebuffer: FrameBuffer::new(width, height, 1, true),
            player_renderer: PlayerRenderer::new(),
            chunk_renderer: ChunkRenderer::new(),
            post_pipeline,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.framebuffer = FrameBuffer::new(width, height, 1, true);
        self.post_pipeline.resize(width, height);
    }

    pub fn update(&mut self, world: &World, input: &InputHandler) {
        self.chunk_renderer.update(world, input);
    }

    pub fn draw<C: Camera>(&self, camera: &C, players: &Vec<&Player>) {
        self.framebuffer.bind();

        unsafe {
            gl::ClearColor(SKY_COLOR.x, SKY_COLOR.y, SKY_COLOR.z, 1.0);
        }

        self.framebuffer.clear(true, true, false);

        self.chunk_renderer.draw(camera);
        self.player_renderer.draw(camera, players);

        self.post_pipeline.apply(&self.framebuffer);

        // draw ui here
    }
}
