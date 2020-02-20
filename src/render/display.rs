use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlProfile, GlRequest, PossiblyCurrent, WindowedContext};

pub struct Display {
    pub context: WindowedContext<PossiblyCurrent>,
}

impl Display {
    // TODO: handle fullscreen
    pub fn create(title: &'static str, event_loop: &EventLoop<()>) -> Self {
        let window_builder = WindowBuilder::new().with_title(title);
        let context_builder = ContextBuilder::new()
            .with_srgb(true)
            .with_vsync(true)
            .with_depth_buffer(24)
            .with_gl_profile(GlProfile::Core)
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)));

        let context = context_builder
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let context = unsafe { context.make_current().unwrap() };

        gl::load_with(|symbol| context.get_proc_address(symbol));

        let size = context.window().inner_size();
        unsafe {
            gl::Viewport(0, 0, size.width as i32, size.height as i32);
        }

        Display { context }
    }
}
