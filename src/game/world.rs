use crate::game::block::Block;
use crate::game::chunk::{Chunk, ChunkGroup, CHUNK_DEPTH, CHUNK_WIDTH};
use crate::game::chunk::{ChunkGrid, ChunkGridCoordinate};
use crate::game::generation::HeightMap;
use crate::input::InputHandler;

use glutin::event::VirtualKeyCode;
use math::vector::Vector3;

pub type WorldCoordinate = Vector3;

pub struct World {
    pub chunks: ChunkGrid,
    pub render_distance: i64,
    height_map: HeightMap,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: ChunkGrid::new(),
            render_distance: 3,
            height_map: HeightMap::new(50..75, 12923874),
        }
    }

    pub fn generate_chunk(&self, coords: ChunkGridCoordinate) -> Chunk {
        let mut chunk = Chunk::new();

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                let absx = x as i64 + coords.x * CHUNK_WIDTH as i64;
                let absz = z as i64 + coords.z * CHUNK_DEPTH as i64;

                let height = self.height_map.get_height(absx, absz) as usize;

                for y in 0..5 {
                    chunk.blocks[x][y][z] = Block { id: 7 };
                }

                for y in 5..10 {
                    chunk.blocks[x][y][z] = Block { id: 4 };
                }

                for y in 10..(height - 3) {
                    chunk.blocks[x][y][z] = Block { id: 1 };
                }

                for y in (height - 3)..height {
                    let id = if height < 59 { 12 } else { 3 };
                    chunk.blocks[x][y][z] = Block { id };
                }

                let id = if height < 59 { 12 } else { 2 };
                chunk.blocks[x][height][z] = Block { id };

                if height < 58 {
                    for y in height..59 {
                        chunk.blocks[x][y][z] = Block { id: 9 };
                    }
                }
            }
        }

        chunk
    }

    pub fn load_chunk(&mut self, x: i64, z: i64) {
        let coords = ChunkGridCoordinate::new(x, z);
        if !self.chunks.contains_key(&coords) {
            let chunk = self.generate_chunk(coords);
            self.chunks.insert(coords, chunk);
        }
    }

    pub fn unload_chunk(&mut self, x: i64, y: i64) {
        let coords = ChunkGridCoordinate::new(x, y);
        self.chunks.remove(&coords);
    }

    pub fn init(&mut self) {
        for x in -self.render_distance..self.render_distance {
            for y in -self.render_distance..self.render_distance {
                self.load_chunk(x, y);
            }
        }

        // const size = 512;
        // let height_map = HeightMap::new(0..255, 1239874);

        // let mut img = Vec::new();
        // img.resize(size * size, 255);

        // for y in 0..size {
        //     for x in 0..size {
        //         let index = y * size + x;
        //         img[index as usize] = height_map.get_height(x, y) as u8;
        //     }
        // }

        // image::save_buffer(
        //     "./noise.png",
        //     img.as_slice(),
        //     size,
        //     size,
        //     image::ColorType::L8,
        // );
    }

    pub fn update(&mut self, input: &InputHandler, player_position: WorldCoordinate) {
        if cfg!(feature = "chunk_loading") {
            // (un?)load chunks as the player moves
            let player_chunk = ChunkGridCoordinate::from_world_coordinate(player_position);
            let range = self.render_distance;
            let is_near =
                |middle, point| -> bool { (middle - range..middle + range).contains(&point) };
            self.chunks.retain(|coord, _| {
                is_near(player_chunk.x, coord.x) && is_near(player_chunk.z, coord.z)
            });
            for x in player_chunk.x - self.render_distance..player_chunk.x + self.render_distance {
                for z in
                    player_chunk.z - self.render_distance..player_chunk.z + self.render_distance
                {
                    self.load_chunk(x, z);
                }
            }
        }

        // increment render distance
        if input.is_key_pressed(VirtualKeyCode::K) {
            self.render_distance += 1;
            for x in -self.render_distance..self.render_distance {
                for y in -self.render_distance..self.render_distance {
                    self.load_chunk(x, y);
                }
            }
        }

        // decrement render distance
        if input.is_key_pressed(VirtualKeyCode::J) {
            if self.render_distance > 1 {
                for i in -self.render_distance..self.render_distance {
                    self.unload_chunk(self.render_distance - 1, i);
                    self.unload_chunk(-self.render_distance, i);
                    self.unload_chunk(i, self.render_distance - 1);
                    self.unload_chunk(i, -self.render_distance);
                }
                self.render_distance -= 1;
            }
        }
    }

    // TODO: handle the case where the current chunk is not in the hashmap
    pub fn get_chunk_group(&self, x: i64, z: i64) -> ChunkGroup {
        ChunkGroup {
            current: &self.chunks.get(&ChunkGridCoordinate::new(x, z)).unwrap(),
            north: self.chunks.get(&ChunkGridCoordinate::new(x, z + 1)),
            south: self.chunks.get(&ChunkGridCoordinate::new(x, z - 1)),
            east: self.chunks.get(&ChunkGridCoordinate::new(x - 1, z)),
            west: self.chunks.get(&ChunkGridCoordinate::new(x + 1, z)),
        }
    }
}
