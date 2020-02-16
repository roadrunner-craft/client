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

            in vec3 position;
            out vec3 color;

            void main() {
                gl_Position = vec4(position, 1.0);
                color = vec3(position.x+0.5, 1, position.y+0.5);
            }
        "#;
        let vertex = VertexShader::compile(vertex_src).unwrap();

        let fragment_src: &'static str = r#"
            #version 410 core

            in vec3 color;
            out vec4 out_color;

            void main() {
                out_color = vec4(color, 1.0);
               // color = vec4(1.0, 0.0, 0.0, 1.0);
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
