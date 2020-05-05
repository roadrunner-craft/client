use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct BlockProperties {
    pub name: String,
    pub solid: bool,
    pub opaque: bool,
    pub texture: BlockTextureProperties,
}

impl Default for BlockProperties {
    fn default() -> Self {
        Self {
            name: String::from("minecraft:undefined"),
            solid: true,
            opaque: true,
            texture: BlockTextureProperties::default(),
        }
    }
}

#[derive(Deserialize, Default, Copy, Clone)]
#[serde(default)]
pub struct BlockTextureProperties {
    pub front: u8,
    pub back: u8,
    pub left: u8,
    pub right: u8,
    pub top: u8,
    pub bottom: u8,
}
