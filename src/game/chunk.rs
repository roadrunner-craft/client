use crate::game::Block;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

pub struct Chunk {
    pub blocks: [[[Block; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_HEIGHT],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[Block { id: 0 }; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_HEIGHT],
        }
    }

    pub fn set_layer(&mut self, y: usize, block: Block) {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                self.blocks[x][y][z] = block;
            }
        }
    }
}
