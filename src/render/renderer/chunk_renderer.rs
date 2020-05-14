use crate::game::TextureDatabase;
use crate::ops::{Bindable, Drawable};
use crate::render::camera::Camera;
use crate::render::display::FrameBuffer;
use crate::render::mesh::chunk_mesh::ChunkMesh;
use crate::render::post::{
    IdentityPostProcessing, PostProcessingEffectType, PostProcessingPipeline,
};
use crate::render::shaders::ShaderProgram;
use crate::render::texture::TextureArray;

use core::block::BlockRegistry;
use core::chunk::{ChunkGridCoordinate, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use core::world::{World, LOAD_DISTANCE};
use math::container::{Volume, AABB};
use math::vector::{Vector2, Vector3};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const TEXTURE_RESOLUTION: u32 = 16;
const FOG: Vector3 = Vector3 {
    x: 0.62,
    y: 0.76,
    z: 1.0,
};

pub struct ChunkRenderer {
    program: ShaderProgram,
    textures: TextureArray,
    meshes: HashMap<ChunkGridCoordinate, ChunkMesh>,
    block_registry: BlockRegistry,
    post: PostProcessingPipeline,
    buffer: FrameBuffer,
}

impl ChunkRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        let vertex_src: &'static str = r#"
            #version 410 core

            layout (location=0) in vec3 position;
            layout (location=1) in uint info;

            out vec3 world_position;
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
            uniform mat4 projection_view; // projection * view

            void main() {
                uint uv_index = info & 3u;

                uv = uvs[uv_index];
                light = float((info >> 2u & 3u) + 2) / 5.0 ;
                texture_id = info >> 4u;

                world_position = vec3(chunk_position.x, 0, chunk_position.y) + position;
                
                gl_Position = projection_view * vec4(world_position, 1.0);
            }
        "#;

        let fragment_src: &'static str = r#"
            #version 410 core

            in vec3 world_position;
            in vec2 uv;
            in float light;
            flat in uint texture_id;

            out vec4 color;
            //layout(location = 0) out vec4 color;

            uniform sampler2DArray diffuse_textures;
            uniform vec3 camera_position;
            uniform vec3 fog_color;
            uniform uint render_distance;
            
            vec4 get_color(uint id) {
                return light * texture(diffuse_textures, vec3(uv, id));
            }

            vec4 apply_fog(vec4 diffuse) {
                float fog_max = max(32.0, float((render_distance - 2) * 16));
                float fog_min = max(16.0, fog_max - 64.0);

                float distance = length(camera_position - world_position);
                float fog = (distance - fog_min) / (fog_max - fog_min);
                fog = max(0.0, min(fog, 1.0));

                return (1.0 - fog) * diffuse + fog * vec4(fog_color, diffuse.a);
            }

            void main() {
                if (texture_id == 2 || texture_id == 10 || texture_id == 4) {
                    vec4 cheapColorMapOutput = vec4(0.492, 0.762, 0.348, 1.0); // jungle
                    //vec4 cheapColorMapOutput = vec4(0.73, 0.71, 0.395, 1.0); // desert

                    if (texture_id == 4) {
                        color = get_color(10) * cheapColorMapOutput;

                        if (color.w == 0.0) {
                            color = get_color(texture_id - 1);
                        }
                    } else {
                        color = get_color(texture_id - 1) * cheapColorMapOutput;
                    }
                    
                    color = apply_fog(color);

                    return;
                }

                color = apply_fog(get_color(texture_id - 1));

                if (color.a < 0.01) {
                    discard;
                }
            }
        "#;

        let database = TextureDatabase::new();
        let textures = TextureArray::new(TEXTURE_RESOLUTION, database.len() as u32, 2);

        for (i, file) in database.iter() {
            let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("res/textures/block/{}.png", file));
            textures.add_file(&path, (*i as u32) - 1);
        }

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("res/data/blocks.json");
        let path = path.to_str().unwrap();

        let data =
            fs::read_to_string(path).expect("<block_database> Could not read data from file");
        let block_registry = BlockRegistry::new(serde_json::from_str(&data).unwrap());

        let mut pipeline = PostProcessingPipeline::new(width, height);
        pipeline.add(PostProcessingEffectType::Identity);

        match ShaderProgram::new(vertex_src, fragment_src) {
            Ok(program) => Self {
                program,
                textures,
                meshes: HashMap::new(),
                block_registry,
                post: pipeline,
                buffer: FrameBuffer::new(width, height, 1, true),
            },
            Err(err) => {
                panic!(
                    "<chunk-renderer> could not compile the shader program:\n\n{}\n",
                    err
                );
            }
        }
    }

    pub fn update(&mut self, world: &World) {
        // remove unloaded chunks' meshes
        self.meshes
            .retain(|coords, _| world.chunks.contains_key(coords));

        // add new loaded chunk's meshes
        for coords in world.chunks.keys() {
            if !self.meshes.contains_key(&coords) {
                let chunk_group = world.get_chunk_group(*coords);
                self.meshes.insert(
                    *coords,
                    ChunkMesh::generate(&chunk_group, &self.block_registry),
                );
            }
        }
    }

    pub fn draw<C: Camera>(&mut self, camera: &C) {
        self.program.use_program();
        self.program
            .set_uniform_m4("projection_view", camera.projection_view());
        self.program
            .set_uniform_texture("diffuse_textures", self.textures.unit());
        self.program
            .set_uniform_v3("camera_position", camera.position());
        self.program.set_uniform_v3("fog_color", FOG);
        self.program
            .set_uniform_u32("render_distance", LOAD_DISTANCE as u32);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);

            self.textures.bind();
            self.buffer.bind();
            self.buffer.clear(true, true, false);

            for (coords, mesh) in self.meshes.iter() {
                let position = Vector2 {
                    x: CHUNK_WIDTH as f32 * coords.x as f32,
                    y: CHUNK_DEPTH as f32 * coords.z as f32,
                };

                let chunk_volume = AABB::new(Volume::new(
                    position.x as i64,
                    0,
                    position.y as i64,
                    CHUNK_WIDTH as i64,
                    CHUNK_HEIGHT as i64,
                    CHUNK_DEPTH as i64,
                ));

                if !camera.frustum().contains(&chunk_volume) {
                    continue;
                }

                self.program.set_uniform_v2("chunk_position", position);

                mesh.draw();
            }

            self.post.apply(&self.buffer);
        }
    }
}
