mod camera;
mod orthographic;
mod perspective;

pub use self::camera::Camera;
pub use self::orthographic::Orthographic;
pub use self::perspective::Perspective;

use crate::math::matrix::m4;

pub trait ProjectionMatrix {
    fn get_matrix(&self) -> &m4;
}
