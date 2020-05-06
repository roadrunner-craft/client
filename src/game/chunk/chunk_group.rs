use crate::game::block::Block;
use crate::game::chunk::{Chunk, CHUNK_DEPTH, CHUNK_WIDTH};

pub struct ChunkGroup<'c> {
    pub current: &'c Chunk,
    pub north: &'c Chunk,
    pub south: &'c Chunk,
    pub east: &'c Chunk,
    pub west: &'c Chunk,
}

impl ChunkGroup<'_> {
    pub fn get_block(&self, x: i8, y: u8, z: i8) -> Block {
        let y = y as usize;

        if x < 0 {
            return self.west.get_blocks()[(x + CHUNK_WIDTH as i8) as usize][y][z as usize];
        }

        let x = x as usize;

        if z < 0 {
            return self.south.get_blocks()[x][y][(z + CHUNK_DEPTH as i8) as usize];
        }

        let z = z as usize;

        if x >= CHUNK_WIDTH {
            return self.east.get_blocks()[x - CHUNK_WIDTH][y][z];
        }

        if z >= CHUNK_DEPTH {
            return self.north.get_blocks()[x][y][z - CHUNK_DEPTH];
        }

        return self.current.get_blocks()[x][y][z];
    }
}
