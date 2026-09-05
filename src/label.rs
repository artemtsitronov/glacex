use crate::color::Color;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

/// Typography label with semantic hierarchy and color styling.
pub struct Label {
    text: String,
    color: Option<Color>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label {
            text: text.into(),
            color: None,
        }
    }

    /// Sets an explicit text color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Convenience for secondary/supporting text hierarchy (`Theme::TEXT_SECONDARY`).
    pub fn secondary(mut self) -> Self {
        self.color = Some(Theme::TEXT_SECONDARY);
        self
    }

    /// Convenience for muted/subdued text hierarchy (`Theme::TEXT_MUTED`).
    pub fn muted(mut self) -> Self {
        self.color = Some(Theme::TEXT_MUTED);
        self
    }

    /// Convenience for primary accent-colored text (`Theme::ACTIVE`).
    pub fn accent(mut self) -> Self {
        self.color = Some(Theme::ACTIVE);
        self
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
        let color = self.color.unwrap_or(Theme::TEXT_PRIMARY);
        ui.draw_text_colored(&self.text, position, clip_rect, color);
    }
}
