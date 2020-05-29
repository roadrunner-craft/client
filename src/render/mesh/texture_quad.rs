use crate::ops::{Bindable, Drawable};
use crate::render::mesh::Mesh;

use math::vector::Vector3;

pub struct TextureQuad {
    mesh: Mesh,
}

impl TextureQuad {
    pub fn new() -> Self {
        let vertices = vec![
            Vector3 {
                x: -1.0,
                y: 1.0,
                z: 0.0,
            },
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            Vector3 {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            },
            Vector3 {
                x: -1.0,
                y: -1.0,
                z: 0.0,
            },
        ];

        let indices = vec![0, 3, 1, 1, 3, 2];

        Self {
            mesh: Mesh::new(&vertices, &indices),
        }
    }
}

impl Drawable for TextureQuad {
    fn draw(&self) {
        self.mesh.draw();
    }
}

impl Bindable for TextureQuad {
    fn bind(&self) {
        self.mesh.bind();
    }

    fn unbind(&self) {
        self.mesh.unbind();
    }
}
