use crate::game::chunk::Chunk;
use crate::game::world::WorldCoordinate;

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

    pub fn from_world_coordinate(WorldCoordinate { x, z, .. }: WorldCoordinate) -> Self {
        Self {
            x: (x / 16.0).floor() as i64,
            z: (z / 16.0).floor() as i64,
        }
    }
}

pub type ChunkGrid = HashMap<ChunkGridCoordinate, Chunk>;
