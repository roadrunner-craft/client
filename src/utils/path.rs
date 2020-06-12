use std::fmt;
use std::path::{Path, PathBuf};

pub enum ResourceType {
    Texture,
    Font,
}

impl ResourceType {
    fn extension(&self) -> &'static str {
        match self {
            ResourceType::Texture => "png",
            ResourceType::Font => "ttf",
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            ResourceType::Texture => "textures",
            ResourceType::Font => "fonts",
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
    pub fn new(resource_type: ResourceType, s: &String) -> Self {
        ResourcePath::generate_path(resource_type, None, s)
    }

    pub fn new_with_subtype(
        resource_type: ResourceType,
        resource_subtype: ResourceSubtype,
        s: &String,
    ) -> Self {
        ResourcePath::generate_path(resource_type, Some(resource_subtype), s)
    }

    fn generate_path(
        resource_type: ResourceType,
        resource_subtype: Option<ResourceSubtype>,
        s: &String,
    ) -> Self {
        let mut buf = PathBuf::new();

        buf.push("res");
        buf.push(resource_type.to_string());

        if let Some(subtype) = resource_subtype {
            buf.push(subtype.to_string());
        }

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
