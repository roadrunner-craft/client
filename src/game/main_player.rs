use crate::input::{InputHandler, InputObserver, InputCallback};
use crate::render::camera::{Camera, PerspectiveCamera};

use core::world::WorldCoordinate;
use math::vector::Vector3;

mod input_handlers;

const SPEED: f64 = 20.0;
const SENSITIVITY: f32 = 0.2;

pub struct MainPlayer {
    pub camera: PerspectiveCamera,
    input_callbacks: Vec<InputCallback<Self>>,
    is_moving_forward: bool,
    is_moving_backward: bool,
    is_moving_left: bool,
    is_moving_right: bool,
    is_moving_up: bool,
    is_moving_down: bool,
}

impl MainPlayer {
    pub fn new(position: WorldCoordinate, input: &mut InputHandler) -> Self {
        let mut p = Self {
            camera: PerspectiveCamera::new(70.0, 0.1, 1024.0),
            input_callbacks: Vec::new(),
            is_moving_forward: false,
            is_moving_backward: false,
            is_moving_left: false,
            is_moving_right: false,
            is_moving_up: false,
            is_moving_down: false,
        };

        p.register_input_handlers(input);
        p.set_position(position);
        p
    }

    fn set_position(&mut self, position: WorldCoordinate) {
        self.camera.set_position(position);
    }

    pub fn position(&self) -> Vector3 {
        self.camera.position()
    }

    pub fn update(&mut self, time_delta: f64, input: &InputHandler) {

        let mut xaxis = 0.0;
        let mut yaxis = 0.0;
        let mut zaxis = 0.0;

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

        self.process_inputs();

        if self.is_moving_forward { zaxis += 1.0; }
        if self.is_moving_backward { zaxis -= 1.0; }
        if self.is_moving_left { xaxis -= 1.0; }
        if self.is_moving_right { xaxis += 1.0; }
        if self.is_moving_up { yaxis += 1.0; }
        if self.is_moving_down { yaxis -= 1.0; }

        let angle = self.camera.euler_angles().y.to_radians();

        let mut delta = Vector3 {
            x: xaxis * angle.cos() + zaxis * angle.sin(),
            y: yaxis,
            z: -xaxis * angle.sin() + zaxis * angle.cos(),
        };
        delta = delta * (SPEED * time_delta) as f32;

        self.set_position(self.camera.position() + delta);
    }
}
