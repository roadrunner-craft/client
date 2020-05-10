mod components;
mod input;
mod render;
mod texture;
mod utils;

extern crate core;
extern crate gl;
extern crate glutin;
extern crate image;
extern crate math;
extern crate serde;
extern crate serde_json;

use crate::input::InputHandler;
use crate::render::camera::PerspectiveCamera;
use crate::render::renderer::Renderer;
use crate::render::Display;

use core::Game;
use glutin::event::{DeviceEvent, Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::time::Instant;

const FPS_REFRESH_TIMEOUT: u64 = 1;
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() {
    let event_loop = EventLoop::new();

    let display = Display::new(PKG_NAME, &event_loop);
    let mut renderer = Renderer::default();
    let mut camera = PerspectiveCamera::new(70.0, 0.1, 1024.0, 1.0);
    let mut input_handler = InputHandler::default();

    let mut game = Game::new();
    game.add_player(String::from("xehos"));

    let mut fps: u32 = 0;
    let mut last_time = Instant::now();
    let mut last_fps_update = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::LoopDestroyed => return,
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                display.resize(size);
                camera.set_aspect_ratio(size.width as f32 / size.height as f32);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                display.resize(*new_inner_size);
                camera.set_aspect_ratio(new_inner_size.width as f32 / new_inner_size.height as f32);
            }
            WindowEvent::KeyboardInput { input, .. } => input_handler.process_keyboard(input),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => input_handler.process_cursor(delta),
            _ => (),
        },
        Event::RedrawRequested(_) => {
            renderer.draw(&camera);
            display.swap_buffers();
        }
        Event::MainEventsCleared => {
            let time_delta = last_time.elapsed().as_secs_f64();
            last_time = Instant::now();

            if last_fps_update.elapsed().as_secs() >= FPS_REFRESH_TIMEOUT {
                fps = (1.0 / time_delta) as u32;
                println!("FPS: {}", fps);
                last_fps_update = Instant::now();
            }

            camera.update(&input_handler, time_delta);
            game.update(time_delta);
            renderer.update(&game);
            input_handler.clear_cursor_delta();
            display.request_redraw();
        }
        _ => (),
    });
}
