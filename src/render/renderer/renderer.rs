use crate::input::InputHandler;
use crate::render::camera::Camera;
use crate::render::renderer::ChunkRenderer;

use core::world::World;

pub struct Renderer {
    chunk_renderer: ChunkRenderer,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            chunk_renderer: ChunkRenderer::new(width, height),
        }
    }

    pub fn update(&mut self, world: &World, input: &InputHandler) {
        self.chunk_renderer.update(world, input);
    }

    pub fn draw<C: Camera>(&mut self, camera: &C) {
        unsafe {
            gl::ClearColor(116.0 / 255.0, 173.0 / 255.0, 251.0 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.chunk_renderer.draw(camera);
    }
}
