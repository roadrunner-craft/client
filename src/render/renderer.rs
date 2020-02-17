use crate::render::models::Quad;
use crate::render::shaders::{FragmentShader, ShaderProgram, VertexShader};
use crate::utils::traits::Bindable;

use std::ptr;

pub struct Renderer {
    pub program: ShaderProgram,
}

impl Renderer {
    pub fn init() -> Self {
        let vertex_src: &'static str = r#"
            #version 410 core

            layout (location=0) in vec3 position;
            layout (location=1) in vec2 uv;

            out vec2 pass_uv;

            void main() {
                pass_uv = uv;
                gl_Position = vec4(position, 1.0);
            }
        "#;
        let vertex = VertexShader::compile(vertex_src).unwrap();

        let fragment_src: &'static str = r#"
            #version 410 core

            in vec2 pass_uv;

            out vec4 color;

            uniform sampler2D textureSampler;

            void main() {
                color = texture(textureSampler, pass_uv);
            }
        "#;
        let fragment = FragmentShader::compile(fragment_src).unwrap();

        Self {
            program: ShaderProgram::create_and_link(vertex, fragment).unwrap(),
        }
    }

    pub fn draw(&self, quad: &Quad) {
        self.program.enable();

        unsafe {
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            quad.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            quad.unbind();
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.program.delete();
    }
}
