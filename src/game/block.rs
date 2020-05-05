use crate::game::models::BlockProperties;

use serde::Deserialize;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct Block {
    pub id: u8,
}

pub struct BlockDatabase {
    map: HashMap<u8, BlockProperties>,
}

impl BlockDatabase {
    pub fn new() -> Self {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("res/data/blocks.json");
        let path = path.to_str().unwrap();

        let data =
            fs::read_to_string(path).expect("<block_database> Could not read data from file");

        Self {
            map: serde_json::from_str(&data).unwrap(),
        }
    }

    pub fn get(&self, id: u8) -> Option<&BlockProperties> {
        self.map.get(&id)
    }
}
