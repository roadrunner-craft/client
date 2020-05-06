use crate::game::block::Block;
use crate::game::chunk::{Chunk, ChunkGroup};
use crate::game::chunk::{ChunkGrid, ChunkGridCoordinate};

pub struct World {
    pub chunks: ChunkGrid,
}

impl World {
    pub fn new() -> Self {
        let mut w = World {
            chunks: ChunkGrid::new(),
        };

        w.chunks
            .insert(ChunkGridCoordinate::new(0, 0), Chunk::new());
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

    // TODO: handle the case where the current chunk is not in the hashmap
    pub fn get_chunk_group(&self, x: i64, z: i64) -> ChunkGroup {
        ChunkGroup {
            current: &self.chunks.get(&ChunkGridCoordinate::new(x, z)).unwrap(),
            north: self.chunks.get(&ChunkGridCoordinate::new(x, z + 1)),
            south: self.chunks.get(&ChunkGridCoordinate::new(x, z - 1)),
            east: self.chunks.get(&ChunkGridCoordinate::new(x - 1, z)),
            west: self.chunks.get(&ChunkGridCoordinate::new(x + 1, z)),
        }
    }
}
