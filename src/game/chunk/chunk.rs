use crate::game::block::Block;
//use crate::math::vector::v2;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

type Blocks = Vec<Vec<Vec<Block>>>;

pub struct Chunk {
    //position: v2,
    pub blocks: Blocks,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            //      position,
            blocks: vec![vec![vec![Block { id: 0 }; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_HEIGHT],
        }
    }

    pub fn set_layer(&mut self, y: usize, block: Block) {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                self.blocks[x][y][z] = block;
            }
        }
    }

    pub fn get_blocks(&self) -> &Blocks {
        &self.blocks
    }
}
