use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct CursorDelta {
    pub x: f64,
    pub y: f64,
}

impl ops::AddAssign for CursorDelta {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Default)]
pub struct CursorHandler {
    delta: CursorDelta,
}

impl CursorHandler {
    pub fn process(&mut self, delta: (f64, f64)) {
        self.delta += CursorDelta {
            x: delta.0,
            y: delta.1,
        };
    }

    pub fn get_delta(&self) -> &CursorDelta {
        &self.delta
    }

    pub fn clear(&mut self) {
        self.delta = CursorDelta::default();
    }
}
