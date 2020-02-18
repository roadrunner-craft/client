use crate::components::Transform;
use crate::math::matrix::m4;
use crate::render::camera::ProjectionMatrix;

pub struct Camera {
    pub transform: Transform,
    projection: Box<dyn ProjectionMatrix>,
}

impl Camera {
    pub fn new(projection: Box<dyn ProjectionMatrix>) -> Camera {
        Self {
            transform: Transform::default(),
            projection: projection,
        }
    }

    pub fn projection_matrix(&self) -> &m4 {
        self.projection.get_matrix()
    }
}
