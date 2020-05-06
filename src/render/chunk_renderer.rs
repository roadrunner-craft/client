use crate::game::chunk::{CHUNK_DEPTH, CHUNK_WIDTH};
use crate::game::texture::TextureDatabase;
use crate::game::Game;
use crate::input::InputHandler;
use crate::math::vector::v2;
use crate::render::camera::Camera;
use crate::render::models::chunk_mesh::ChunkMesh;
use crate::render::shaders::{FragmentShader, ShaderProgram, VertexShader};
use crate::render::Texture;
use crate::utils::atlas::AtlasGenerator;
use crate::utils::traits::Bindable;

use gl::types::GLint;
use std::ptr;

struct ChunkRenderable {
    position: v2,
    mesh: ChunkMesh,
}

impl ChunkRenderable {
    fn get_mesh(&self) -> &ChunkMesh {
        &self.mesh
    }

    fn get_position(&self) -> v2 {
        self.position
    }
}

pub struct ChunkRenderer {
    program: ShaderProgram,
    atlas: Texture,
    renderables: Vec<ChunkRenderable>,
    temp: bool,
}

impl ChunkRenderer {
    pub fn new() -> Self {
        let vertex_src: &'static str = r#"
            #version 410 core

            layout (location=0) in vec3 position;
            layout (location=1) in uint info;
            out vec2 uv;
            out float light;

            vec2 uvs[4] = vec2[4](
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f),
                vec2(1.0f, 1.0f),
                vec2(0.0f, 1.0f)
            );

            uniform vec2 chunk_position;
            uniform mat4 view;
            uniform mat4 projection;
            uniform float texture_size;

            vec2 atlas_uv(vec2 uv, uint texture_id) {
                float tile_size = 16;
                float tile_per_row = texture_size / tile_size;
                float atlas_index = texture_id - 1;

                vec2 new_uv;
                new_uv.x = mod(atlas_index, tile_per_row) / tile_per_row + uv.x * 16 / texture_size;
                new_uv.y = floor(atlas_index / tile_per_row) / tile_per_row + uv.y * 16 / texture_size;

                return new_uv;
            }

            void main() {
                uint uv_index = info & 3u;
                uint texture_id = info >> 4u;

                uv = atlas_uv(uvs[uv_index], texture_id);
                light = ((info >> 2u & 3u) + 2) / 5.0 ;

                vec2 position_abs = chunk_position + position.xz;
                gl_Position = projection * view * vec4(position_abs.x, position.y, position_abs.y, 1.0);
            }
        "#;
        let vertex = VertexShader::compile(vertex_src).unwrap();

        let fragment_src: &'static str = r#"
            #version 410 core

            in vec2 uv;
            in float light;
            out vec4 color;

            uniform sampler2D diffuseTexture;

            void main() {
                color = light * texture(diffuseTexture, uv);
            }
        "#;
        let fragment = FragmentShader::compile(fragment_src).unwrap();

        let (img, img_size) = AtlasGenerator::generate(TextureDatabase::new());

        Self {
            program: ShaderProgram::create_and_link(vertex, fragment).unwrap(),
            atlas: Texture::from_image(&img, img_size),
            renderables: Vec::new(),
            temp: false,
        }
    }

    pub fn update(&mut self, input: &InputHandler) {}

    pub fn draw<C: Camera>(&mut self, camera: &C, game: &Game) {
        self.program.enable();
        self.program.set_uniform_m4("view", camera.get_view());
        self.program
            .set_uniform_m4("projection", camera.get_projection());
        self.program
            .set_uniform_float("texture_size", self.atlas.size as f32);

        if !self.temp {
            self.temp = true;

            let chunks = game.world.get_chunk_group(0, 0);
            self.renderables.push(ChunkRenderable {
                mesh: ChunkMesh::generate(&chunks, &game.block_database),
                position: v2 { x: 0.0, y: 0.0 },
            });
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            for renderable in self.renderables.iter_mut() {
                renderable.get_mesh().bind();

                let position = v2 {
                    x: CHUNK_WIDTH as f32 * renderable.get_position().x,
                    y: CHUNK_DEPTH as f32 * renderable.get_position().y,
                };
                self.program.set_uniform_v2("chunk_position", position);

                gl::DrawElements(
                    gl::TRIANGLES,
                    renderable.get_mesh().index_count() as GLint,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );

                renderable.mesh.unbind();
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
