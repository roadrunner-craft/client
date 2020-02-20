use crate::math::matrix::m4;

pub trait Bindable {
    fn bind(&self);
    fn unbind(&self);
}

pub trait Matrix {
    fn get_matrix(&self) -> &m4;
}
