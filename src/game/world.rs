use crate::game::block::Block;
use crate::game::chunk::{Chunk, ChunkGroup};
use crate::utils::point::Point;

use std::collections::HashMap;

pub struct World {
    pub chunks: HashMap<Point, Chunk>,
}

impl World {
    pub fn new() -> Self {
        let mut w = World {
            chunks: HashMap::new(),
        };

        w.chunks.insert(Point { x: 0, y: 0 }, Chunk::new());
        w.chunks.insert(Point { x: 1, y: 0 }, Chunk::new());
        w.chunks.insert(Point { x: 0, y: 1 }, Chunk::new());
        w.chunks.insert(Point { x: 0, y: -1 }, Chunk::new());
        w.chunks.insert(Point { x: -1, y: 0 }, Chunk::new());
        w
    }

    pub fn init(&mut self) {
        for chunk in self.chunks.values_mut() {
            for i in 0..5 {
                chunk.set_layer(i, Block { id: 7 });
            }

            for i in 5..20 {
                chunk.set_layer(i, Block { id: 1 });
            }

            for i in 20..23 {
                chunk.set_layer(i, Block { id: 3 });
            }

            chunk.set_layer(23, Block { id: 2 });

            for i in 0..2 {
                for j in 0..5 {
                    for k in 0..5 {
                        chunk.blocks[6 + j][27 + i][6 + k] = Block { id: 18 };
                    }
                }
            }

            for j in 1..4 {
                for k in 1..4 {
                    chunk.blocks[6 + j][29][6 + k] = Block { id: 18 };
                }
            }

            chunk.blocks[8][30][8] = Block { id: 18 };
            chunk.blocks[7][30][8] = Block { id: 18 };
            chunk.blocks[9][30][8] = Block { id: 18 };
            chunk.blocks[8][30][7] = Block { id: 18 };
            chunk.blocks[8][30][9] = Block { id: 18 };

            for i in 24..30 {
                chunk.blocks[8][i][8] = Block { id: 17 };
            }
        }
    }

    pub fn get_chunk_group(&self, x: isize, y: isize) -> ChunkGroup {
        ChunkGroup {
            current: &self.chunks.get(&Point { x: 0, y: 0 }).unwrap(),
            north: &self.chunks.get(&Point { x: 0, y: 1 }).unwrap(),
            south: &self.chunks.get(&Point { x: 0, y: -1 }).unwrap(),
            east: &self.chunks.get(&Point { x: -1, y: 0 }).unwrap(),
            west: &self.chunks.get(&Point { x: 1, y: 0 }).unwrap(),
        }
    }
}
