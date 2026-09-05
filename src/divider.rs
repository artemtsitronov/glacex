use crate::color::Color;
use crate::fill::Fill;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerOrientation {
    Horizontal,
    Vertical,
}

pub struct Divider {
    orientation: DividerOrientation,
    length: f32,
    thickness: f32,
    color: Color,
}

impl Divider {
    pub fn horizontal(length: f32) -> Self {
        Divider {
            orientation: DividerOrientation::Horizontal,
            length,
            thickness: 1.0,
            color: Theme::BORDER,
        }
    }

    pub fn vertical(length: f32) -> Self {
        Divider {
            orientation: DividerOrientation::Vertical,
            length,
            thickness: 1.0,
            color: Theme::BORDER,
        }
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets the divider to an ultra-subtle hairline (`Theme::BORDER_FAINT`).
    pub fn faint(mut self) -> Self {
        self.color = Theme::BORDER_FAINT;
        self
    }
}

impl Widget for Divider {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for Divider {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        match self.orientation {
            DividerOrientation::Horizontal => [self.length, self.thickness],
            DividerOrientation::Vertical => [self.thickness, self.length],
        }
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        ui.draw_rect(
            position,
            size,
            Fill::Solid(self.color),
            0.0,
            0.0,
            Color::TRANSPARENT,
            0.0,
            true,
            0.0,
        );
    }
}
