use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::center_text_in;
use crate::painter::FontWeight;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct BadgeStyle {
    pub fill: Fill,
    pub text_color: Color,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
    pub padding: [f32; 2],
}

impl Default for BadgeStyle {
    fn default() -> Self {
        BadgeStyle {
            fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            text_color: Theme::TEXT_SECONDARY,
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: Theme::RADIUS_FULL,
            padding: [8.0, 2.0],
        }
    }
}

pub struct Badge {
    text: String,
    variant: BadgeVariant,
    style: Option<BadgeStyle>,
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Badge {
            text: text.into(),
            variant: BadgeVariant::Default,
            style: None,
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.variant = BadgeVariant::Secondary;
        self
    }

    pub fn outline(mut self) -> Self {
        self.variant = BadgeVariant::Outline;
        self
    }

    pub fn success(mut self) -> Self {
        self.variant = BadgeVariant::Success;
        self
    }

    pub fn warning(mut self) -> Self {
        self.variant = BadgeVariant::Warning;
        self
    }

    pub fn error(mut self) -> Self {
        self.variant = BadgeVariant::Error;
        self
    }

    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<BadgeStyle>) {
        self.style = style;
    }

    fn resolved_style(&self, theme: &Theme) -> BadgeStyle {
        if let Some(s) = &self.style {
            return s.clone();
        }
        match self.variant {
            BadgeVariant::Default => BadgeStyle {
                fill: Fill::Solid(theme.active),
                text_color: if theme.is_dark {
                    Color::BLACK
                } else {
                    Color::WHITE
                },
                border_width: 1.0,
                border_color: Color::TRANSPARENT,
                corner_radius: Theme::RADIUS_FULL,
                padding: [8.0, 2.0],
            },
            BadgeVariant::Secondary => BadgeStyle {
                fill: Fill::Solid(theme.surface_subtle),
                text_color: theme.text_secondary,
                border_width: 1.0,
                border_color: theme.border_faint,
                corner_radius: Theme::RADIUS_FULL,
                padding: [8.0, 2.0],
            },
            BadgeVariant::Outline => BadgeStyle {
                fill: Fill::Solid(Color::TRANSPARENT),
                text_color: theme.text_primary,
                border_width: 1.0,
                border_color: theme.border,
                corner_radius: Theme::RADIUS_FULL,
                padding: [8.0, 2.0],
            },
            BadgeVariant::Success => BadgeStyle {
                fill: Fill::Solid(theme.success.with_alpha(0.12)),
                text_color: theme.success,
                border_width: 1.0,
                border_color: theme.success.with_alpha(0.28),
                corner_radius: Theme::RADIUS_FULL,
                padding: [8.0, 2.0],
            },
            BadgeVariant::Warning => BadgeStyle {
                fill: Fill::Solid(theme.warning.with_alpha(0.12)),
                text_color: theme.warning,
                border_width: 1.0,
                border_color: theme.warning.with_alpha(0.28),
                corner_radius: Theme::RADIUS_FULL,
                padding: [8.0, 2.0],
            },
            BadgeVariant::Error => BadgeStyle {
                fill: Fill::Solid(theme.error.with_alpha(0.12)),
                text_color: theme.error,
                border_width: 1.0,
                border_color: theme.error.with_alpha(0.28),
                corner_radius: Theme::RADIUS_FULL,
                padding: [8.0, 2.0],
            },
        }
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
        let style = self.resolved_style(ui.theme());
        let text_width = ui.measure_text_styled(&self.text, 12.0, 16.0, FontWeight::Medium, false);
        [
            text_width + style.padding[0] * 2.0,
            20.0, // sleek 20px badge height
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let style = self.resolved_style(ui.theme());

        ui.draw_rect(
            position,
            size,
            style.fill,
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            false,
            0.0,
        );

        let text_width = ui.measure_text_styled(&self.text, 12.0, 16.0, FontWeight::Medium, false);
        let text_pos = center_text_in(position, size, text_width, 16.0);
        let clip_rect = [
            position[0],
            position[1],
            position[0] + size[0],
            position[1] + size[1],
        ];
        ui.draw_text_styled(
            &self.text,
            text_pos,
            clip_rect,
            style.text_color,
            12.0,
            16.0,
            FontWeight::Medium,
            false,
        );
    }
}
