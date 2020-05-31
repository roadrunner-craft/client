use glutin::event::VirtualKeyCode;

use crate::input::{InputHandler, InputObserver, InputCallback};
use crate::game::main_player::MainPlayer;

impl InputObserver for MainPlayer {

    fn get_input_callbacks(&mut self) -> &Vec<InputCallback<Self>> {
        &self.input_callbacks
    }
}

impl MainPlayer {
    pub fn register_input_handlers(&mut self, input : &mut InputHandler) {
        self.register_move_forward();
        // self.register_move_backward();
        // self.register_move_left();
        // self.register_move_right();
        // self.register_move_up();
        // self.register_move_down();

        input.register(self);
    }

    fn register_move_forward(&mut self) {
        let on_pressed = |player : &mut Self| { player.is_moving_forward = true; };
        let on_released = |player : &mut Self| { player.is_moving_forward = false; };
        let callback = InputCallback::new(VirtualKeyCode::W, on_pressed, on_released);
        self.input_callbacks.push(callback);
    }
}
