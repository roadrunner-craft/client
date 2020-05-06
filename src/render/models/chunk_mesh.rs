use crate::game::block::BlockDatabase;
use crate::game::chunk::ChunkGroup;
use crate::game::chunk::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::math::vector::v3;
use crate::render::models::Model;
use crate::utils::traits::Bindable;

use gl::types::GLuint;

struct Face {
    vertices: [v3; 4],
    light: u8,
}

const FRONT_FACE: Face = Face {
    vertices: [
        v3 {
            x: -0.5,
            y: 1.0,
            z: -0.5,
        },
        v3 {
            x: 0.5,
            y: 1.0,
            z: -0.5,
        },
        v3 {
            x: 0.5,
            y: 0.0,
            z: -0.5,
        },
        v3 {
            x: -0.5,
            y: 0.0,
            z: -0.5,
        },
    ],
    light: 2,
};

const BACK_FACE: Face = Face {
    vertices: [
        v3 {
            x: 0.5,
            y: 1.0,
            z: 0.5,
        },
        v3 {
            x: -0.5,
            y: 1.0,
            z: 0.5,
        },
        v3 {
            x: -0.5,
            y: 0.0,
            z: 0.5,
        },
        v3 {
            x: 0.5,
            y: 0.0,
            z: 0.5,
        },
    ],
    light: 2,
};

const RIGHT_FACE: Face = Face {
    vertices: [
        v3 {
            x: 0.5,
            y: 1.0,
            z: -0.5,
        },
        v3 {
            x: 0.5,
            y: 1.0,
            z: 0.5,
        },
        v3 {
            x: 0.5,
            y: 0.0,
            z: 0.5,
        },
        v3 {
            x: 0.5,
            y: 0.0,
            z: -0.5,
        },
    ],
    light: 1,
};

const LEFT_FACE: Face = Face {
    vertices: [
        v3 {
            x: -0.5,
            y: 1.0,
            z: 0.5,
        },
        v3 {
            x: -0.5,
            y: 1.0,
            z: -0.5,
        },
        v3 {
            x: -0.5,
            y: 0.0,
            z: -0.5,
        },
        v3 {
            x: -0.5,
            y: 0.0,
            z: 0.5,
        },
    ],
    light: 1,
};

const TOP_FACE: Face = Face {
    vertices: [
        v3 {
            x: -0.5,
            y: 1.0,
            z: 0.5,
        },
        v3 {
            x: 0.5,
            y: 1.0,
            z: 0.5,
        },
        v3 {
            x: 0.5,
            y: 1.0,
            z: -0.5,
        },
        v3 {
            x: -0.5,
            y: 1.0,
            z: -0.5,
        },
    ],
    light: 3,
};

const BOTTOM_FACE: Face = Face {
    vertices: [
        v3 {
            x: -0.5,
            y: 0.0,
            z: -0.5,
        },
        v3 {
            x: 0.5,
            y: 0.0,
            z: -0.5,
        },
        v3 {
            x: 0.5,
            y: 0.0,
            z: 0.5,
        },
        v3 {
            x: -0.5,
            y: 0.0,
            z: 0.5,
        },
    ],
    light: 0,
};

#[derive(Default)]
pub struct ChunkMesh {
    model: Option<Model>,
    vertices: Vec<v3>,
    vertices_info: Vec<GLuint>,
    vertex_count: GLuint,
    indices: Vec<GLuint>,
}

impl ChunkMesh {
    fn add_face(&mut self, face: Face, position: v3, texture_id: u8) {
        for i in 0..4 {
            self.vertices.push(face.vertices[i] + position);

            let info: GLuint = (texture_id as GLuint) << 4
                | ((face.light & 0b11) << 2) as GLuint
                | (i & 0b11) as GLuint;
            self.vertices_info.push(info);
        }

        self.indices.push(self.vertex_count);
        self.indices.push(self.vertex_count + 3);
        self.indices.push(self.vertex_count + 1);
        self.indices.push(self.vertex_count + 1);
        self.indices.push(self.vertex_count + 3);
        self.indices.push(self.vertex_count + 2);

        self.vertex_count += 4;
    }

    pub fn index_count(&self) -> usize {
        if let Some(ref model) = self.model {
            return model.index_count();
        } else {
            return 0;
        }
    }

    pub fn generate(chunks: &ChunkGroup, block_database: &BlockDatabase) -> ChunkMesh {
        let mut mesh = ChunkMesh::default();

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    let x = x as i8;
                    let y = y as u8;
                    let z = z as i8;

                    let block = chunks.get_block(x, y, z);

                    if block.id == 0 {
                        continue;
                    }

                    if let Some(properties) = block_database.get(block.id) {
                        let position = v3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        };

                        if !block_database.is_opaque(chunks.get_block(x, y, z - 1).id) {
                            mesh.add_face(FRONT_FACE, position, properties.texture.front);
                        }

                        if !block_database.is_opaque(chunks.get_block(x, y, z + 1).id) {
                            mesh.add_face(BACK_FACE, position, properties.texture.back);
                        }

                        if !block_database.is_opaque(chunks.get_block(x - 1, y, z).id) {
                            mesh.add_face(LEFT_FACE, position, properties.texture.left);
                        }

                        if !block_database.is_opaque(chunks.get_block(x + 1, y, z).id) {
                            mesh.add_face(RIGHT_FACE, position, properties.texture.right);
                        }

                        if y as usize == (CHUNK_HEIGHT - 1) {
                            mesh.add_face(TOP_FACE, position, properties.texture.top);
                        } else if y == 0 {
                            mesh.add_face(BOTTOM_FACE, position, properties.texture.bottom);
                        } else {
                            if !block_database.is_opaque(chunks.get_block(x, y + 1, z).id) {
                                mesh.add_face(TOP_FACE, position, properties.texture.top);
                            }

                            if !block_database.is_opaque(chunks.get_block(x, y - 1, z).id) {
                                mesh.add_face(BOTTOM_FACE, position, properties.texture.bottom);
                            }
                        }
                    }
                }
            }
        }

        if !mesh.vertices.is_empty() {
            mesh.model = Some(Model::new(&mesh.vertices, &mesh.indices));
            mesh.model.as_mut().unwrap().add_vbo(1, &mesh.vertices_info);
        }
        mesh
    }
}

impl Bindable for ChunkMesh {
    fn bind(&self) {
        if let Some(ref model) = self.model {
            model.bind();
        }
    }

    fn unbind(&self) {
        if let Some(ref model) = self.model {
            model.unbind();
        }
    }
}
