use crate::math::matrix::{Matrix4, Matrix};
use crate::math::vector::v3;

pub struct Transform {
    position: v3,
    rotation: v3,
    scale: v3,
    m: Option<Matrix4>,
}

impl Transform {
    #[allow(dead_code)]
    pub fn new_position(x: f32, y: f32, z: f32) -> Self {
        let mut t = Self {
            position: v3 { x, y, z },
            rotation: v3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: v3 {
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
    pub fn new(position: v3, rotation: v3, scale: v3) -> Self {
        let mut t = Self {
            position,
            rotation,
            scale,
            m: None,
        };

        t.generate_matrix();
        t
    }

    //#[allow(dead_code)]
    //pub fn new(position: v3, rotation: v3, scale: v3) -> Self {
    //    let ((cos_x, sin_x), (cos_y, sin_y), (cos_z, sin_z)) = (
    //        (rotation.z.to_radians().cos(), rotation.z.to_radians().sin()),
    //        (rotation.y.to_radians().cos(), rotation.y.to_radians().sin()),
    //        (rotation.x.to_radians().cos(), rotation.x.to_radians().sin()),
    //    );

    //    let m11 = cos_x * cos_y * scale.x;
    //    let m12 = cos_x * sin_y * sin_z - sin_x * cos_z;
    //    let m13 = cos_x * sin_y * cos_z + sin_x * sin_z;
    //    let m21 = sin_x * cos_y;
    //    let m22 = sin_x * sin_y * sin_z + cos_x * cos_z * scale.y;
    //    let m23 = sin_x * sin_y * cos_z - cos_x * sin_z;
    //    let m31 = -sin_y;
    //    let m32 = cos_y * sin_z;
    //    let m33 = cos_y * cos_z * scale.z;

    //    Self {
    //        m: Matrix4([
    //            [m11, m12, m13, position.x],
    //            [m21, m22, m23, position.y],
    //            [m31, m32, m33, position.z],
    //            [0.0, 0.0, 0.0, 1.0],
    //        ]),
    //        scale: scale,
    //    }
    //}

    #[allow(dead_code)]
    pub fn get_position(&self) -> v3 {
        self.position
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, value: v3) -> &mut Self {
        self.position = value;
        self.generate_matrix();
        self
    }

    #[allow(dead_code)]
    pub fn get_scale(&self) -> v3 {
        self.scale
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, value: v3) -> &mut Self {
        self.scale = value;
        self.generate_matrix();
        self
    }

    #[allow(dead_code)]
    pub fn get_euler_angles(&self) -> v3 {
        // https://www.gregslabaugh.net/publications/euler.pdf
        self.rotation

        //let tetha;
        //let psi;
        //let phi;

        //if self.m[2][0].abs() != 1.0 {
        //    tetha = -self.m[2][0].asin();
        //    let tcos = tetha.cos();

        //    psi = (self.m[2][1] / tcos).atan2((self.m[2][2] / self.scale.z) / tcos);
        //    phi = (self.m[1][0] / tcos).atan2((self.m[0][0] / self.scale.x) / tcos);
        //} else {
        //    phi = 0.0;
        //    if self.m[2][0] == -1.0 {
        //        tetha = PI / 2.0;
        //        psi = phi + self.m[0][1].atan2(self.m[0][2]);
        //    } else {
        //        tetha = -PI / 2.0;
        //        psi = -phi + (-self.m[0][1]).atan2(-self.m[0][2]);
        //    }
        //}

        //return v3 {
        //    x: tetha.to_degrees(),
        //    y: psi.to_degrees(),
        //    z: phi.to_degrees(),
        //};
    }

    #[allow(dead_code)]
    pub fn set_euler_angles(&mut self, value: v3) -> &mut Self {
        self.rotation = value;
        self.generate_matrix();
        self
    }

    fn generate_matrix(&mut self) {
        let ((cos_x, sin_x), (cos_y, sin_y), (cos_z, sin_z)) = (
            (
                self.rotation.z.to_radians().cos(),
                self.rotation.z.to_radians().sin(),
            ),
            (
                self.rotation.y.to_radians().cos(),
                self.rotation.y.to_radians().sin(),
            ),
            (
                self.rotation.x.to_radians().cos(),
                self.rotation.x.to_radians().sin(),
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

impl Matrix for Transform {
    fn get_matrix(&self) -> &Matrix4 {
        &self.m.as_ref().unwrap()
    }
}
