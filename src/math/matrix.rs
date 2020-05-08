mod m4;
pub use self::m4::M4;

pub trait Matrix {
    fn get_matrix(&self) -> &M4;
}

