use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};
use std::collections::HashMap;

#[derive(Debug, Default)]
struct KeyState {
    is_pressed: bool
}

#[derive(Debug, Default)]
pub struct Keyboard {
    states: HashMap<VirtualKeyCode, KeyState>
}

impl Keyboard {
    pub fn press(&mut self, keycode: VirtualKeyCode) {
        self.states.entry(keycode).or_default().is_pressed = true;
    }

    pub fn release(&mut self, keycode: VirtualKeyCode) {
        self.states.entry(keycode).or_default().is_pressed = false;
    }

    pub fn is_pressed(&self, keycode: VirtualKeyCode) -> bool {
        if let Some(state) = self.states.get(&keycode) {
            return state.is_pressed
        }

        return false
    }
}
