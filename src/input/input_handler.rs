use glutin::event::{KeyboardInput, VirtualKeyCode};

use crate::input::{CursorDelta, CursorHandler, KeyboardHandler};

#[derive(Debug, Default)]
pub struct InputHandler {
    keyboard: KeyboardHandler,
    cursor: CursorHandler,
}

impl InputHandler {
    pub fn process_keyboard(&mut self, input: KeyboardInput) {
        self.keyboard.process(input)
    }

    pub fn process_cursor(&mut self, input: (f64, f64)) {
        self.cursor.process(input)
    }

    pub fn is_key_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.keyboard.is_pressed(keycode)
    }

    pub fn get_cursor_delta(&self) -> &CursorDelta {
        self.cursor.get_delta()
    }

    pub fn clear_cursor_delta(&mut self) {
        self.cursor.clear_delta()
    }

    pub fn clear_keyboard_state(&mut self) {
        self.keyboard.clear_keyboard_state()
    }
}
