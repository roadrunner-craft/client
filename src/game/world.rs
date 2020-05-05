use std::vec::Vec;

use crate::game::chunk::Chunk;
use crate::game::Block;

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
        self.chunks[0][0].set_layer(3, Block { id: 1 });
        self.chunks[0][0].set_layer(0, Block { id: 2 });
        self.chunks[0][0].set_layer(1, Block { id: 2 });
        self.chunks[0][0].set_layer(2, Block { id: 2 });
    }

    pub fn update(&mut self, time_delta: &f32) {}
}
