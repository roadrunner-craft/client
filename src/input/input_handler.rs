use glutin::event::{KeyboardInput, VirtualKeyCode};
use crate::input::{CursorDelta, CursorHandler, KeyboardHandler};
use crate::traits::InputObserver;
use std::collections::HashMap;

#[derive(Default)]
pub struct InputHandler {
    observers: HashMap<VirtualKeyCode, Vec<&'static dyn InputObserver>>
}

impl<'a> InputHandler {
    fn new() -> InputHandler {
        InputHandler {
            observers: HashMap::new(),
        }
    }

    pub fn attach<T: InputObserver>(&mut self, key: VirtualKeyCode, observer: &'static T) {
        let observers = self.observers.entry(key).or_default();
        observers.push(observer);
    }

    pub fn process_keyboard(&mut self,
                            KeyboardInput { virtual_keycode, state, .. }: KeyboardInput) {

        let maybe_observers = match virtual_keycode {
            Some(keycode) => self.observers.get(&keycode),
            _ => return
        };

        match maybe_observers {
            Some(observers) => for observer in observers { observer.handle_input() },
            _ => return
        }
    }


    ///
    pub fn process_cursor(&mut self, input: (f64, f64)) {
        // self.cursor.process(input)
    }

    pub fn is_key_pressed(&self, keycode: VirtualKeyCode) -> bool {
        // self.keyboard.is_pressed(keycode)
        true
    }

    pub fn get_cursor_delta(&self) { // -> &CursorDelta {
        // self.cursor.get_delta()
    }

    pub fn clear_cursor_delta(&mut self) {
        // self.cursor.clear_delta()
    }
}

