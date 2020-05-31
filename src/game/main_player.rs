use crate::input::InputHandler;
use crate::render::camera::{Camera, PerspectiveCamera};

use core::world::WorldCoordinate;
use glutin::event::VirtualKeyCode;
use math::vector::Vector3;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashMap;

const SPEED: f64 = 20.0;
const SENSITIVITY: f32 = 0.2;

pub struct MainPlayer {
    pub camera: PerspectiveCamera,
    event_handlers: Vec<(Receiver<VirtualKeyCode>, fn(&mut MainPlayer))>,
    xaxis: f32,
    yaxis: f32,
    zaxis: f32,
}

impl MainPlayer {
    pub fn new(position: WorldCoordinate, input_handler: &mut InputHandler) -> Self {
        let mut p = Self {
            camera: PerspectiveCamera::new(70.0, 0.1, 1024.0),
            event_handlers: Vec::new(),
            xaxis: 0.0,
            yaxis: 0.0,
            zaxis: 0.0,
        };

        p.register_move_forward(input_handler);
        p.register_move_backward(input_handler);
        p.register_move_left(input_handler);
        p.register_move_right(input_handler);
        p.set_position(position);
        p
    }

    fn register_move_forward(&mut self, input: &mut InputHandler) {
        let (sender, receiver) = channel();
        input.register(VirtualKeyCode::W, sender);
        self.event_handlers.push((receiver, |player| {
            player.zaxis += 1.0;
        }));
    }

   fn register_move_backward(&mut self, input: &mut InputHandler) {
        let (sender, receiver) = channel();
        input.register(VirtualKeyCode::S, sender);
        self.event_handlers.push((receiver, |player| {
            player.zaxis -= 1.0;
        }));
    }

   fn register_move_left(&mut self, input: &mut InputHandler) {
        let (sender, receiver) = channel();
        input.register(VirtualKeyCode::A, sender);
        self.event_handlers.push((receiver, |player| {
            player.xaxis -= 1.0;
        }));
    }

   fn register_move_right(&mut self, input: &mut InputHandler) {
        let (sender, receiver) = channel();
        input.register(VirtualKeyCode::D, sender);
        self.event_handlers.push((receiver, |player| {
            player.xaxis += 1.0;
        }));
    }


    fn set_position(&mut self, position: WorldCoordinate) {
        self.camera.set_position(position);
    }

    pub fn position(&self) -> Vector3 {
        self.camera.position()
    }

    pub fn update(&mut self, time_delta: f64, input: &InputHandler) {

        self.xaxis = 0.0;
        self.yaxis = 0.0;
        self.zaxis = 0.0;

        let cursor_delta = input.get_cursor_delta();
        let camera_delta = Vector3 {
            x: cursor_delta.y as f32,
            y: cursor_delta.x as f32,
            z: 0.0,
        } * SENSITIVITY;
        let mut camera_angles = self.camera.euler_angles() + camera_delta;

        if camera_angles.x > 90.0 {
            camera_angles.x = 90.0;
        } else if camera_angles.x < -90.0 {
            camera_angles.x = -90.0;
        }

        camera_angles.y %= 360.0;

        self.camera.set_euler_angles(camera_angles);

        let events = self.event_handlers.iter().filter(|(receiver, _)| {
            receiver.try_recv().is_ok()
        });

        let handlers = events.map(|(_, func)| {
            func.clone()
        }).collect::<Vec<fn(&mut MainPlayer)>>();

        for handler in handlers {
            (handler)(self);
        }

        let angle = self.camera.euler_angles().y.to_radians();

        let mut delta = Vector3 {
            x: self.xaxis * angle.cos() + self.zaxis * angle.sin(),
            y: self.yaxis,
            z: -self.xaxis * angle.sin() + self.zaxis * angle.cos(),
        };
        delta = delta * (SPEED * time_delta) as f32;

        self.set_position(self.camera.position() + delta);


    }
}
