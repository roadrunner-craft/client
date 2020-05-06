use crate::game::Game;
use crate::input::InputHandler;
use crate::render::camera::Camera;
use crate::render::chunk_renderer::ChunkRenderer;
use crate::render::Display;

#[derive(Default)]
pub struct Renderer {
    chunk_renderer: ChunkRenderer,
}

impl Renderer {
    pub fn set_size(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn update(&mut self, input: &InputHandler) {
        self.chunk_renderer.update(input);
    }

    pub fn draw<C: Camera>(&mut self, display: &Display, camera: &C, game: &Game) {
        unsafe {
            gl::ClearColor(116.0 / 255.0, 173.0 / 255.0, 251.0 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.chunk_renderer.draw(camera, game);

        display.context.swap_buffers().unwrap();
    }
}
