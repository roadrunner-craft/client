use glutin::event::{KeyboardInput, VirtualKeyCode, ElementState};

use crate::input::{CursorDelta, CursorHandler, Keyboard, Axis};
use std::sync::mpsc::Sender;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct InputHandler {
    keyboard: Keyboard,
    cursor: CursorHandler,
    input_axes: HashMap<Axis, AxisControl>
}

#[derive(Debug)]
pub struct AxisControl {
    up_key: VirtualKeyCode,
    down_key: VirtualKeyCode,
}

impl InputHandler {

    pub fn new() -> Self {
        let mut i = Self::default();
        i.input_axes.insert(Axis::Vertical, AxisControl {
            up_key: VirtualKeyCode::W,
            down_key: VirtualKeyCode::S,
        });

        i.input_axes.insert(Axis::Horizontal, AxisControl {
            up_key: VirtualKeyCode::D,
            down_key: VirtualKeyCode::A,
        });

        i
    }
    pub fn process_keyboard(
        &mut self,
        KeyboardInput {
            virtual_keycode,
            state,
            ..
        }: KeyboardInput
    ) {

        if let Some(keycode) = virtual_keycode {
            match state {
                ElementState::Pressed => { self.keyboard.press(keycode) }
                ElementState::Released => { self.keyboard.release(keycode) }
            };
        };
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

    pub fn clear(&mut self) {
        self.cursor.clear();
    }

    pub fn get_axis(&self, axis: Axis) -> f32 {

        let mut val = 0.0;

        if let Some(axis_control) = self.input_axes.get(&axis) {
            if self.is_key_pressed(axis_control.up_key) { val += 1.0 }
            if self.is_key_pressed(axis_control.down_key) { val -= 1.0 }
        }

        return val
    }
}
