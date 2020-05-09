use crate::game::block::Block;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

type Blocks = Vec<Vec<Vec<Block>>>;

pub struct Chunk {
    pub blocks: Blocks,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: vec![vec![vec![Block { id: 0 }; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_HEIGHT],
        }
    }

    pub fn get_blocks(&self) -> &Blocks {
        &self.blocks
    }
}
