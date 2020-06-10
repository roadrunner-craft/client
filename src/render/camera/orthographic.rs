use math::matrix::Matrix4;

pub struct OrthographicProjection {
    m: Matrix4,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
}

impl OrthographicProjection {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut projection = Self {
            m: Matrix4::identity(),
            left,
            right,
            bottom,
            top,
            near,
            far,
        };

        projection.generate_matrix();
        projection
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.right = width as f32;
        self.bottom = height as f32;

        self.generate_matrix();
    }

    pub fn matrix(&self) -> &Matrix4 {
        &self.m
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
