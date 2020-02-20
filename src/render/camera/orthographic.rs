use crate::math::matrix::m4;
use crate::utils::traits::Matrix;

pub struct OrthographicProjection {
    m: m4,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    near: f32,
    far: f32,
}

impl OrthographicProjection {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        unimplemented!();

        let mut projection = Self {
            m: m4::identity(),
            left,
            right,
            top,
            bottom,
            near,
            far,
        };

        projection.generate_matrix();
        projection
    }

    pub fn set_planes(
        &mut self,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) {
        self.left = left;
        self.right = right;
        self.top = top;
        self.bottom = bottom;
        self.near = near;
        self.far = far;
        self.generate_matrix();
    }

    fn generate_matrix(&mut self) {
        self.m[0][0] = 2.0 / (self.right - self.left);
        self.m[1][1] = 2.0 / (self.top - self.bottom);
        self.m[2][2] = -2.0 / (self.far - self.near);
        self.m[0][3] = -(self.right + self.left) / (self.right - self.left);
        self.m[1][3] = -(self.top + self.bottom) / (self.top - self.bottom);
        self.m[2][3] = -(self.far + self.near) / (self.far - self.near);
    }
}

impl Matrix for OrthographicProjection {
    fn get_matrix(&self) -> &m4 {
        &self.m
    }
}
