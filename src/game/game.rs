use crate::game::block::BlockDatabase;
use crate::game::World;
use crate::input::InputHandler;

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

    pub fn update(&mut self, input: &InputHandler, _time_delta: &f32) {
        self.world.update(input);
    }
}
