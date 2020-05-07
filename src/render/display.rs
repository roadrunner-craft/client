use glutin::dpi::Size;
use glutin::event_loop::EventLoop;
use glutin::monitor::MonitorHandle;
use glutin::window::{Fullscreen, WindowBuilder};
use glutin::{Api, ContextBuilder, GlProfile, GlRequest, PossiblyCurrent, WindowedContext};

pub struct Display {
    pub context: WindowedContext<PossiblyCurrent>,
    monitor: MonitorHandle,
}

impl Display {
    pub fn new(title: &'static str, event_loop: &EventLoop<()>) -> Self {
        let monitor = event_loop.available_monitors().nth(0).unwrap();
        let psize = monitor.size();
        let size = Size::Physical(psize);

        let window_builder = WindowBuilder::new()
            .with_title(title)
            //           .with_maximized(false)
            //         .with_resizable(true)
            .with_inner_size(size);
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

        let psize = context.window().inner_size();
        unsafe {
            gl::Viewport(0, 0, psize.width as i32, psize.height as i32);
        }

        Display { context, monitor }
    }

    pub fn set_fullscreen(&self, value: bool) {
        self.context.window().set_fullscreen(if value {
            Some(Fullscreen::Borderless(self.monitor.clone()))
        } else {
            None
        });
    }
}
