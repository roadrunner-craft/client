use core::world::WorldCoordinate;

pub struct Player {
    id: u128,
    pub position: WorldCoordinate,
}

impl Player {
    pub fn new(id: u128) -> Self {
        Self {
            id,
            position: WorldCoordinate::zero(),
        }
    }
}
