use crate::components::Transform;
use crate::game::chunk::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::game::World;
use crate::input::InputHandler;
use crate::math::vector::v3;
use crate::render::camera::Camera;
use crate::render::models::Quad;
use crate::render::shaders::{FragmentShader, ShaderProgram, VertexShader};
use crate::render::Texture;
use crate::render::{Display, RenderSettings};
use crate::utils::atlas::AtlasGenerator;
use crate::utils::direction::Direction;
use crate::utils::traits::{Bindable, Matrix};

use gl::types::GLint;
use scancode::Scancode;
use std::collections::HashMap;
use std::path::Path;
use std::ptr;

pub struct Renderer {
    pub program: ShaderProgram,
    atlas: Texture,
    settings: RenderSettings,
}

impl Renderer {
    pub fn init(width: u32, height: u32) -> Self {
        let vertex_src: &'static str = r#"
            #version 410 core

            layout (location=0) in vec3 position;
            layout (location=1) in vec2 uv;
            out vec2 pass_uv;

            uniform mat4 transform;
            uniform mat4 view;
            uniform mat4 projection;
            uniform uint block_id;
            uniform float texture_size;

            vec2 atlas_uv(vec2 uv) {
                float tile_size = 16;
                float tile_per_row = texture_size / tile_size;
                float atlas_index = block_id - 1;

                vec2 new_uv;
                new_uv.x = mod(atlas_index, tile_per_row) / tile_per_row + uv.x * 16 / texture_size;
                new_uv.y = floor(atlas_index / tile_per_row) / tile_per_row + uv.y * 16 / texture_size;

                return new_uv;
            }

            void main() {
                pass_uv = atlas_uv(uv);
                gl_Position = projection * view * transform * vec4(position, 1.0);
            }
        "#;
        let vertex = VertexShader::compile(vertex_src).unwrap();

        let fragment_src: &'static str = r#"
            #version 410 core

            in vec2 pass_uv;
            out vec4 color;

            uniform sampler2D diffuseTexture;

            void main() {
                color = texture(diffuseTexture, pass_uv);
            }
        "#;
        let fragment = FragmentShader::compile(fragment_src).unwrap();

        // TODO: this probably should be in the game code and passed as a parameter
        let mut textures = HashMap::new();

        textures.insert(0, "res/textures/block/dirt.png");
        textures.insert(1, "res/textures/block/stone.png");

        let (img, img_size) = AtlasGenerator::generate(textures);

        // TODO: make sure to handle dpi and update
        Self {
            program: ShaderProgram::create_and_link(vertex, fragment).unwrap(),
            atlas: Texture::from_image(&img, img_size),
            settings: RenderSettings::default(),
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn update(&mut self, input: &InputHandler) {
        if input.is_key_pressed(Scancode::M) {
            self.settings.wireframe = !self.settings.wireframe;
        }
    }

    pub fn draw<C: Camera>(&self, display: &Display, camera: &C, world: &World) {
        //world: &World, camera: &Camera) {
        self.program.enable();
        self.program.set_uniform_m4("view", camera.get_view());
        self.program
            .set_uniform_m4("projection", camera.get_projection());
        self.program
            .set_uniform_float("texture_size", self.atlas.size as f32);

        //uniform! {
        //    texture: atlas.get_texture(),
        //    projection: camera.get_projection(),
        //    view: camera.get_view(),
        //    transform: mesh.get_matrix(),
        //    uvs: mesh.get_uvs()
        //};

        unsafe {
            gl::ClearColor(116.0 / 255.0, 173.0 / 255.0, 251.0 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::DEPTH_TEST);

            self.settings.apply();

            let face = Quad::new_face();
            face.bind();

            let blocks = &world.chunks[0][0].blocks;

            for x in 0..CHUNK_WIDTH {
                for y in 0..CHUNK_HEIGHT {
                    for z in 0..CHUNK_DEPTH {
                        let block = blocks[x][y][z];

                        if block.id == 0 {
                            continue;
                        }

                        self.program.set_uniform_u32("block_id", block.id as u32);

                        if z == 0 || blocks[x][y][z - 1].id == 0 {
                            self.draw_face(x, y, z, Direction::FRONT);
                        }

                        if z == CHUNK_DEPTH - 1 || blocks[x][y][z + 1].id == 0 {
                            self.draw_face(x, y, z, Direction::BACK);
                        }

                        if x == 0 || blocks[x - 1][y][z].id == 0 {
                            self.draw_face(x, y, z, Direction::LEFT);
                        }

                        if x == CHUNK_WIDTH - 1 || blocks[x + 1][y][z].id == 0 {
                            self.draw_face(x, y, z, Direction::RIGHT);
                        }

                        if y == 0 || blocks[x][y - 1][z].id == 0 {
                            self.draw_face(x, y, z, Direction::BOTTOM);
                        }

                        if y == CHUNK_HEIGHT - 1 || blocks[x][y + 1][z].id == 0 {
                            self.draw_face(x, y, z, Direction::TOP);
                        }
                    }
                }
            }

            face.unbind();
        }

        display.context.swap_buffers().unwrap();
    }

    fn draw_face(&self, x: usize, y: usize, z: usize, direction: Direction) {
        let position = match direction {
            Direction::FRONT => v3 {
                x: x as f32 - 0.5,
                y: y as f32,
                z: z as f32 - 0.5,
            },
            Direction::BACK => v3 {
                x: x as f32 + 0.5,
                y: y as f32,
                z: z as f32 + 0.5,
            },
            Direction::LEFT => v3 {
                x: x as f32 - 0.5,
                y: y as f32,
                z: z as f32 + 0.5,
            },
            Direction::RIGHT => v3 {
                x: x as f32 + 0.5,
                y: y as f32,
                z: z as f32 - 0.5,
            },
            Direction::TOP => v3 {
                x: x as f32 - 0.5,
                y: y as f32 + 1.0,
                z: z as f32 - 0.5,
            },
            Direction::BOTTOM => v3 {
                x: x as f32 - 0.5,
                y: y as f32,
                z: z as f32 + 0.5,
            },
        };

        let rotation = match direction {
            Direction::FRONT => v3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Direction::BACK => v3 {
                x: 0.0,
                y: 180.0,
                z: 0.0,
            },
            Direction::LEFT => v3 {
                x: 0.0,
                y: 90.0,
                z: 0.0,
            },
            Direction::RIGHT => v3 {
                x: 0.0,
                y: -90.0,
                z: 0.0,
            },
            Direction::TOP => v3 {
                x: 90.0,
                y: 0.0,
                z: 0.0,
            },
            Direction::BOTTOM => v3 {
                x: -90.0,
                y: 0.0,
                z: 0.0,
            },
        };

        self.program.set_uniform_m4(
            "transform",
            Transform::new(position, rotation, v3::identity()).get_matrix(),
        );

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6 as GLint, gl::UNSIGNED_INT, ptr::null());
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.program.delete();
    }
}
