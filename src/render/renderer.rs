use crate::components::Transform;
use crate::input::InputHandler;
use crate::math::vector::v3;
use crate::render::camera::Camera;
use crate::render::models::Cube;
use crate::render::shaders::{FragmentShader, ShaderProgram, VertexShader};
use crate::render::{Display, RenderSettings};
use crate::utils::traits::{Bindable, Matrix};

use gl::types::GLint;
use scancode::Scancode;
use std::ptr;

pub struct Renderer {
    pub program: ShaderProgram,
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

        // TODO: remove this temporary data
        crate::render::Texture::new();

        // TODO: make sure to handle dpi and update
        Self {
            program: ShaderProgram::create_and_link(vertex, fragment).unwrap(),
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

    pub fn draw<C: Camera>(&self, display: &Display, camera: &C, cube: &Cube) {
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
        self.program.set_uniform_m4("view", camera.get_view());
        self.program
            .set_uniform_m4("projection", camera.get_projection());

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

            cube.bind();
            gl::DrawElements(
                gl::TRIANGLES,
                cube.get_indices_count() as GLint,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            cube.unbind();
        }

        display.context.swap_buffers().unwrap();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.program.delete();
    }
}
