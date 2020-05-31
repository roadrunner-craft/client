use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct KeyboardHandler {
    pressed: HashSet<VirtualKeyCode>,
    pressed_since_clear: HashSet<VirtualKeyCode>,
    released_since_clear: HashSet<VirtualKeyCode>,
}

impl KeyboardHandler {
    pub fn process(
        &mut self,
        KeyboardInput {
            virtual_keycode,
            state,
            ..
        }: KeyboardInput,
    ) {
        if let Some(keycode) = virtual_keycode {
            match state {
                ElementState::Pressed => {
                    if !self.is_pressed(keycode) {
                        self.pressed_since_clear.insert(keycode);
                    }
                    self.pressed.insert(keycode);
                }
                ElementState::Released => {
                    if self.is_pressed(keycode) {
                        self.released_since_clear.insert(keycode);
                    }
                    self.pressed.remove(&keycode);
                }
            };
        }
    }

    /// indicates a currently is currently being held
    pub fn is_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.pressed.contains(&(keycode))
    }

    /// indicates a previously unpressed key was just pressed
    pub fn just_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.pressed_since_clear.contains(&keycode)
    }

    /// indicates a previously pressed key was just released
    pub fn just_released(&self, keycode: VirtualKeyCode) -> bool {
        self.released_since_clear.contains(&keycode)
    }

    /// to call at the end of each gameloop
    pub fn clear(&mut self) {
        self.pressed_since_clear.clear();
        self.released_since_clear.clear()
    }
}
