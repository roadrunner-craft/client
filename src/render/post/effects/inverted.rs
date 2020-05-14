use crate::render::display::FrameBuffer;
use crate::render::post::PostProcessingEffect;
use crate::render::shaders::ShaderProgram;

pub struct InvertedPostProcessing {
    program: ShaderProgram,
}

impl InvertedPostProcessing {
    pub fn new() -> Self {
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

        let fragment_src: &'static str = r#"
            #version 410 core
            
            in vec2 pass_uv;
            out vec4 color;

            uniform sampler2D input_texture;

            void main() {
                vec4 diffuse = texture(input_texture, pass_uv);
                color = vec4(1.0 - diffuse.r, 1.0 - diffuse.g, 1.0 - diffuse.b, 1.0);
            }
        "#;

        match ShaderProgram::new(vertex_src, fragment_src) {
            Ok(program) => Self { program },
            Err(err) => panic!(
                "<post> could not compile shaders in {}: \n\n{}\n",
                file!(),
                err
            ),
        }
    }
}

impl PostProcessingEffect for InvertedPostProcessing {
    fn prepare(&self, src: &FrameBuffer) {
        self.program.use_program();
        self.program
            .set_uniform_texture("input_texture", src.unit());
    }
}
