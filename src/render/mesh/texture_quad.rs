use crate::ops::{Bindable, Drawable};
use crate::render::mesh::Mesh;

use math::utils::next_power_of_two;
use math::vector::{Vector2, Vector3};

pub struct TextureQuad {
    mesh: Mesh,
}

impl TextureQuad {
    pub fn new() -> Self {
        let vertices = vec![
            Vector3 {
                x: -1.0,
                y: -1.0,
                z: 0.0,
            },
            Vector3 {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            },
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            Vector3 {
                x: -1.0,
                y: 1.0,
                z: 0.0,
            },
        ];

        let indices = vec![0, 1, 3, 1, 2, 3];

        let uvs = vec![
            Vector2 { x: 0.0, y: 0.0 },
            Vector2 { x: 1.0, y: 0.0 },
            Vector2 { x: 1.0, y: 1.0 },
            Vector2 { x: 0.0, y: 1.0 },
        ];

        let mut mesh = Mesh::new(&vertices, &indices);
        mesh.add_vbo(&uvs);

        Self { mesh }
    }

    pub fn new_rect(x: f32, y: f32, width: f32, height: f32) -> Self {
        let vertices = vec![
            Vector3 { x, y, z: 0.0 },
            Vector3 {
                x: x + width,
                y,
                z: 0.0,
            },
            Vector3 {
                x: x + width,
                y: y + height,
                z: 0.0,
            },
            Vector3 {
                x,
                y: y + height,
                z: 0.0,
            },
        ];

        let indices = vec![0, 3, 1, 1, 3, 2];

        let texture_width = next_power_of_two(width as u32) as f32;
        let texture_height = next_power_of_two(height as u32) as f32;

        let u = width / texture_width;
        let v = height / texture_height;

        let uvs = vec![
            Vector2 { x: 0.0, y: 0.0 },
            Vector2 { x: u, y: 0.0 },
            Vector2 { x: u, y: v },
            Vector2 { x: 0.0, y: v },
        ];

        let mut mesh = Mesh::new(&vertices, &indices);
        mesh.add_vbo(&uvs);

        Self { mesh }
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
