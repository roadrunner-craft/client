use crate::game::chunk::ChunkGridCoordinate;
use crate::game::chunk::{CHUNK_DEPTH, CHUNK_WIDTH};
use crate::game::texture::TextureDatabase;
use crate::game::Game;
use crate::math::vector::v2;
use crate::render::camera::Camera;
use crate::render::models::chunk_mesh::ChunkMesh;
use crate::render::shaders::{FragmentShader, ShaderProgram, VertexShader};
use crate::render::texture::TextureArray;
use crate::utils::Bindable;

use gl::types::GLint;
use std::collections::HashMap;
use std::path::Path;
use std::ptr;

const TEXTURE_RESOLUTION: u32 = 16;

pub struct ChunkRenderer {
    program: ShaderProgram,
    textures: TextureArray,
    meshes: HashMap<ChunkGridCoordinate, ChunkMesh>,
}

impl ChunkRenderer {
    pub fn new() -> Self {
        let vertex_src: &'static str = r#"
            #version 410 core

            layout (location=0) in vec3 position;
            layout (location=1) in uint info;
            out vec2 uv;
            out float light;
            flat out uint texture_id;

            vec2 uvs[4] = vec2[4](
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f),
                vec2(1.0f, 1.0f),
                vec2(0.0f, 1.0f)
            );

            uniform vec2 chunk_position;
            uniform mat4 view;
            uniform mat4 projection;

            void main() {
                uint uv_index = info & 3u;

                uv = uvs[uv_index];
                light = float((info >> 2u & 3u) + 2) / 5.0 ;
                texture_id = info >> 4u;

                vec2 position_abs = chunk_position + position.xz;
                gl_Position = projection * view * vec4(position_abs.x, position.y, position_abs.y, 1.0);
            }
        "#;
        let vertex = VertexShader::compile(vertex_src).unwrap();

        let fragment_src: &'static str = r#"
            #version 410 core

            in vec2 uv;
            in float light;
            flat in uint texture_id;
            out vec4 color;

            uniform sampler2DArray diffuseTextures;

            void main() {
                if (texture_id == 2 || texture_id == 10) {
                    vec4 cheapColorMapOutput = vec4(0.492, 0.762, 0.348, 1.0); // jungle
                    //vec4 cheapColorMapOutput = vec4(0.73, 0.71, 0.395, 1.0); // desert

                    if (texture_id == 4) {
                        color = light * texture(diffuseTextures, vec3(uv, 11 - 1));
                        color *= cheapColorMapOutput;
                        if (color.w == 0.0) {
                            color = light * texture(diffuseTextures, vec3(uv, texture_id - 1));
                        }
                        return;
                    }

                    color = light * texture(diffuseTextures, vec3(uv, texture_id - 1));
                    color *= cheapColorMapOutput;

                    return;
                }
                color = light * texture(diffuseTextures, vec3(uv, texture_id - 1));
            }
        "#;
        let fragment = FragmentShader::compile(fragment_src).unwrap();

        let database = TextureDatabase::new();
        let textures = TextureArray::new(TEXTURE_RESOLUTION, database.len() as u32);

        for (i, file) in database.iter() {
            let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("res/textures/block/{}.png", file));
            textures.add_file(&path, (*i as u32) - 1);
        }

        Self {
            program: ShaderProgram::create_and_link(vertex, fragment).unwrap(),
            textures,
            meshes: HashMap::new(),
        }
    }

    pub fn update(&mut self, game: &Game) {
        self.meshes
            .retain(|coords, _| game.world.chunks.contains_key(coords));

        for coords in game.world.chunks.keys() {
            if !self.meshes.contains_key(&coords) {
                let chunks = game.world.get_chunk_group(coords.x, coords.z);
                self.meshes
                    .insert(*coords, ChunkMesh::generate(&chunks, &game.block_database));
            }
        }
    }

    pub fn draw<C: Camera>(&mut self, camera: &C) {
        self.program.enable();
        self.program.set_uniform_m4("view", camera.get_view());
        self.program
            .set_uniform_m4("projection", camera.get_projection());
        self.program
            .set_uniform_texture("diffuseTextures", self.textures.id());

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);

            self.textures.bind();

            for (coords, mesh) in self.meshes.iter_mut() {
                let position = v2 {
                    x: CHUNK_WIDTH as f32 * coords.x as f32,
                    y: CHUNK_DEPTH as f32 * coords.z as f32,
                };
                self.program.set_uniform_v2("chunk_position", position);

                mesh.bind();

                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh.index_count() as GLint,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );

                mesh.unbind();
            }
        }
    }
}

impl Default for ChunkRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ChunkRenderer {
    fn drop(&mut self) {
        self.program.delete();
    }
}
