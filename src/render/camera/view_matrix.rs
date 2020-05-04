use crate::math::matrix::m4;
use crate::math::vector::v3;
use crate::utils::traits::Matrix;

// TODO: merge this class with the transformation component
pub struct ViewMatrix {
    position: v3,
    rotation: v3,
    m: Option<m4>,
}

impl ViewMatrix {
    pub fn new_position(x: f32, y: f32, z: f32) -> Self {
        let mut view = Self {
            position: v3 { x, y, z },
            rotation: v3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            m: None,
        };

        view.generate_matrix();
        view
    }

    pub fn new(position: v3, rotation: v3) -> Self {
        let mut view = Self {
            position,
            rotation,
            m: None,
        };

        view.generate_matrix();
        view
    }

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
    pub fn get_euler_angles(&self) -> v3 {
        self.rotation
    }

    #[allow(dead_code)]
    pub fn set_euler_angles(&mut self, value: v3) -> &mut Self {
        self.rotation = value;
        self.generate_matrix();
        self
    }

    fn generate_matrix(&mut self) {
        let ((cx, sx), (cy, sy), (cz, sz)) = (
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

        let mut m = m4::identity();

        // TODO: reduce this into a single m4 assignment
        m = m * m4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cx, sx, 0.0],
            [0.0, -sx, cx, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        m = m * m4([
            [cy, 0.0, -sy, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sy, 0.0, cy, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        m = m * m4([
            [cz, sz, 0.0, 0.0],
            [-sz, cz, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        m = m * m4([
            [1.0, 0.0, 0.0, -self.position.x],
            [0.0, 1.0, 0.0, -self.position.y],
            [0.0, 0.0, 1.0, -self.position.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        self.m = Some(m);

        return;

        // let xaxis = v3 {
        //     x: cos_x * cos_y,
        //     y: cos_x * sin_y * sin_z - sin_x * cos_z,
        //     z: cos_x * sin_y * cos_z + sin_x * sin_z,
        // };
        // let yaxis = v3 {
        //     x: sin_x * cos_y,
        //     y: sin_x * sin_y * sin_z + cos_x * cos_z,
        //     z: sin_x * sin_y * cos_z - cos_x * sin_z,
        // };
        // let zaxis = v3 {
        //     x: -sin_y,
        //     y: cos_y * sin_z,
        //     z: cos_y * cos_z,
        // };

        // let xaxis = v3 {
        //     x: cos_y,
        //     y: 0.0,
        //     z: -sin_y,
        // };
        // let yaxis = v3 {
        //     x: sin_y * sin_x,
        //     y: cos_x,
        //     z: cos_y * sin_x,
        // };
        // let zaxis = v3 {
        //     x: sin_y * cos_x,
        //     y: -sin_x,
        //     z: cos_x * cos_y,
        // };

        // let m03 = v3::dot(xaxis, self.position);
        // let m13 = v3::dot(yaxis, self.position);
        // let m23 = v3::dot(zaxis, self.position);

        // self.m = Some(m4([
        //     [xaxis.x, xaxis.y, xaxis.z, m03],
        //     [yaxis.x, yaxis.y, yaxis.z, m13],
        //     [zaxis.x, zaxis.y, zaxis.z, m23],
        //     [0.0, 0.0, 0.0, 1.0],
        // ]));

        // println!("{:?}", self.rotation);

        // // could probably do this cleaner with the roll value and extract this behavior into an
        // // FPS camera component
        // let mut front = v3 {
        //     x: cos_y * cos_x,
        //     y: sin_x,
        //     z: sin_y * cos_x,
        // };
        // front.normalize();

        // let mut right = v3::cross(
        //     front,
        //     v3 {
        //         x: 0.0,
        //         y: 1.0,
        //         z: 0.0,
        //     },
        // );
        // right.normalize();

        // let mut up = v3::cross(right, front);
        // up.normalize();

        // self.look_at(self.position, self.position - front, up);
    }

    pub fn look_at(&mut self, eye: v3, target: v3, up: v3) {
        let mut f = target - eye;
        f.normalize();

        let mut u = up.normalized();

        let mut r = v3::cross(f, u);
        r.normalize();

        u = v3::cross(r, f);

        let m03 = -v3::dot(r, eye);
        let m13 = -v3::dot(u, eye);
        let m23 = v3::dot(f, eye);

        self.m = Some(m4([
            [r.x, r.y, r.z, m03],
            [u.x, u.y, u.z, m13],
            [-f.x, -f.y, -f.z, m23],
            [0.0, 0.0, 0.0, 1.0],
        ]));
    }
}

impl Default for ViewMatrix {
    fn default() -> Self {
        Self::new_position(0.0, 0.0, 0.0)
    }
}

impl Matrix for ViewMatrix {
    fn get_matrix(&self) -> &m4 {
        &self.m.as_ref().unwrap()
    }
}
