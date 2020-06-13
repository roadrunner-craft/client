use crate::ops::Drawable;
use crate::render::{
    mesh::TextureQuad,
    shaders::ShaderProgram,
    ui::{Point, Rect},
};
use crate::utils::Color;

type UIElementBox = Box<dyn UIElement>;

pub struct UIView {
    frame: Rect,
    quad: TextureQuad,
    needs_layout: bool,
    subviews: Vec<UIElementBox>,
    pub background_color: Color,
    pub tint_color: Color,
}

impl UIView {
    pub fn new(rect: Rect) -> Self {
        Self {
            quad: TextureQuad::new_rect(rect.x, rect.y, rect.width, rect.height),
            frame: rect,
            needs_layout: true,
            subviews: Vec::new(),
            background_color: Color::clear(),
            tint_color: Color::black(),
        }
    }

    pub fn add_subview(&mut self, view: UIElementBox) {
        self.subviews.push(view);
    }

    pub fn frame(&self) -> Rect {
        self.frame
    }

    pub fn set_frame(&mut self, rect: Rect) {
        self.needs_layout = true;
        self.frame = rect;
    }
}

impl UIElement for UIView {
    fn as_view(&mut self) -> &mut Self {
        self
    }

    fn update(&mut self, origin: Point) {
        let new_origin = origin + self.frame.origin();

        if self.needs_layout {
            self.quad = TextureQuad::new_rect(
                new_origin.x,
                new_origin.y,
                self.frame.width,
                self.frame.height,
            );

            self.needs_layout = false;
        }

        for subview in self.subviews.as_mut_slice() {
            subview.update(new_origin);
        }
    }

    fn render(&self, origin: Point, program: &ShaderProgram) {
        program.set_uniform_bool("render_texture", false);
        program.set_uniform_v4("background_color", self.background_color.as_vec());
        program.set_uniform_v4("tint_color", self.tint_color.as_vec());

        let new_origin = origin + self.frame.origin();

        self.quad.draw();

        for subview in self.subviews.as_slice() {
            subview.render(new_origin, program);
        }
    }
}

pub trait UIElement {
    fn as_view(&mut self) -> &mut UIView;

    fn update(&mut self, origin: Point);

    fn render(&self, origin: Point, program: &ShaderProgram);
}
