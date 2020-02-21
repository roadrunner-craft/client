use glutin::dpi::PhysicalPosition;
use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct CursorDelta {
    x: f64,
    y: f64,
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
    old_position: Option<PhysicalPosition<f64>>,
}

impl CursorHandler {
    pub fn process(&mut self, position: PhysicalPosition<f64>) {
        if self.old_position == None {
            self.old_position = Some(position);
            return;
        }

        let added_delta = CursorDelta {
            x: position.x - self.old_position.unwrap().x,
            y: position.y - self.old_position.unwrap().y,
        };

        self.old_position = Some(position);
        self.delta += added_delta;
    }

    pub fn get_delta(&self) -> &CursorDelta {
        &self.delta
    }

    pub fn clear_delta(&mut self) {
        self.delta = CursorDelta::default();
    }
}
