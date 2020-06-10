use crate::render::display::FrameBuffer;
use crate::render::post::PostProcessingEffect;
use crate::render::shaders::ShaderProgram;

pub struct IdentityPostProcessing {
    program: ShaderProgram,
}

impl IdentityPostProcessing {
    pub fn new() -> Self {
        let vertex_src: &'static str = r#"
            #version 410 core

            layout (location=0) in vec3 position;
            layout (location=1) in vec2 uv_data;

            out vec2 uv;

            void main() {
                uv = uv_data;
                gl_Position = vec4(position, 1.0);
            }
        "#;

        let fragment_src: &'static str = r#"
            #version 410 core
            
            in vec2 uv;
            out vec4 color;

            uniform sampler2D input_texture;

            void main() {
                color = texture(input_texture, uv);
            }
        "#;

        match ShaderProgram::new(vertex_src, fragment_src) {
            Ok(program) => Self { program },
            Err(err) => {
                error!("could not compile shader program {}:{}", file!(), line!());
                panic!("\n{}\n", err);
            }
        }
    }
}

impl PostProcessingEffect for IdentityPostProcessing {
    fn prepare(&self, src: &FrameBuffer) {
        self.program.use_program();
        self.program
            .set_uniform_texture("input_texture", src.unit());
    }
}
