use crate::game::entity::Player;
use crate::ops::Drawable;
use crate::render::camera::Camera;
use crate::render::mesh::PlayerMesh;
use crate::render::shaders::ShaderProgram;

use math::vector::Vector3;

pub struct PlayerRenderer {
    program: ShaderProgram,
    mesh: PlayerMesh,
}

impl PlayerRenderer {
    pub fn new() -> Self {
        let vertex_src: &'static str = r#"
            #version 410 core

            layout (location=0) in vec3 position;

            uniform vec3 world_position;
            uniform mat4 projection_view; // projection * view

            void main() {
                gl_Position = projection_view * vec4(world_position + position, 1.0);
            }
        "#;

        let fragment_src: &'static str = r#"
            #version 410 core

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.5, 0.0, 1.0);
            }
        "#;

        match ShaderProgram::new(vertex_src, fragment_src) {
            Ok(program) => Self {
                program,
                mesh: PlayerMesh::new(),
            },
            Err(err) => {
                panic!(
                    "<player-renderer> could not compile the shader program:\n\n{}\n",
                    err
                );
            }
        }
    }

    pub fn draw<C: Camera>(&self, camera: &C, players: &Vec<&Player>) {
        self.program.use_program();
        self.program
            .set_uniform_m4("projection_view", camera.projection_view());

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);

            for player in players.iter() {
                // TODO: add camera frustum check

                self.program.set_uniform_v3(
                    "world_position",
                    player.position()
                        - Vector3 {
                            x: 0.5,
                            y: 1.5,
                            z: 0.5,
                        },
                );

                self.mesh.draw();
            }
        }
    }
}
