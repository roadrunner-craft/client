use crate::ops::Drawable;
use crate::render::{mesh::TextureQuad, shaders::ShaderProgram, ui::Rect};
use crate::utils::Color;

pub struct UIView<'a> {
    frame: Rect,
    quad: TextureQuad,
    parent: Option<Box<&'a dyn UIElement<'a>>>,
    subviews: Vec<Box<dyn UIElement<'a>>>,
    pub background_color: Color,
}

impl<'a> UIView<'a> {
    pub fn new(rect: Rect) -> Self {
        Self {
            quad: TextureQuad::new_rect(rect.x, rect.y, rect.width, rect.height),
            frame: rect,
            parent: None,
            background_color: Color::black(),
            subviews: Vec::new(),
        }
    }

    pub fn add_subview(&'a mut self, view: Box<dyn UIElement<'a>>) {
        (*view).set_parent(Box::new(self));
        self.subviews.push(view);
    }

    pub fn subviews(&self) -> &Vec<Box<dyn UIElement>> {
        &self.subviews
    }

    pub fn frame(&self) -> Rect {
        self.frame
    }

    pub fn set_frame(&mut self, rect: Rect) {
        self.frame = rect;

        let mut abs_frame = rect;
        let mut current = self.parent;

        while let Some(p) = current {
            let parent_frame = (**p).frame();

            abs_frame.x += parent_frame.x;
            abs_frame.y += parent_frame.y;
            current = (**p).parent;
        }

        self.quad =
            TextureQuad::new_rect(abs_frame.x, abs_frame.y, abs_frame.width, abs_frame.height);
    }
}

impl<'a> UIElement<'a> for UIView<'a> {
    fn as_view(&'a self) -> &'a Self {
        self
    }

    fn as_mut_view(&'a mut self) -> &'a mut Self {
        self
    }

    fn parent(&'a self) -> Option<&'a Box<dyn UIElement>> {
        self.parent
    }

    fn render(&self, program: &ShaderProgram) {
        program.set_uniform_bool("render_texture", false);
        program.set_uniform_v4("background_color", self.background_color.as_vec());

        self.quad.draw();

        // render subviews
    }
}

pub trait UIElement<'a> {
    fn as_view(&'a self) -> &'a UIView;
    fn as_mut_view(&'a mut self) -> &'a mut UIView;

    fn parent(&'a self) -> Option<&'a Box<dyn UIElement>>;

    fn render(&self, program: &ShaderProgram);
}
