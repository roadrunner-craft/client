mod matrix4;
pub use self::matrix4::Matrix4;

pub trait Matrix {
    fn get_matrix(&self) -> &Matrix4;
}

