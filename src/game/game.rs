use crate::game::block::BlockDatabase;
use crate::game::World;

pub struct Game {
    pub world: World,
    pub block_database: BlockDatabase,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            block_database: BlockDatabase::new(),
        }
    }

    pub fn update(&mut self, _time_delta: &f32) {}
}
