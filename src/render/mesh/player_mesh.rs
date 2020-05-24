use crate::ops::{Bindable, Drawable};
use crate::render::mesh::Mesh;

use math::vector::Vector3;

pub struct PlayerMesh {
    mesh: Mesh,
}

impl PlayerMesh {
    pub fn new() -> Self {
        let vertices = vec![
            Vector3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            Vector3 {
                x: 1.0,
                y: 2.0,
                z: 0.0,
            },
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 1.0,
                y: 2.0,
                z: 1.0,
            },
            Vector3 {
                x: 0.0,
                y: 2.0,
                z: 1.0,
            },
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        ];

        let indices = vec![
            0, 3, 1, 3, 2, 1, // front
            5, 0, 4, 0, 1, 4, // top
            3, 6, 2, 6, 7, 2, // bottom
            1, 2, 4, 2, 7, 4, // right
            0, 5, 3, 5, 6, 3, // left
            4, 7, 5, 7, 6, 5, // back
        ];

        Self {
            mesh: Mesh::new(&vertices, &indices),
        }
    }
}

impl Drawable for PlayerMesh {
    fn draw(&self) {
        self.mesh.draw();
    }
}

impl Bindable for PlayerMesh {
    fn bind(&self) {
        self.mesh.bind();
    }

    fn unbind(&self) {
        self.mesh.unbind();
    }
}
