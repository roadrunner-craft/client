use crate::render::display::FrameBuffer;
use crate::render::post::PostProcessingEffect;
use crate::render::shaders::ShaderProgram;

use std::time::Instant;

pub struct StaticWavePostProcessing {
    program: ShaderProgram,
    time: Instant,
}

impl StaticWavePostProcessing {
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
            uniform float offset;

            void main() {
                vec2 uv = pass_uv;
                uv.x += sin(uv.y * 8.0 * 3.14159 + offset) / 100.0;
                color = texture(input_texture, uv);
            }
        "#;

        match ShaderProgram::new(vertex_src, fragment_src) {
            Ok(program) => Self {
                program,
                time: Instant::now(),
            },
            Err(err) => panic!(
                "<post> could not compile shaders in {}: \n\n{}\n",
                file!(),
                err
            ),
        }
    }
}

impl PostProcessingEffect for StaticWavePostProcessing {
    fn prepare(&self, src: &FrameBuffer) {
        self.program.use_program();
        self.program
            .set_uniform_texture("input_texture", src.unit());
        self.program.set_uniform_float(
            "offset",
            self.time.elapsed().as_secs_f32() * 2.0 * 3.14159 * 0.75,
        );
    }
}
