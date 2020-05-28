use crate::render::display::FrameBuffer;
use crate::render::post::PostProcessingEffect;
use crate::render::shaders::ShaderProgram;

pub struct FXAAPostProcessing {
    program: ShaderProgram,
}

impl FXAAPostProcessing {
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

            #define FXAA_REDUCE_MIN   (1.0/128.0)
            #define FXAA_REDUCE_MUL   (1.0/8.0)
            #define FXAA_SPAN_MAX     8.0
            
            in vec2 pass_uv;

            out vec4 color;

            uniform sampler2D input_texture;
            uniform vec2 buffer_size;

            vec3 fxaa(sampler2D tex, vec2 uv, vec2 texel_size) {
                vec3 rgb_north_west = texture(tex, uv + vec2(-1.0, -1.0) * texel_size).rgb;
                vec3 rgb_north_east = texture(tex, uv + vec2(1.0, -1.0) * texel_size).rgb;
                vec3 rgb_south_west = texture(tex, uv + vec2(-1.0, 1.0) * texel_size).rgb;
                vec3 rgb_south_east = texture(tex, uv + vec2(1.0, 1.0) * texel_size).rgb;
                vec3 rgb_current = texture(tex, uv).rgb;

                vec3 luma = vec3(0.299, 0.587, 0.114);
                float luma_north_west = dot(rgb_north_west, luma);
                float luma_north_east = dot(rgb_north_east, luma);
                float luma_south_west = dot(rgb_south_west, luma);
                float luma_south_east = dot(rgb_south_east, luma);
                float luma_current = dot(rgb_current, luma);

                float luma_min = min(luma_current, min(min(luma_north_west, luma_north_east), min(luma_south_west, luma_south_east)));
                float luma_max = max(luma_current, max(max(luma_north_west, luma_north_east), max(luma_south_west, luma_south_east)));

                // edge detection, gives the blur direction
                vec2 direction = vec2(
                    -((luma_north_west + luma_north_east) - (luma_south_west + luma_south_east)),
                    (luma_north_west + luma_south_west) - (luma_north_east + luma_south_east)
                );

                // hack to avoid division by zero errors
                float direction_reduction = max(
                    FXAA_REDUCE_MIN,
                    (luma_north_west + luma_north_east + luma_south_west + luma_south_east) * (0.25 * FXAA_REDUCE_MUL)  // biased luma average
                );
                
                float reciprocal_scaler = 1.0 / (min(abs(direction.x), abs(direction.y)) + direction_reduction);

                // this is the scaled direction vector used to apply the blur
                direction = clamp(direction * reciprocal_scaler, -FXAA_SPAN_MAX, FXAA_SPAN_MAX) * texel_size;

                vec3 rgb_near = (1.0/2.0) * (
                    texture(tex, uv + direction * (1.0/3.0 - 0.5)).rgb +
                    texture(tex, uv + direction * (2.0/3.0 - 0.5)).rgb);
                    
                vec3 rgb_far = rgb_near * (1.0/2.0) + (1.0/4.0) * (
                    texture(tex, uv + direction * (0.0/3.0 - 0.5)).rgb +
                    texture(tex, uv + direction * (3.0/3.0 - 0.5)).rgb);
                
                float luma_far = dot(rgb_far, luma);

                return (luma_far < luma_min || luma_far > luma_max) ? 
                    rgb_near :
                    rgb_far;
            }

            void main() {
                vec2 texel_size = vec2(1.0 / buffer_size.x, 1.0 / buffer_size.y);
                color = vec4(fxaa(input_texture, pass_uv, texel_size), 1.0);
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

impl PostProcessingEffect for FXAAPostProcessing {
    fn prepare(&self, src: &FrameBuffer) {
        self.program.use_program();
        self.program
            .set_uniform_texture("input_texture", src.unit());
        self.program.set_uniform_v2("buffer_size", src.size());
    }
}
