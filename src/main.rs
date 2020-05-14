mod game;
mod input;
mod ops;
mod render;
mod utils;

extern crate core;
extern crate gl;
extern crate glutin;
extern crate image;
extern crate math;
extern crate serde;
extern crate serde_json;

use crate::game::Player;
use crate::input::InputHandler;
use crate::render::display::Display;
use crate::render::renderer::Renderer;

use core::world::{World, WorldCoordinate};
use glutin::event::{DeviceEvent, Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::time::Instant;

const FPS_REFRESH_TIMEOUT: u64 = 1;
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() {
    let event_loop = EventLoop::new();

    let display = Display::new(PKG_NAME, &event_loop);
    let size = display.size();
    let mut renderer = Renderer::new(size.0, size.1);
    let mut input_handler = InputHandler::default();

    let mut world = World::new();
    let mut player = Player::new(WorldCoordinate {
        x: 0.0,
        y: 150.0,
        z: 0.0,
    });

    let mut fps: u32 = 0;
    let mut last_time = Instant::now();
    let mut last_fps_update = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::LoopDestroyed => return,
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                display.resize(size);
                player
                    .camera
                    .set_aspect_ratio(size.width as f32 / size.height as f32);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                display.resize(*new_inner_size);
                player
                    .camera
                    .set_aspect_ratio(new_inner_size.width as f32 / new_inner_size.height as f32);
            }
            WindowEvent::KeyboardInput { input, .. } => input_handler.process_keyboard(input),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => input_handler.process_cursor(delta),
            _ => (),
        },
        Event::MainEventsCleared => {
            let time_delta = last_time.elapsed().as_secs_f64();
            last_time = Instant::now();

            if last_fps_update.elapsed().as_secs() >= FPS_REFRESH_TIMEOUT {
                fps = (1.0 / time_delta) as u32;
                println!("FPS: {}", fps);
                last_fps_update = Instant::now();
            }

            player.update(time_delta, &input_handler);
            world.load_around(vec![player.position()]);
            renderer.update(&world, player.position(), &input_handler);
            input_handler.clear();
            display.request_redraw();
        }
        Event::RedrawRequested(_) => {
            renderer.draw(&player.camera);
            display.swap_buffers();
        }
        _ => (),
    });
}
