use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct TextureDatabase {
    pub map: HashMap<u8, String>,
}

impl TextureDatabase {
    pub fn new() -> Self {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("res/data/textures.json");
        let path = path.to_str().unwrap();

        let data =
            fs::read_to_string(path).expect("<texture_database> Could not read data from file");

        Self {
            map: serde_json::from_str(&data).unwrap(),
        }
    }

    pub fn iter(&self) -> Iter<u8, String> {
        self.map.iter()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}
