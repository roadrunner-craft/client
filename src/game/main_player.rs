use crate::input::InputHandler;
use crate::render::camera::{Camera, PerspectiveCamera};

use core::block::BlockRegistry;
use core::world::{World, WorldCoordinate};
use glutin::event::VirtualKeyCode;
use math::container::Volume;
use math::vector::Vector3;
use std::fs;
use std::path::Path;

const SPEED: f64 = 13.0;
const SENSITIVITY: f32 = 0.1;

pub struct MainPlayer {
    pub camera: PerspectiveCamera,
}

impl MainPlayer {
    pub fn new(position: WorldCoordinate) -> Self {
        let mut p = Self {
            camera: PerspectiveCamera::new(70.0, 0.1, 1024.0),
        };

        p.set_position(position);
        p
    }

    fn set_position(&mut self, position: WorldCoordinate) {
        self.camera.set_position(position);
    }

    pub fn position(&self) -> Vector3 {
        self.camera.position()
    }

    pub fn update(&mut self, time_delta: f64, world: &World, input: &InputHandler) {
        let cursor_delta = input.get_cursor_delta();
        let camera_delta = Vector3 {
            x: cursor_delta.y as f32,
            y: cursor_delta.x as f32,
            z: 0.0,
        } * SENSITIVITY;
        let mut camera_angles = self.camera.euler_angles() + camera_delta;

        if camera_angles.x > 90.0 {
            camera_angles.x = 90.0;
        } else if camera_angles.x < -90.0 {
            camera_angles.x = -90.0;
        }

        camera_angles.y %= 360.0;

        self.camera.set_euler_angles(camera_angles);

        let mut xaxis = 0.0;
        let mut yaxis = 0.0;
        let mut zaxis = 0.0;

        if input.is_key_pressed(VirtualKeyCode::W) {
            zaxis += 1.0;
        }

        if input.is_key_pressed(VirtualKeyCode::S) {
            zaxis -= 1.0;
        }

        if input.is_key_pressed(VirtualKeyCode::A) {
            xaxis -= 1.0;
        }

        if input.is_key_pressed(VirtualKeyCode::D) {
            xaxis += 1.0;
        }

        if input.is_key_pressed(VirtualKeyCode::Space) {
            yaxis += 1.0;
        }

        if input.is_key_pressed(VirtualKeyCode::LShift) {
            yaxis -= 1.0;
        }

        let angle = self.camera.euler_angles().y.to_radians();

        let mut delta = Vector3 {
            x: xaxis * angle.cos() + zaxis * angle.sin(),
            y: yaxis,
            z: -xaxis * angle.sin() + zaxis * angle.cos(),
        };
        delta = delta * (SPEED * time_delta) as f32;

        // remove this and move into a cache somewhere else
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("res/data/blocks.json");
        let path = path.to_str().unwrap();
        let data =
            fs::read_to_string(path).expect("<block_database> Could not read data from file");
        let block_registry = BlockRegistry::new(serde_json::from_str(&data).unwrap());

        let old_position = self.camera.position();
        let mut new_position = old_position + delta;

        let new_positions = vec![
            old_position
                + Vector3 {
                    x: delta.x,
                    y: 0.0,
                    z: 0.0,
                },
            old_position
                + Vector3 {
                    x: 0.0,
                    y: delta.y,
                    z: 0.0,
                },
            old_position
                + Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: delta.z,
                },
        ];

        for (i, position) in new_positions.iter().enumerate() {
            let block_position =
                Vector3::new(position.x.round(), position.y.round(), position.z.round());

            let mut blocks = vec![];

            for x in -3..=3 {
                for z in -3..=3 {
                    for y in -3..=3 {
                        blocks.push(
                            block_position
                                + Vector3 {
                                    x: x as f32,
                                    y: y as f32,
                                    z: z as f32,
                                },
                        )
                    }
                }
            }

            // println!("{:?}", old_position);

            for block in blocks {
                if let Some(block_info) =
                    world.get_block(block.x as isize, block.y as isize, block.z as isize)
                {
                    if !block_registry.is_solid(block_info.id) {
                        continue;
                    }

                    let player_min = Vector3 {
                        x: position.x - 0.35,
                        y: position.y - 0.5,
                        z: position.z - 0.35,
                    };
                    let player_max = Vector3 {
                        x: position.x + 0.35,
                        y: position.y + 0.5,
                        z: position.z + 0.35,
                    };
                    let block_min = Vector3 {
                        x: block.x - 0.5,
                        y: block.y,
                        z: block.z - 0.5,
                    };
                    let block_max = Vector3 {
                        x: block.x + 0.5,
                        y: block.y + 1.0,
                        z: block.z + 0.5,
                    };

                    if player_min.x < block_max.x
                        && player_max.x > block_min.x
                        && player_min.y < block_max.y
                        && player_max.y > block_min.y
                        && player_min.z < block_max.z
                        && player_max.z > block_min.z
                    {
                        if i == 0 {
                            new_position.x = old_position.x;
                        } else if i == 1 {
                            new_position.y = old_position.y;
                        } else if i == 2 {
                            new_position.z = old_position.z;
                        }
                    }
                }
            }
        }

        self.set_position(new_position);
    }
}
