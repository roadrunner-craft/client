#[derive(Debug, Copy, Clone)]
pub struct RenderSettings {
    pub wireframe: bool,
}

impl RenderSettings {
    pub fn apply(&self) {
        unsafe {
            if self.wireframe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self { wireframe: false }
    }
}
