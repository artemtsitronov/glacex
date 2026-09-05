use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label { text: text.into() }
    }
}

impl Widget for Label {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for Label {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let text_width = ui.measure_text(&self.text);
        [text_width, ui.line_height()]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let clip_rect = [
            position[0],
            position[1],
            position[0] + size[0],
            position[1] + size[1],
        ];
        ui.draw_text(&self.text, position, clip_rect);
    }
}
