use glutin::event::{VirtualKeyCode, ElementState};
use std::sync::mpsc::{Sender, Receiver, channel};

pub struct InputCallback<T> {
    pub keycode : VirtualKeyCode,
    pub sender : Sender<ElementState>,
    pub receiver : Receiver<ElementState>,
    pub on_pressed : fn(&mut T),
    pub on_released : fn(&mut T)
}

impl<T> InputCallback<T> {
    pub fn new(keycode : VirtualKeyCode,
               on_pressed : fn(&mut T),
               on_released : fn(&mut T)) -> Self {
        let (sender, receiver) = channel();
        Self {
            keycode: keycode,
            sender: sender,
            receiver: receiver,
            on_pressed: on_pressed,
            on_released: on_released,
        }
    }
}
