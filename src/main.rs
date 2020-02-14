mod math;

use glium::{
    Display,
    Surface,
    glutin::{
        ContextBuilder,
        window::WindowBuilder,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop}
    }
};

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Hello world");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = &event {
            *control_flow = ControlFlow::Exit;
            return;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
    });
}

