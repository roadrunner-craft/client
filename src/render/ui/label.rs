use crate::ops::Drawable;
use crate::render::{
    mesh::TextureQuad,
    shaders::ShaderProgram,
    ui::{Font, Rect, UIElement, UIView, FONT_STORE},
};

use std::sync::Arc;

pub struct UILabel {
    view: UIView,
    text: String,
    font: Arc<Font>,
}

impl UILabel {
    pub fn new(rect: Rect) -> Self {
        Self {
            view: UIView::new(rect),
            text: String::default(),
            font: FONT_STORE
                .lock()
                .unwrap()
                .default_font()
                .unwrap_or_else(|| {
                    error!("could not load default font");
                    panic!();
                }),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl UIElement for UILabel {
    fn as_view(&self) -> &UIView {
        &self.view
    }

    fn as_view_mut(&mut self) -> &mut UIView {
        &mut self.view
    }

    fn render(&self, program: &ShaderProgram) {
        self.view.render(program);

        program.set_uniform_bool("render_texture", true);

        for (rect, texture) in self.font.iter_for(&self.text) {
            program.set_uniform_texture("diffuse_texture", texture.unit());

            let quad = TextureQuad::new_rect(rect.x, rect.y, rect.width, rect.height);
            quad.draw();
        }
    }
}
