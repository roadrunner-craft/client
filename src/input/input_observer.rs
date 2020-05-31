use crate::input::InputCallback;
use glutin::event::ElementState;

pub trait InputObserver {

    fn get_input_callbacks(&mut self) -> &Vec<InputCallback<Self>>
        where Self: Sized;

    fn process_inputs(&mut self) where Self: Sized {
        let handlers = self.get_input_callbacks()
                           .iter()
                           .filter_map(|callback| {
            match callback.receiver.try_recv() {
               Ok(ElementState::Pressed) => Some(callback.on_pressed.clone()),
               Ok(ElementState::Released) => Some(callback.on_released.clone()),
               Err(_) => None
            }
        }).collect::<Vec<fn(&mut Self)>>();

        for handler in handlers {
            (handler)(self);
        }
    }
}
