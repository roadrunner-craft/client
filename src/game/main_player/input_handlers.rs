use glutin::event::{VirtualKeyCode, ElementState};
use std::sync::mpsc::{Sender, Receiver, channel};

use crate::input::InputHandler;
use crate::game::main_player::{MainPlayer, Callback};

impl MainPlayer {

    pub fn process_inputs(&mut self) {
        let handlers = self.event_handlers.iter().filter_map(|(receiver, on_press, on_release)| {
            match receiver.try_recv() {
               Ok(ElementState::Pressed) => Some(on_press.clone()),
               Ok(ElementState::Released) => Some(on_release.clone()),
               Err(_) => None
            }
        }).collect::<Vec<Callback>>();

        for handler in handlers {
            (handler)(self);
        }
    }

    pub fn register_input_handlers(&mut self, input: &mut InputHandler) {
        self.register_move_forward(input);
        self.register_move_backward(input);
        self.register_move_left(input);
        self.register_move_right(input);
        self.register_move_up(input);
        self.register_move_down(input);
    }

    pub fn register_move_forward(&mut self, input: &mut InputHandler) {
        let on_pressed = |player: &mut MainPlayer| {
            player.is_moving_forward = true;
        };

        let on_released = |player: &mut MainPlayer| {
            player.is_moving_forward = false;
        };

        let (sender, receiver) = channel();
        self.event_handlers.push((receiver, on_pressed, on_released));
        input.register(VirtualKeyCode::W, sender)
    }

    pub fn register_move_backward(&mut self, input: &mut InputHandler) {
        let on_pressed = |player: &mut MainPlayer| {
            player.is_moving_backward = true;
        };

        let on_released = |player: &mut MainPlayer| {
            player.is_moving_backward = false;
        };

        let (sender, receiver) = channel();
        self.event_handlers.push((receiver, on_pressed, on_released));
        input.register(VirtualKeyCode::S, sender)
    }

    pub fn register_move_left(&mut self, input: &mut InputHandler) {
        let on_pressed = |player: &mut MainPlayer| {
            player.is_moving_left = true;
        };

        let on_released = |player: &mut MainPlayer| {
            player.is_moving_left = false;
        };

        let (sender, receiver) = channel();
        self.event_handlers.push((receiver, on_pressed, on_released));
        input.register(VirtualKeyCode::A, sender)
    }

    pub fn register_move_right(&mut self, input: &mut InputHandler) {
        let on_pressed = |player: &mut MainPlayer| {
            player.is_moving_right = true;
        };

        let on_released = |player: &mut MainPlayer| {
            player.is_moving_right = false;
        };

        let (sender, receiver) = channel();
        self.event_handlers.push((receiver, on_pressed, on_released));
        input.register(VirtualKeyCode::D, sender)
    }

    pub fn register_move_up(&mut self, input: &mut InputHandler) {
        let on_pressed = |player: &mut MainPlayer| {
            player.is_moving_up = true;
        };

        let on_released = |player: &mut MainPlayer| {
            player.is_moving_up = false;
        };

        let (sender, receiver) = channel();
        self.event_handlers.push((receiver, on_pressed, on_released));
        input.register(VirtualKeyCode::Space, sender)
    }

    pub fn register_move_down(&mut self, input: &mut InputHandler) {
        let on_pressed = |player: &mut MainPlayer| {
            player.is_moving_down = true;
        };

        let on_released = |player: &mut MainPlayer| {
            player.is_moving_down = false;
        };

        let (sender, receiver) = channel();
        self.event_handlers.push((receiver, on_pressed, on_released));
        input.register(VirtualKeyCode::LShift, sender)
    }
}
