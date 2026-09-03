use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::center_text_in;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    Default,
    Outline,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct BadgeStyle {
    pub fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
    pub padding: [f32; 2],
}

impl Default for BadgeStyle {
    fn default() -> Self {
        BadgeStyle {
            fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: 10.0,
            padding: [8.0, 3.0],
        }
    }
}

pub struct Badge {
    text: String,
    style: BadgeStyle,
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Badge {
            text: text.into(),
            style: BadgeStyle::default(),
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        match variant {
            BadgeVariant::Default => {
                self.style.fill = Fill::Solid(Theme::SURFACE_SUBTLE);
                self.style.border_color = Theme::BORDER;
            }
            BadgeVariant::Outline => {
                self.style.fill = Fill::Solid(Color::TRANSPARENT);
                self.style.border_color = Theme::BORDER_STRONG;
            }
            BadgeVariant::Success => {
                self.style.fill = Fill::Solid(Theme::SUCCESS.with_alpha(0.18));
                self.style.border_color = Theme::SUCCESS.with_alpha(0.4);
            }
            BadgeVariant::Warning => {
                self.style.fill = Fill::Solid(Theme::WARNING.with_alpha(0.18));
                self.style.border_color = Theme::WARNING.with_alpha(0.4);
            }
            BadgeVariant::Error => {
                self.style.fill = Fill::Solid(Theme::ERROR.with_alpha(0.18));
                self.style.border_color = Theme::ERROR.with_alpha(0.4);
            }
        }
        self
    }

    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Badge {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for Badge {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let text_width = ui.measure_text(&self.text);
        [
            text_width + self.style.padding[0] * 2.0,
            ui.line_height() + self.style.padding[1] * 2.0,
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        ui.draw_rect(
            position,
            size,
            self.style.fill.clone(),
            self.style.corner_radius,
            self.style.border_width,
            self.style.border_color,
            0.0,
            false,
        );

        let text_width = ui.measure_text(&self.text);
        let text_pos = center_text_in(position, size, text_width, ui.line_height());
        let clip_rect = [
            position[0],
            position[1],
            position[0] + size[0],
            position[1] + size[1],
        ];
        ui.draw_text(&self.text, text_pos, clip_rect);
    }
}
