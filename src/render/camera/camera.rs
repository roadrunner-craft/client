use super::perspective::PerspectiveProjection;
use crate::input::InputHandler;
use crate::math::matrix::{m4, Matrix};
use crate::math::vector::v3;
use crate::render::camera::ViewMatrix;

use scancode::Scancode;

const SPEED: f32 = 7.0;
const SENSITIVITY: f32 = 0.2;

pub trait Camera {
    fn get_view(&self) -> &m4;
    fn get_projection(&self) -> &m4;
}

pub struct PerspectiveCamera {
    view: ViewMatrix,
    projection: PerspectiveProjection,
    speed: f32,
}

impl PerspectiveCamera {
    pub fn new(fov: f32, near: f32, far: f32, aspect_ratio: f32) -> PerspectiveCamera {
        Self {
            // TODO: change this to take the position as a parameter or add a method
            view: ViewMatrix::new_position(0.0, 30.0, 0.0),
            projection: PerspectiveProjection::new(fov, near, far, aspect_ratio),
            speed: SPEED,
        }
    }

    pub fn update<'a>(&mut self, input: &InputHandler, time_delta: &f32) {
        // TODO: move this code into a player entity
        let cursor_delta = input.get_cursor_delta();
        let camera_delta = v3 {
            x: cursor_delta.y as f32,
            y: cursor_delta.x as f32,
            z: 0.0,
        } * SENSITIVITY;
        let mut camera_angles = self.view.get_euler_angles() + camera_delta;

        if camera_angles.x > 90.0 {
            camera_angles.x = 90.0;
        } else if camera_angles.x < -90.0 {
            camera_angles.x = -90.0;
        }

        camera_angles.y %= 360.0;

        self.view.set_euler_angles(camera_angles);

        let mut xaxis = 0.0;
        let mut yaxis = 0.0;
        let mut zaxis = 0.0;

        if input.is_key_pressed(Scancode::W) {
            zaxis += 1.0;
        }

        if input.is_key_pressed(Scancode::S) {
            zaxis -= 1.0;
        }

        if input.is_key_pressed(Scancode::A) {
            xaxis -= 1.0;
        }

        if input.is_key_pressed(Scancode::D) {
            xaxis += 1.0;
        }

        if input.is_key_pressed(Scancode::Space) {
            yaxis += 1.0;
        }

        if input.is_key_pressed(Scancode::LeftShift) {
            yaxis -= 1.0;
        }

        let angle = self.view.get_euler_angles().y.to_radians();

        let mut delta = v3 {
            x: xaxis * angle.cos() + zaxis * angle.sin(),
            y: yaxis,
            z: -xaxis * angle.sin() + zaxis * angle.cos(),
        };
        delta = delta * (self.speed * time_delta);

        self.view.set_position(self.view.get_position() + delta);
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.projection.set_aspect_ratio(aspect_ratio);
    }
}

impl Camera for PerspectiveCamera {
    fn get_projection(&self) -> &m4 {
        self.projection.get_matrix()
    }

    fn get_view(&self) -> &m4 {
        self.view.get_matrix()
    }
}
