use crate::math::vector::{v2, v3};
use crate::render::models::Model;
use crate::utils::traits::Bindable;

pub struct Cube {
    model: Model,
}

impl Cube {
    pub fn new() -> Cube {
        let f1 = v3 {
            x: -0.5,
            y: 0.5,
            z: -0.5,
        };
        let f2 = v3 {
            x: 0.5,
            y: 0.5,
            z: -0.5,
        };
        let f3 = v3 {
            x: 0.5,
            y: -0.5,
            z: -0.5,
        };
        let f4 = v3 {
            x: -0.5,
            y: -0.5,
            z: -0.5,
        };

        let b1 = v3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        };
        let b2 = v3 {
            x: -0.5,
            y: 0.5,
            z: 0.5,
        };
        let b3 = v3 {
            x: -0.5,
            y: -0.5,
            z: 0.5,
        };
        let b4 = v3 {
            x: 0.5,
            y: -0.5,
            z: 0.5,
        };

        Cube {
            model: Model::new_textured(
                vec![
                    f1, f2, f3, f4, // front
                    b1, b2, b3, b4, // back
                    f2, b1, b4, f3, // right
                    b2, f1, f4, b3, // left
                    b2, b1, f2, f1, // top
                    f4, f3, b4, b3, // bottom
                ],
                vec![
                    v2 { x: 0.0, y: 0.0 },
                    v2 { x: 1.0, y: 0.0 },
                    v2 { x: 1.0, y: 1.0 },
                    v2 { x: 0.0, y: 1.0 },
                    v2 { x: 0.0, y: 0.0 },
                    v2 { x: 1.0, y: 0.0 },
                    v2 { x: 1.0, y: 1.0 },
                    v2 { x: 0.0, y: 1.0 },
                    v2 { x: 0.0, y: 0.0 },
                    v2 { x: 1.0, y: 0.0 },
                    v2 { x: 1.0, y: 1.0 },
                    v2 { x: 0.0, y: 1.0 },
                    v2 { x: 0.0, y: 0.0 },
                    v2 { x: 1.0, y: 0.0 },
                    v2 { x: 1.0, y: 1.0 },
                    v2 { x: 0.0, y: 1.0 },
                    v2 { x: 0.0, y: 0.0 },
                    v2 { x: 1.0, y: 0.0 },
                    v2 { x: 1.0, y: 1.0 },
                    v2 { x: 0.0, y: 1.0 },
                    v2 { x: 0.0, y: 0.0 },
                    v2 { x: 1.0, y: 0.0 },
                    v2 { x: 1.0, y: 1.0 },
                    v2 { x: 0.0, y: 1.0 },
                ],
                vec![
                    0, 1, 2, 2, 3, 0, // front
                    4, 5, 6, 6, 7, 4, // back
                    8, 9, 10, 10, 11, 8, // right
                    12, 13, 14, 14, 15, 12, // left
                    16, 17, 18, 18, 19, 16, // top
                    20, 21, 22, 22, 23, 20, // bottom
                ],
            ),
        }
    }

    pub fn get_indices_count(&self) -> usize {
        self.model.get_indices_count()
    }
}

impl Bindable for Cube {
    fn bind(&self) {
        self.model.bind();
    }

    fn unbind(&self) {
        self.model.unbind();
    }
}
