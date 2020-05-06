use crate::game::chunk::Chunk;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Default, Copy, Clone, Debug)]
pub struct ChunkGridCoordinate {
    pub x: i64,
    pub z: i64,
}

impl ChunkGridCoordinate {
    pub fn new(x: i64, z: i64) -> Self {
        Self { x, z }
    }
}

pub type ChunkGrid = HashMap<ChunkGridCoordinate, Chunk>;
