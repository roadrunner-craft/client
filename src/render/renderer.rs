use crate::components::Transform;
use crate::math::vector::v3;
use crate::render::camera::{Camera, Perspective, ProjectionMatrix};
use crate::render::models::Cube;
use crate::render::shaders::{FragmentShader, ShaderProgram, VertexShader};
use crate::utils::traits::Bindable;

use gl::types::GLint;
use std::ptr;

pub struct Renderer {
    pub program: ShaderProgram,
    //camera: Camera,
    projection: Perspective,
}

impl Renderer {
    pub fn init(width: u32, height: u32) -> Self {
        let vertex_src: &'static str = r#"
            #version 410 core

            layout (location=0) in vec3 position;
            layout (location=1) in vec2 uv;
            out vec2 pass_uv;

            uniform mat4 transform;
            uniform mat4 camera;
            uniform mat4 projection;

            void main() {
                pass_uv = uv;
                gl_Position = projection * transform * vec4(position, 1.0);
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

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        // TODO: make sure to handle dpi and update
        Self {
            program: ShaderProgram::create_and_link(vertex, fragment).unwrap(),
            //camera: Camera::new(Box::new(projection)),
            projection: Perspective::new(90.0, 0.1, 1024.0, width as f32 / height as f32),
        }
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.projection
            .set_aspect_ratio(width as f32 / height as f32);
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
                    z: -1.0,
                },
                v3 {
                    x: 0.0,
                    y: 45.0,
                    z: 45.0,
                },
                v3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            )
            .m,
        );
        self.program.set_uniform_m4(
            "camera",
            Transform::new(
                v3 {
                    x: 0.0,
                    y: 2.0,
                    z: 0.0,
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
            .m,
        );
        self.program
            .set_uniform_m4("projection", self.projection.get_matrix().clone());

        unsafe {
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::DEPTH_TEST);

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
