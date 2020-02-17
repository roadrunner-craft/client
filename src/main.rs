mod math;
mod render;
mod utils;

extern crate gl;
extern crate glutin;
extern crate image;

use crate::render::display::Display;
use crate::render::renderer::Renderer;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new();
    let display = Display::create("sick opengl shitshow", &event_loop);
    let renderer = Renderer::init();

    // TODO: remove this temporary data
    use crate::math::vector::v3;
    use crate::render::models::Quad;
    use crate::render::Texture;

    let dirt = Texture::new();

    let quad = Quad::new(
        v3 {
            x: -0.5,
            y: 0.5,
            z: 0.0,
        },
        v3 {
            x: -0.5,
            y: -0.5,
            z: 0.0,
        },
        v3 {
            x: 0.5,
            y: -0.5,
            z: 0.0,
        },
        v3 {
            x: 0.5,
            y: 0.5,
            z: 0.0,
        },
    );

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = &event
        {
            *control_flow = ControlFlow::Exit;
            return;
        }

        renderer.draw(&quad);

        display.context.swap_buffers().unwrap();
    });
}
