use glutin::event::{ElementState, KeyboardInput, ScanCode};
use std::collections::HashSet;

pub const W: ScanCode = 13;
pub const A: ScanCode = 0;
pub const S: ScanCode = 1;
pub const D: ScanCode = 2;
pub const M: ScanCode = 46;
pub const SPACE: ScanCode = 49;
pub const LEFT_SHIFT: ScanCode = 56;

#[derive(Debug, Default)]
pub struct KeyboardHandler {
    pressed: HashSet<ScanCode>,
}

impl KeyboardHandler {
    pub fn process(&mut self, input: KeyboardInput) {
        // println!("{}", input.scancode);

        match input.state {
            ElementState::Pressed => self.pressed.insert(input.scancode),
            ElementState::Released => self.pressed.remove(&input.scancode),
        };
    }

    pub fn is_pressed(&self, key: ScanCode) -> bool {
        self.pressed.contains(&key)
    }
}
