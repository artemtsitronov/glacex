use crate::Fill;
use crate::color::Color;
use crate::geometry::center_text_in;
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};
use std::default::Default;

pub type ButtonResponse = Interaction;

#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub fill: Fill,
    pub hover_fill: Fill,
    pub pressed_fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
    pub shadow: Option<ShadowStyle>,
    pub sharp: bool,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle {
            fill: Fill::Solid(Theme::IDLE),
            hover_fill: Fill::Solid(Theme::HOVERED),
            pressed_fill: Fill::Solid(Theme::ACTIVE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: 12.0,
            shadow: Some(ShadowStyle::default()),
            sharp: false,
        }
    }
}

pub struct Button {
    label: String,
    interaction: Interaction,
    style: Option<ButtonStyle>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Button {
            label: label.into(),
            interaction: Interaction::default(),
            style: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<ButtonStyle>) {
        self.style = style;
    }

    pub fn hovered(&self) -> bool {
        self.interaction.hovered
    }

    pub fn pressed(&self) -> bool {
        self.interaction.pressed
    }

    pub fn clicked(&self) -> bool {
        self.interaction.clicked
    }
}

impl Widget for Button {
    type Output = ButtonResponse;

    fn ui(&mut self, ui: &mut Ui) -> ButtonResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Button {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let padding = [16.0, 12.0];
        let text_width = ui.measure_text(&self.label);
        [
            text_width + padding[0] * 2.0,
            ui.line_height() + padding[1] * 2.0,
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> ButtonResponse {
        let style = self.style.clone().unwrap_or_default();

        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

        let color = if interaction.pressed {
            style.pressed_fill
        } else if interaction.hovered {
            style.hover_fill
        } else {
            style.fill
        };

        if let Some(shadow) = &style.shadow {
            draw_shadow(shadow, position, size, style.corner_radius, ui);
        }
        ui.draw_rect(
            position,
            size,
            color,
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            style.sharp,
        );

        let text_width = ui.measure_text(&self.label);
        let text_position = center_text_in(position, size, text_width, ui.line_height());
        let clip_rect = [
            position[0],
            position[1],
            position[0] + size[0],
            position[1] + size[1],
        ];
        ui.draw_text(&self.label, text_position, clip_rect);

        interaction
    }
}
