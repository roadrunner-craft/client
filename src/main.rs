mod components;
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
    let size = display.context.window().inner_size();
    let mut renderer = Renderer::init(size.width, size.height);

    // TODO: remove this temporary data
    use crate::math::vector::v3;
    use crate::render::models::Cube;
    use crate::render::Texture;

    let dirt = Texture::new();
    let cube = Cube::new();
    //   use crate::utils::traits::Bindable;
    // dirt.unbind();

    event_loop.run(move |event, _, control_flow| match event {
        Event::LoopDestroyed => return,
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => renderer.set_size(size.width, size.height),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::RedrawRequested(_) => {
            renderer.draw(&cube);
            display.context.swap_buffers().unwrap();
        }
        _ => (),
    });
}
