use math::matrix::Matrix4;
use math::vector::Vector3;

// TODO: merge this class with the transformation component
pub struct ViewMatrix {
    position: Vector3,
    rotation: Vector3,
    m: Option<Matrix4>,
}

impl ViewMatrix {
    pub fn new_position(x: f32, y: f32, z: f32) -> Self {
        let mut view = Self {
            position: Vector3 { x, y, z },
            rotation: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            m: None,
        };

        view.generate_matrix();
        view
    }

    pub fn get_position(&self) -> Vector3 {
        self.position
    }

    pub fn set_position(&mut self, value: Vector3) -> &mut Self {
        self.position = value;
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

        let mut m = Matrix4::identity();

        // TODO: reduce this into a single Matrix4 assignment
        m = m * Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cx, sx, 0.0],
            [0.0, -sx, cx, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        m = m * Matrix4([
            [cy, 0.0, -sy, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sy, 0.0, cy, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        m = m * Matrix4([
            [cz, sz, 0.0, 0.0],
            [-sz, cz, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        m = m * Matrix4([
            [1.0, 0.0, 0.0, -self.position.x],
            [0.0, 1.0, 0.0, -self.position.y],
            [0.0, 0.0, 1.0, -self.position.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        self.m = Some(m);
    }
}

impl Default for ViewMatrix {
    fn default() -> Self {
        Self::new_position(0.0, 0.0, 0.0)
    }
}
