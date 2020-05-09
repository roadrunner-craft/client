use crate::game::block::Block;
use crate::game::chunk::{Chunk, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};

pub struct ChunkGroup<'c> {
    pub current: &'c Chunk,
    pub north: Option<&'c Chunk>,
    pub south: Option<&'c Chunk>,
    pub east: Option<&'c Chunk>,
    pub west: Option<&'c Chunk>,
    count: usize,
}

impl<'c> ChunkGroup<'c> {
    pub fn new(
        current: &'c Chunk,
        north: Option<&'c Chunk>,
        south: Option<&'c Chunk>,
        east: Option<&'c Chunk>,
        west: Option<&'c Chunk>,
    ) -> Self {
        Self {
            current,
            north,
            south,
            east,
            west,
            count: 0,
        }
    }
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

impl<'c> Iterator for ChunkGroup<'c> {
    type Item = Option<&'c Chunk>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        self.count += 1;
        match self.count {
            1 => Some(self.north),
            2 => Some(self.south),
            3 => Some(self.east),
            4 => Some(self.west),
            _ => None,
        }
    }
}
