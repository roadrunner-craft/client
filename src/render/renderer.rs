use crate::components::Transform;
use crate::math::vector::v3;
use crate::render::camera::PerspectiveCamera;
use crate::render::models::Cube;
use crate::render::shaders::{FragmentShader, ShaderProgram, VertexShader};
use crate::utils::traits::{Bindable, Matrix};

use gl::types::GLint;
use std::ptr;

pub struct Renderer {
    pub program: ShaderProgram,
    camera: PerspectiveCamera,
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

            void main() {
                pass_uv = uv;
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

        // TODO: make sure to handle dpi and update
        Self {
            program: ShaderProgram::create_and_link(vertex, fragment).unwrap(),
            camera: PerspectiveCamera::new(70.0, 0.1, 1024.0, width as f32 / height as f32),
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        let aspect_ratio = width as f32 / height as f32;
        self.camera.set_aspect_ratio(aspect_ratio);
    }

    pub fn draw(&self, cube: &Cube) {
        //world: &World, camera: &Camera) {
        self.program.enable();
        self.program.set_uniform_m4(
            "transform",
            Transform::new(
                v3 {
                    x: 0.0,
                    y: 0.0,
                    z: 2.0,
                },
                v3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                v3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            )
            .get_matrix(),
        );
        self.program.set_uniform_m4("view", self.camera.get_view());
        self.program
            .set_uniform_m4("projection", self.camera.get_projection());

        //uniform! {
        //    texture: atlas.get_texture(),
        //    projection: self.projection.get_matrix(),
        //    view: self.camera.get_matrix(),
        //    transform: mesh.get_matrix(),
        //    uvs: mesh.get_uvs()
        //};

        unsafe {
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::DEPTH_TEST);

            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

            cube.bind();
            gl::DrawElements(
                gl::TRIANGLES,
                cube.get_indices_count() as GLint,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            cube.unbind();
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.program.delete();
    }
}
