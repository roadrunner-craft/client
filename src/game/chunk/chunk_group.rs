use crate::game::block::Block;
use crate::game::chunk::{Chunk, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};

pub struct ChunkGroup<'c> {
    pub current: &'c Chunk,
    pub north: Option<&'c Chunk>,
    pub south: Option<&'c Chunk>,
    pub east: Option<&'c Chunk>,
    pub west: Option<&'c Chunk>,
}

impl ChunkGroup<'_> {
    pub fn get_block(&self, x: i8, y: i16, z: i8) -> Option<Block> {
        if y < 0 || y > CHUNK_HEIGHT as i16 {
            return None;
        }

        let y = y as usize;

        if x < 0 {
            return Some(self.east?.get_blocks()[(x + CHUNK_WIDTH as i8) as usize][y][z as usize]);
        }

        let x = x as usize;

        if z < 0 {
            return Some(self.south?.get_blocks()[x][y][(z + CHUNK_DEPTH as i8) as usize]);
        }

        let z = z as usize;

        if x >= CHUNK_WIDTH {
            return Some(self.west?.get_blocks()[x - CHUNK_WIDTH][y][z]);
        }

        if z >= CHUNK_DEPTH {
            return Some(self.north?.get_blocks()[x][y][z - CHUNK_DEPTH]);
        }

        return Some(self.current.get_blocks()[x][y][z]);
    }
}
