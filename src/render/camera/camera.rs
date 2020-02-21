use super::orthographic::OrthographicProjection;
use super::perspective::PerspectiveProjection;
use crate::components::Transform;
use crate::input::{keyboard, CursorDelta, InputHandler};
use crate::math::matrix::m4;
use crate::math::vector::v3;
use crate::utils::traits::Matrix;

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

    pub fn update(&mut self, input: &InputHandler, time_delta: &f32) {
        let speed = self.speed * time_delta;
        let mut delta = v3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        if input.is_key_pressed(keyboard::W) {
            delta.z += speed;
        }

        if input.is_key_pressed(keyboard::S) {
            delta.z -= speed;
        }

        if input.is_key_pressed(keyboard::A) {
            delta.x -= speed;
        }

        if input.is_key_pressed(keyboard::D) {
            delta.x += speed;
        }

        if input.is_key_pressed(keyboard::SPACE) {
            delta.y += speed;
        }

        if input.is_key_pressed(keyboard::LEFT_SHIFT) {
            delta.y -= speed;
        }

        self.move_by(delta);
    }

    fn move_by(&mut self, delta: v3) {
        self.transform
            .set_position(self.transform.get_position() - delta);
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

    // TODO: needed ?
    pub fn set_rotation(&mut self, rotation: v3) {
        self.transform.set_rotation(rotation);
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
