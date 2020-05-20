use crate::NETWORK_REFRESH_TIMEOUT;

use core::world::WorldCoordinate;

pub struct Player {
    id: u128,
    position: WorldCoordinate,
    start_position: WorldCoordinate,
    target_position: WorldCoordinate,
    elapsed_time: f64,
}

impl Player {
    pub fn new(id: u128) -> Self {
        Self {
            id,
            position: WorldCoordinate::zero(),
            start_position: WorldCoordinate::zero(),
            target_position: WorldCoordinate::zero(),
            elapsed_time: 0.0,
        }
    }

    pub fn position(&self) -> WorldCoordinate {
        self.position
    }

    pub fn set_position(&mut self, position: WorldCoordinate) {
        self.start_position = self.position;
        self.target_position = position;
        self.elapsed_time = 0.0;
    }

    pub fn update(&mut self, time_delta: f64) {
        self.elapsed_time += time_delta * 1000.0;

        let mut percent = self.elapsed_time as f32 / (2 * NETWORK_REFRESH_TIMEOUT) as f32;

        if percent > 1.0 {
            percent = 1.0;
        }

        self.position =
            self.start_position + (self.target_position - self.start_position) * percent;
    }
}
