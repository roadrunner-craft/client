use glutin::event::{ElementState, KeyboardInput};
use scancode::Scancode;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct KeyboardHandler {
    pressed: HashSet<u32>,
}

impl KeyboardHandler {
    pub fn process(&mut self, input: KeyboardInput) {
        if let Some(scancode) = Scancode::new(input.scancode as u8) {
            // println!("{:?}", scancode);

            match input.state {
                ElementState::Pressed => self.pressed.insert(scancode as u32),
                ElementState::Released => self.pressed.remove(&(scancode as u32)),
            };
        }
    }

    pub fn is_pressed(&self, key: Scancode) -> bool {
        self.pressed.contains(&(key as u32))
    }
}
