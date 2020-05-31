use glutin::event::{KeyboardInput, VirtualKeyCode};

use crate::input::{CursorDelta, CursorHandler, KeyboardHandler};
use std::sync::mpsc::{Sender, channel};
use std::collections::{HashMap};

#[derive(Debug, Default)]
pub struct InputHandler {
    keyboard: KeyboardHandler,
    cursor: CursorHandler,

    keycode_channels: HashMap<VirtualKeyCode, Vec<Sender<VirtualKeyCode>>>
}

impl InputHandler {
    pub fn process_keyboard(
        &mut self,
        KeyboardInput {
            virtual_keycode,
            state,
            ..
        }: KeyboardInput
    ) {

        if let Some(keycode) = virtual_keycode {
            for sender in self.keycode_channels.entry(keycode).or_default() {
                sender.send(keycode);
            };
        }
        // self.keyboard.process(input)
    }

    pub fn process_cursor(&mut self, input: (f64, f64)) {
        self.cursor.process(input)
    }

    pub fn is_key_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.keyboard.is_pressed(keycode)
    }

    pub fn just_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.keyboard.just_pressed(keycode)
    }

    pub fn just_released(&self, keycode: VirtualKeyCode) -> bool {
        self.keyboard.just_released(keycode)
    }

    pub fn get_cursor_delta(&self) -> &CursorDelta {
        self.cursor.get_delta()
    }

    pub fn clear(&mut self) {
        self.cursor.clear();
        self.keyboard.clear();
    }

    pub fn register(&mut self, keycode: VirtualKeyCode, sender: Sender<VirtualKeyCode>) {
        self.keycode_channels.entry(keycode).or_default().push(sender);
    }
}
