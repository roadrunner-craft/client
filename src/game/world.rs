use crate::game::block::Block;
use crate::game::chunk::{Chunk, ChunkGroup, CHUNK_DEPTH, CHUNK_WIDTH};
use crate::game::chunk::{ChunkGrid, ChunkGridCoordinate};
use crate::game::generation::HeightMap;

pub struct World {
    pub chunks: ChunkGrid,
}

impl World {
    pub fn new() -> Self {
        let mut w = World {
            chunks: ChunkGrid::new(),
        };

        for x in -3..3 {
            for y in -3..3 {
                w.chunks
                    .insert(ChunkGridCoordinate::new(x, y), Chunk::new());
            }
        }
        w
    }

    pub fn init(&mut self) {
        let height_map = HeightMap::new(50..75, 12923874);

        for (coords, chunk) in self.chunks.iter_mut() {
            for x in 0..CHUNK_WIDTH {
                for z in 0..CHUNK_DEPTH {
                    let absx = x as i64 + coords.x * CHUNK_WIDTH as i64;
                    let absz = z as i64 + coords.z * CHUNK_DEPTH as i64;

                    let height = height_map.get_height(absx, absz) as usize;

                    for y in 0..5 {
                        chunk.blocks[x][y][z] = Block { id: 7 };
                    }

                    for y in 5..(height - 3) {
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
