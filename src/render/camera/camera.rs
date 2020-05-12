use crate::render::camera::perspective::PerspectiveProjection;

use math::container::Frustum;
use math::matrix::Matrix4;
use math::transform::Transform;
use math::vector::Vector3;

pub trait Camera {
    fn position(&self) -> Vector3;
    fn view(&self) -> &Matrix4;
    fn projection(&self) -> &Matrix4;
    fn projection_view(&self) -> &Matrix4;
    fn frustum(&self) -> &Frustum;
}

pub struct PerspectiveCamera {
    transform: Transform,
    projection: PerspectiveProjection,
    projection_view: Matrix4,
    frustum: Frustum,
}

impl PerspectiveCamera {
    pub fn new(fov: f32, near: f32, far: f32) -> PerspectiveCamera {
        let transform = Transform::default();
        let projection = PerspectiveProjection::new(fov, near, far);
        let projection_view = *projection.matrix() * *transform.matrix();

        Self {
            transform,
            projection,
            projection_view: projection_view,
            frustum: Frustum::new(&projection_view),
        }
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.transform.set_position(-position);
        self.update();
    }

    pub fn set_euler_angles(&mut self, rotation: Vector3) {
        self.transform.set_euler_angles(rotation);
        self.update();
    }

    pub fn euler_angles(&self) -> Vector3 {
        self.transform.euler_angle()
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.projection.set_aspect_ratio(aspect_ratio);
        self.update();
    }

    fn update(&mut self) {
        self.projection_view = *self.projection.matrix() * *self.transform.matrix();
        self.frustum.update(&self.projection_view);
    }
}

impl Camera for PerspectiveCamera {
    fn position(&self) -> Vector3 {
        -self.transform.position()
    }

    fn view(&self) -> &Matrix4 {
        self.transform.matrix()
    }

    fn projection(&self) -> &Matrix4 {
        self.projection.matrix()
    }

    fn projection_view(&self) -> &Matrix4 {
        &self.projection_view
    }

    fn frustum(&self) -> &Frustum {
        &self.frustum
    }
}
