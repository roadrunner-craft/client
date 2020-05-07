use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct KeyboardHandler {
    pressed: HashSet<VirtualKeyCode>,
}

impl KeyboardHandler {
    pub fn process(&mut self, input: KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            // println!("{:?}", keycode);

            match input.state {
                ElementState::Pressed => self.pressed.insert(keycode),
                ElementState::Released => self.pressed.remove(&(keycode)),
            };
        }
    }

    pub fn is_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.pressed.contains(&(keycode))
    }
}
