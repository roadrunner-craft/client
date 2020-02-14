mod math;

extern crate gl;
extern crate glutin;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new().with_title("Minecraft.rs");
    let context = glutin::ContextBuilder::new()
        .with_srgb(true)
        .with_vsync(true)
        .with_depth_buffer(24)
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 1)));
    let windowed_context = context.build_windowed(window, &event_loop).unwrap();

    let display = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|symbol| display.get_proc_address(symbol));

    event_loop.run(move |event, _, control_flow| {
        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = &event
        {
            *control_flow = ControlFlow::Exit;
            return;
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        display.swap_buffers().unwrap();
    });
}
