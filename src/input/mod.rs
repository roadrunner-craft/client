mod cursor;
pub mod keyboard;

pub use self::cursor::CursorDelta;
pub use self::cursor::CursorHandler;
pub use self::keyboard::KeyboardHandler;

use glutin::dpi::PhysicalPosition;
use glutin::event::KeyboardInput;
use scancode::Scancode;

#[derive(Debug, Default)]
pub struct InputHandler {
    keyboard: KeyboardHandler,
    cursor: CursorHandler,
}

impl InputHandler {
    pub fn process_keyboard(&mut self, input: KeyboardInput) {
        self.keyboard.process(input)
    }

    pub fn process_cursor(&mut self, input: PhysicalPosition<f64>) {
        self.cursor.process(input)
    }

    pub fn is_key_pressed(&self, key: Scancode) -> bool {
        self.keyboard.is_pressed(key)
    }

    pub fn get_cursor_delta(&self) -> &CursorDelta {
        self.cursor.get_delta()
    }

    pub fn clear_cursor_delta(&mut self) {
        self.cursor.clear_delta()
    }
}
