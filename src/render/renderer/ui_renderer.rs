use crate::ops::{Bindable, Drawable};
use crate::render::{
    camera::OrthographicProjection, mesh::TextureQuad, shaders::ShaderProgram, ui::Font,
};

use math::vector::Vector3;
use std::path::Path;

pub struct UIRenderer {
    program: ShaderProgram,
    projection: OrthographicProjection,
    font: Font,
}

impl UIRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        let vertex_src: &'static str = r#"
            #version 410 core

            layout (location=0) in vec3 position;
            layout (location=1) in vec2 uv_data;

            out vec2 uv;

            uniform mat4 projection;

            void main() {
                uv = uv_data;
                gl_Position = projection * vec4(position, 1.0);
            }
        "#;

        let fragment_src: &'static str = r#"
            #version 410 core

            in vec2 uv;

            out vec4 color;

            uniform sampler2D diffuse_texture;
            uniform vec3 background_color;
            uniform vec3 tint_color;
            uniform bool render_texture;

            void main() {
                if (!render_texture) {
                    color = vec4(background_color, 1.0);
                } else {
                    color = texture(diffuse_texture, uv);
                    color = vec4(color.r * tint_color.r, 
                                 color.r * tint_color.g, 
                                 color.r * tint_color.b, 
                                 color.r * 1.0);
                }

                if (color.a < 0.01) {
                    discard;
                }
            }
        "#;

        match ShaderProgram::new(vertex_src, fragment_src) {
            Ok(program) => Self {
                program,
                font: Font::new(Path::new("res/fonts/font.ttf"), 128.0).unwrap(),
                projection: OrthographicProjection::new(
                    0.0,
                    width as f32,
                    height as f32,
                    0.0,
                    -1.0,
                    1.0,
                ),
            },
            Err(err) => {
                error!("could not compile shader program {}:{}", file!(), line!());
                panic!("\n{}\n", err);
            }
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.projection.resize(width, height);
    }

    pub fn draw(&self) {
        self.program.use_program();
        self.program
            .set_uniform_m4("projection", self.projection.matrix());

        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
        }

        for (rect, texture) in self
            .font
            .iter_for(String::from("abcdefghijklmnopqrstuvwxyz"), 0.0)
        {
            let quad =
                TextureQuad::new_rect(rect.x + 300.0, rect.y + 300.0, rect.width, rect.height);

            self.program.set_uniform_bool("render_texture", true);
            self.program
                .set_uniform_texture("diffuse_texture", texture.unit());

            self.program.set_uniform_v3(
                "tint_color",
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            );

            texture.bind();
            quad.bind();
            quad.draw();
        }

        unsafe {
            gl::Disable(gl::BLEND);
        }
    }
}
