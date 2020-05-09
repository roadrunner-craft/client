use math::matrix::Matrix4;
use math::vector::Vector3;

pub struct Transform {
    position: Vector3,
    rotation: Vector3,
    scale: Vector3,
    m: Option<Matrix4>,
}

impl Transform {
    #[allow(dead_code)]
    pub fn new_position(x: f32, y: f32, z: f32) -> Self {
        let mut t = Self {
            position: Vector3 { x, y, z },
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            m: None,
        };

        t.generate_matrix();
        t
    }

    #[allow(dead_code)]
    pub fn new(position: Vector3, rotation: Vector3, scale: Vector3) -> Self {
        let mut t = Self {
            position,
            rotation,
            scale,
            m: None,
        };

        t.generate_matrix();
        t
    }

    pub fn get_position(&self) -> Vector3 {
        self.position
    }

    pub fn set_position(&mut self, value: Vector3) -> &mut Self {
        self.position = value;
        self.generate_matrix();
        self
    }

    pub fn get_scale(&self) -> Vector3 {
        self.scale
    }

    pub fn set_scale(&mut self, value: Vector3) -> &mut Self {
        self.scale = value;
        self.generate_matrix();
        self
    }

    pub fn get_euler_angles(&self) -> Vector3 {
        self.rotation
    }

    pub fn set_euler_angles(&mut self, value: Vector3) -> &mut Self {
        self.rotation = value;
        self.generate_matrix();
        self
    }

    pub fn get_matrix(&self) -> &Matrix4 {
        &self.m.as_ref().unwrap()
    }

    fn generate_matrix(&mut self) {
        let ((cos_x, sin_x), (cos_y, sin_y), (cos_z, sin_z)) = (
            (
                self.rotation.x.to_radians().cos(),
                self.rotation.x.to_radians().sin(),
            ),
            (
                self.rotation.y.to_radians().cos(),
                self.rotation.y.to_radians().sin(),
            ),
            (
                self.rotation.z.to_radians().cos(),
                self.rotation.z.to_radians().sin(),
            ),
        );

        let m00 = cos_x * cos_y * self.scale.x;
        let m01 = cos_x * sin_y * sin_z - sin_x * cos_z;
        let m02 = cos_x * sin_y * cos_z + sin_x * sin_z;
        let m10 = sin_x * cos_y;
        let m11 = sin_x * sin_y * sin_z + cos_x * cos_z * self.scale.y;
        let m12 = sin_x * sin_y * cos_z - cos_x * sin_z;
        let m20 = -sin_y;
        let m21 = cos_y * sin_z;
        let m22 = cos_y * cos_z * self.scale.z;

        let m03 = self.position.x;
        let m13 = self.position.y;
        let m23 = self.position.z;

        self.m = Some(Matrix4([
            [m00, m01, m02, m03],
            [m10, m11, m12, m13],
            [m20, m21, m22, m23],
            [0.0, 0.0, 0.0, 1.0],
        ]));
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new_position(0.0, 0.0, 0.0)
    }
}
