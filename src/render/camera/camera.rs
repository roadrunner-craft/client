use crate::render::camera::perspective::PerspectiveProjection;

use math::matrix::Matrix4;
use math::transform::Transform;
use math::vector::Vector3;

pub trait Camera {
    fn get_view(&self) -> &Matrix4;
    fn get_projection(&self) -> &Matrix4;
}

pub struct PerspectiveCamera {
    transform: Transform,
    projection: PerspectiveProjection,
}

impl PerspectiveCamera {
    pub fn new(fov: f32, near: f32, far: f32) -> PerspectiveCamera {
        Self {
            transform: Transform::default(),
            projection: PerspectiveProjection::new(fov, near, far),
        }
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.transform.set_position(-position);
    }

    pub fn position(&self) -> Vector3 {
        -self.transform.position()
    }

    pub fn set_euler_angles(&mut self, rotation: Vector3) {
        self.transform.set_euler_angles(rotation);
    }

    pub fn euler_angles(&self) -> Vector3 {
        self.transform.euler_angle()
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.projection.set_aspect_ratio(aspect_ratio);
    }
}

impl Camera for PerspectiveCamera {
    fn get_projection(&self) -> &Matrix4 {
        self.projection.get_matrix()
    }

    fn get_view(&self) -> &Matrix4 {
        self.transform.get_matrix()
    }
}
