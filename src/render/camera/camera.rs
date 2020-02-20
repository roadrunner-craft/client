use super::orthographic::OrthographicProjection;
use super::perspective::PerspectiveProjection;
use crate::components::Transform;
use crate::math::matrix::m4;
use crate::math::vector::v3;
use crate::utils::traits::Matrix;

pub struct PerspectiveCamera {
    transform: Transform,
    projection: PerspectiveProjection,
}

impl PerspectiveCamera {
    pub fn new(fov: f32, near: f32, far: f32, aspect_ratio: f32) -> PerspectiveCamera {
        Self {
            transform: Transform::default(),
            projection: PerspectiveProjection::new(fov, near, far, aspect_ratio),
        }
    }

    pub fn set_position(&mut self, position: v3) {
        self.transform.set_position(-position);
    }

    pub fn set_rotation(&mut self, rotation: v3) {
        self.transform.set_rotation(rotation);
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.projection.set_aspect_ratio(aspect_ratio);
    }

    pub fn get_projection(&self) -> &m4 {
        self.projection.get_matrix()
    }

    pub fn get_view(&self) -> &m4 {
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
    pub fn set_position(&mut self, position: v3) {
        self.transform.set_position(-position);
    }

    // TODO: needed ?
    pub fn set_rotation(&mut self, rotation: v3) {
        self.transform.set_rotation(rotation);
    }

    pub fn get_projection(&self) -> &m4 {
        self.projection.get_matrix()
    }

    pub fn get_view(&self) -> &m4 {
        self.transform.get_matrix()
    }
}
