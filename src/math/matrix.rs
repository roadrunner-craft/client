mod m4;
pub use self::m4::Matrix4;

pub trait Matrix {
    fn get_matrix(&self) -> &Matrix4;
}

