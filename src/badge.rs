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
    style: Option<BadgeStyle>,
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Badge {
            text: text.into(),
            style: None,
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        let mut style = self.style.take().unwrap_or_default();
        match variant {
            BadgeVariant::Default => {
                style.fill = Fill::Solid(Theme::SURFACE_SUBTLE);
                style.border_color = Theme::BORDER;
            }
            BadgeVariant::Outline => {
                style.fill = Fill::Solid(Color::TRANSPARENT);
                style.border_color = Theme::BORDER_STRONG;
            }
            BadgeVariant::Success => {
                style.fill = Fill::Solid(Theme::SUCCESS.with_alpha(0.18));
                style.border_color = Theme::SUCCESS.with_alpha(0.4);
            }
            BadgeVariant::Warning => {
                style.fill = Fill::Solid(Theme::WARNING.with_alpha(0.18));
                style.border_color = Theme::WARNING.with_alpha(0.4);
            }
            BadgeVariant::Error => {
                style.fill = Fill::Solid(Theme::ERROR.with_alpha(0.18));
                style.border_color = Theme::ERROR.with_alpha(0.4);
            }
        }
        self.style = Some(style);
        self
    }

    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<BadgeStyle>) {
        self.style = style;
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
        let style = self.style.clone().unwrap_or_default();
        let text_width = ui.measure_text(&self.text);
        [
            text_width + style.padding[0] * 2.0,
            ui.line_height() + style.padding[1] * 2.0,
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let style = self.style.clone().unwrap_or_default();

        ui.draw_rect(
            position,
            size,
            style.fill.clone(),
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            false,
            0.0,
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
