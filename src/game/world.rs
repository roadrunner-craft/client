use std::vec::Vec;

use crate::game::block::Block;
use crate::game::chunk::Chunk;

pub struct World {
    pub chunks: Vec<Vec<Chunk>>,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: vec![vec![Chunk::new()]],
        }
    }

    pub fn init(&mut self) {
        for i in 0..5 {
            self.chunks[0][0].set_layer(i, Block { id: 7 });
        }

        for i in 5..20 {
            self.chunks[0][0].set_layer(i, Block { id: 1 });
        }

        for i in 20..23 {
            self.chunks[0][0].set_layer(i, Block { id: 3 });
        }

        self.chunks[0][0].set_layer(23, Block { id: 2 });
    }
}
