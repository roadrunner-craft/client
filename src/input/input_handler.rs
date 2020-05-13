use glutin::event::{KeyboardInput, VirtualKeyCode};
use crate::input::{CursorDelta, CursorHandler, KeyboardHandler};
use crate::traits::InputObserver;
use std::collections::HashMap;

#[derive(Default)]
pub struct InputHandler<'a> {
    observers: HashMap<VirtualKeyCode, Vec<&'a InputObserver>>
}

impl<'a> InputHandler<'a> {
    fn new() -> InputHandler<'a> {
        InputHandler {
            observers: HashMap::new(),
        }
    }

    fn attach<T: InputObserver>(&mut self, key: VirtualKeyCode, observer: &'a T) {
        let obs = self.observers.entry(key).or_insert(Vec::new());
        obs.push(observer);
    }
}

// impl InputHandler {
//     pub fn process_keyboard(&mut self, input: KeyboardInput) {
//         self.keyboard.process(input)
//     }

//     pub fn process_cursor(&mut self, input: (f64, f64)) {
//         self.cursor.process(input)
//     }

//     pub fn is_key_pressed(&self, keycode: VirtualKeyCode) -> bool {
//         self.keyboard.is_pressed(keycode)
//     }

//     pub fn get_cursor_delta(&self) -> &CursorDelta {
//         self.cursor.get_delta()
//     }

//     pub fn clear_cursor_delta(&mut self) {
//         self.cursor.clear_delta()
//     }
// }
