mod game;
mod input;
mod ops;
mod render;
mod utils;

extern crate bincode;
extern crate core;
extern crate gl;
extern crate glutin;
extern crate image;
extern crate math;
extern crate serde;
extern crate serde_json;

use crate::game::entity::Player;
use crate::game::MainPlayer;
use crate::input::InputHandler;
use crate::input::NetworkHandler;
use crate::render::display::Display;
use crate::render::renderer::Renderer;

use core::events::{ClientEvent, ServerEvent};
use core::world::{World, WorldCoordinate};
use glutin::event::{DeviceEvent, Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use math::vector::Vector3;
use std::collections::HashMap;
use std::io;
use std::time::Instant;

const FPS_REFRESH_TIMEOUT: u64 = 1;
pub const NETWORK_REFRESH_TIMEOUT: u128 = 50;
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() -> io::Result<()> {
    let event_loop = EventLoop::new();

    let display = Display::new(PKG_NAME, &event_loop);
    let (width, height) = display.size();
    let mut renderer = Renderer::new(width, height);
    let mut input_handler = InputHandler::default();
    let network_handler = NetworkHandler::new()?;

    let mut world = World::new();
    let mut players: HashMap<u128, Player> = HashMap::new();
    let mut player = MainPlayer::new(WorldCoordinate {
        x: 0.0,
        y: 150.0,
        z: 0.0,
    });

    network_handler.send(ClientEvent::PlayerConnect);

    let mut fps: u32 = 0;
    let mut last_time = Instant::now();
    let mut last_fps_update = Instant::now();
    let mut last_network_update = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::LoopDestroyed => return,
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                display.resize(size);
                renderer.resize(size.width as usize, size.height as usize);
                player
                    .camera
                    .set_aspect_ratio(size.width as f32 / size.height as f32);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                display.resize(*new_inner_size);
                renderer.resize(
                    new_inner_size.width as usize,
                    new_inner_size.height as usize,
                );
                player
                    .camera
                    .set_aspect_ratio(new_inner_size.width as f32 / new_inner_size.height as f32);
            }
            WindowEvent::KeyboardInput { input, .. } => input_handler.process_keyboard(input),
            WindowEvent::CloseRequested => {
                network_handler.send(ClientEvent::PlayerDisconnect);
                *control_flow = ControlFlow::Exit
            }
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

            if let Ok(events) = network_handler.process() {
                for event in events {
                    match event {
                        ServerEvent::PlayerConnected { id } => {
                            players.insert(id, Player::new(id));
                        }
                        ServerEvent::PlayerList { ids } => {
                            for id in ids.iter() {
                                players.insert(*id, Player::new(*id));
                            }
                        }
                        ServerEvent::PlayerDisconnected { id } => {
                            players.remove(&id);
                        }
                        ServerEvent::PlayerMoved { id, position } => {
                            if let Some(player) = players.get_mut(&id) {
                                player.set_position(position);
                            };
                        }
                    };
                }
            } else {
                println!("could not process network events");
            }

            player.update(time_delta, &input_handler);

            for (_, player) in players.iter_mut() {
                player.update(time_delta);
            }

            world.load_around(vec![player.position()]);
            renderer.update(&world, &input_handler);
            input_handler.clear();

            if last_network_update.elapsed().as_millis() >= NETWORK_REFRESH_TIMEOUT {
                network_handler.send(ClientEvent::PlayerMove {
                    position: player.position(),
                });

                last_network_update = Instant::now();
            }

            display.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let pals = players.values().collect::<Vec<&Player>>();
            renderer.draw(&player.camera, &pals);
            display.swap_buffers();
        }
        _ => (),
    });
}
