use crate::math::matrix::m4;
use crate::math::vector::v3;
use crate::utils::traits::Matrix;

pub struct Transform {
    m: m4,
    scale: v3,
}

impl Transform {
    #[allow(dead_code)]
    pub fn new_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            m: m4([
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            scale: v3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    #[allow(dead_code)]
    pub fn new(position: v3, rotation: v3, scale: v3) -> Self {
        let ((cos_x, sin_x), (cos_y, sin_y), (cos_z, sin_z)) = (
            (rotation.z.to_radians().cos(), rotation.z.to_radians().sin()),
            (rotation.y.to_radians().cos(), rotation.y.to_radians().sin()),
            (rotation.x.to_radians().cos(), rotation.x.to_radians().sin()),
        );

        let m11 = cos_x * cos_y * scale.x;
        let m12 = cos_x * sin_y * sin_z - sin_x * cos_z;
        let m13 = cos_x * sin_y * cos_z + sin_x * sin_z;
        let m21 = sin_x * cos_y;
        let m22 = sin_x * sin_y * sin_z + cos_x * cos_z * scale.y;
        let m23 = sin_x * sin_y * cos_z - cos_x * sin_z;
        let m31 = -sin_y;
        let m32 = cos_y * sin_z;
        let m33 = cos_y * cos_z * scale.z;

        Self {
            m: m4([
                [m11, m12, m13, position.x],
                [m21, m22, m23, position.y],
                [m31, m32, m33, position.z],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            scale: scale,
        }
    }

    #[allow(dead_code)]
    pub fn get_position(&self) -> v3 {
        v3 {
            x: self.m[0][3],
            y: self.m[1][3],
            z: self.m[2][3],
        }
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, value: v3) -> &mut Self {
        self.m[0][3] = value.x;
        self.m[1][3] = value.y;
        self.m[2][3] = value.z;
        self
    }

    #[allow(dead_code)]
    pub fn get_scale(&self) -> v3 {
        self.scale
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, value: v3) -> &mut Self {
        self.m[0][0] = self.m[1][1] / self.scale.x * value.x;
        self.m[1][1] = self.m[2][2] / self.scale.y * value.y;
        self.m[2][2] = self.m[3][3] / self.scale.z * value.z;
        self.scale = value;
        self
    }

    #[allow(dead_code)]
    pub fn get_euler_rotation() -> v3 {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub fn set_rotation(&mut self, value: v3) -> &mut Self {
        let ((cos_x, sin_x), (cos_y, sin_y), (cos_z, sin_z)) = (
            (value.z.to_radians().cos(), value.z.to_radians().sin()),
            (value.y.to_radians().cos(), value.y.to_radians().sin()),
            (value.x.to_radians().cos(), value.x.to_radians().sin()),
        );

        self.m[0][0] = cos_x * cos_y * self.scale.x;
        self.m[0][1] = cos_x * sin_y * sin_z - sin_x * cos_z;
        self.m[0][2] = cos_x * sin_y * cos_z + sin_x * sin_z;
        self.m[1][0] = sin_x * cos_y;
        self.m[1][1] = sin_x * sin_y * sin_z + cos_x * cos_z * self.scale.y;
        self.m[1][2] = sin_x * sin_y * cos_z - cos_x * sin_z;
        self.m[2][0] = -sin_y;
        self.m[2][1] = cos_y * sin_z;
        self.m[2][2] = cos_y * cos_z * self.scale.z;
        self
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            m: m4::identity(),
            scale: v3::identity(),
        }
    }
}

impl Matrix for Transform {
    fn get_matrix(&self) -> &m4 {
        &self.m
    }
}
