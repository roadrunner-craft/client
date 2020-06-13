use crate::ops::{Bindable, Drawable};
use crate::render::{
    camera::OrthographicProjection,
    mesh::TextureQuad,
    shaders::ShaderProgram,
    ui::{Point, Rect, UIElement, UIView},
};
use crate::utils::Color;

use math::vector::Vector3;
use std::path::Path;
use std::sync::Mutex;

//lazy_static! {
//    pub static ref FONT_STORE: Mutex<FontStore> = Mutex::new(FontStore::default());
//}

pub struct UIRenderer {
    program: ShaderProgram,
    projection: OrthographicProjection,
    view: UIView,
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
            uniform vec4 background_color;
            uniform vec4 tint_color;
            uniform bool render_texture;

            void main() {
                if (!render_texture) {
                    color = background_color;
                } else {
                    color = texture(diffuse_texture, uv);
                    color = vec4(color.r * tint_color.r, 
                                 color.r * tint_color.g, 
                                 color.r * tint_color.b, 
                                 color.r * tint_color.a);
                }

                if (color.a < 0.01) {
                    discard;
                }
            }
        "#;

        let mut view = UIView::new(Rect::new(300.0, 300.0, 300.0, 100.0));
        view.background_color = Color::from_hex(0xffdab9);

        let mut child = UIView::new(Rect::new(150.0, 50.0, 300.0, 300.0));
        child.background_color = Color::from_hex(0x00abff);

        view.add_subview(Box::new(child));

        match ShaderProgram::new(vertex_src, fragment_src) {
            Ok(program) => Self {
                program,
                projection: OrthographicProjection::new(
                    0.0,
                    width as f32,
                    height as f32,
                    0.0,
                    -1.0,
                    1.0,
                ),
                view,
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

    pub fn update(&mut self) {
        self.view.update(Point::zero());
    }

    pub fn draw(&self) {
        self.program.use_program();
        self.program
            .set_uniform_m4("projection", self.projection.matrix());

        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
        }

        self.view.render(Point::zero(), &self.program);

        unsafe {
            gl::Disable(gl::BLEND);
        }
    }
}
