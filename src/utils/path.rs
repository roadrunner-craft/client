use std::fmt;
use std::path::{Path, PathBuf};

pub enum ResourceType {
    Texture,
}

impl ResourceType {
    fn extension(&self) -> &'static str {
        match self {
            ResourceType::Texture => "png",
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            ResourceType::Texture => "textures",
        };

        write!(f, "{}", value)
    }
}

pub enum ResourceSubtype {
    Block,
}

impl fmt::Display for ResourceSubtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            ResourceSubtype::Block => "block",
        };

        write!(f, "{}", value)
    }
}

pub struct ResourcePath {
    path: Box<Path>,
}

impl ResourcePath {
    pub fn new(resource_type: ResourceType, resource_subtype: ResourceSubtype, s: &String) -> Self {
        let mut buf = PathBuf::new();

        buf.push("res");
        buf.push(resource_type.to_string());
        buf.push(resource_subtype.to_string());
        buf.push(s);
        buf.set_extension(resource_type.extension());

        Self {
            path: buf.into_boxed_path(),
        }
    }

    pub fn as_path(&self) -> &Path {
        &*self.path
    }
}
