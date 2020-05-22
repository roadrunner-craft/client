use crate::NETWORK_REFRESH_TIMEOUT;

use core::world::WorldCoordinate;
use std::time::Instant;

pub struct Player {
    id: u128,
    position: WorldCoordinate,
    start_position: WorldCoordinate,
    target_position: WorldCoordinate,
    last_update: Instant,
}

impl Player {
    pub fn new(id: u128) -> Self {
        Self {
            id,
            position: WorldCoordinate::zero(),
            start_position: WorldCoordinate::zero(),
            target_position: WorldCoordinate::zero(),
            last_update: Instant::now(),
        }
    }

    pub fn position(&self) -> WorldCoordinate {
        self.position
    }

    pub fn set_position(&mut self, position: WorldCoordinate) {
        self.start_position = self.position;
        self.target_position = position;
        self.last_update = Instant::now();
    }

    pub fn update(&mut self) {
        let percent = (self.last_update.elapsed().as_secs_f32() * 1000.0
            / (2 * NETWORK_REFRESH_TIMEOUT) as f32)
            .min(1.0);

        self.position =
            self.start_position + (self.target_position - self.start_position) * percent;
    }
}
