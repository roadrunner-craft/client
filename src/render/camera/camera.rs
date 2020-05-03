use super::orthographic::OrthographicProjection;
use super::perspective::PerspectiveProjection;
use crate::components::Transform;
use crate::input::{CursorDelta, InputHandler};
use crate::math::matrix::m4;
use crate::math::vector::v3;
use crate::utils::traits::Matrix;

use scancode::Scancode;

const SPEED: f32 = 5.0;

pub trait Camera {
    fn get_view(&self) -> &m4;
    fn get_projection(&self) -> &m4;
}

pub struct PerspectiveCamera {
    transform: Transform,
    projection: PerspectiveProjection,
    speed: f32,
}

impl PerspectiveCamera {
    pub fn new(fov: f32, near: f32, far: f32, aspect_ratio: f32) -> PerspectiveCamera {
        Self {
            transform: Transform::default(),
            projection: PerspectiveProjection::new(fov, near, far, aspect_ratio),
            speed: SPEED,
        }
    }

    pub fn update<'a>(&mut self, input: &InputHandler, time_delta: &f32) {
        let speed = self.speed * time_delta;
        let mut delta = v3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        if input.is_key_pressed(Scancode::W) {
            delta.z += speed;
        }

        if input.is_key_pressed(Scancode::S) {
            delta.z -= speed;
        }

        if input.is_key_pressed(Scancode::A) {
            delta.x -= speed;
        }

        if input.is_key_pressed(Scancode::D) {
            delta.x += speed;
        }

        if input.is_key_pressed(Scancode::Space) {
            delta.y += speed;
        }

        if input.is_key_pressed(Scancode::LeftShift) {
            delta.y -= speed;
        }

        self.transform
            .set_position(self.transform.get_position() - delta);

        let cursor_delta = input.get_cursor_delta();

        self.transform.set_euler_angles(
            self.transform.get_euler_angles()
                - v3 {
                    x: cursor_delta.y as f32,
                    y: cursor_delta.x as f32,
                    z: 0.0,
                },
        );
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
        self.transform.get_matrix()
    }
}

pub struct OrthographicCamera {
    transform: Transform,
    projection: OrthographicProjection,
}

impl OrthographicCamera {
    pub fn new(
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) -> OrthographicCamera {
        Self {
            transform: Transform::default(),
            projection: OrthographicProjection::new(left, right, top, bottom, near, far),
        }
    }

    // TODO: maybe use a v2 as position ?
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.transform.set_position(v3 {
            x: -x,
            y: -y,
            z: -z,
        });
    }
}

impl Camera for OrthographicCamera {
    fn get_projection(&self) -> &m4 {
        self.projection.get_matrix()
    }

    fn get_view(&self) -> &m4 {
        self.transform.get_matrix()
    }
}
