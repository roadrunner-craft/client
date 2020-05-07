use crate::math::matrix::{m4, Matrix};

pub struct PerspectiveProjection {
    m: m4,
    fov: f32,
    near: f32,
    far: f32,
    aspect_ratio: f32,
}

impl PerspectiveProjection {
    pub fn new(fov: f32, near: f32, far: f32, aspect_ratio: f32) -> Self {
        let mut projection = Self {
            m: m4::identity(),
            fov,
            near,
            far,
            aspect_ratio,
        };

        projection.generate_matrix();
        projection
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.generate_matrix();
    }

    fn generate_matrix(&mut self) {
        let y_scale = 1.0 / (self.fov.to_radians() / 2.0).tan();
        let x_scale = y_scale / self.aspect_ratio;
        let frustum_length = self.far - self.near;

        self.m[0][0] = x_scale;
        self.m[1][1] = y_scale;
        self.m[2][2] = (self.far + self.near) / frustum_length;
        self.m[3][2] = 1.0;
        self.m[2][3] = (-2.0 * self.far * self.near) / frustum_length;
        self.m[3][3] = 0.0;
    }
}

impl Matrix for PerspectiveProjection {
    fn get_matrix(&self) -> &m4 {
        &self.m
    }
}
