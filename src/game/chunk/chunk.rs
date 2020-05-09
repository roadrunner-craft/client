use crate::game::block::Block;
use crate::game::chunk::chunk_grid::ChunkGridCoordinate;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

type Blocks = Vec<Vec<Vec<Block>>>;

pub struct Chunk {
    pub blocks: Blocks,
    pub coords: ChunkGridCoordinate,
}

impl Chunk {
    pub fn new(coords: ChunkGridCoordinate) -> Self {
        Self {
            blocks: vec![vec![vec![Block { id: 0 }; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_HEIGHT],
            coords,
        }
    }

    pub fn get_blocks(&self) -> &Blocks {
        &self.blocks
    }
}
