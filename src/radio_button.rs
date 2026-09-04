use crate::color::Color;
use crate::fill::Fill;
use crate::interaction::Interaction;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

#[derive(Debug, Clone, Copy)]
pub struct RadioButtonResponse {
    pub selected: bool,
    pub clicked: bool,
    pub hovered: bool,
}

use crate::shadow::{ShadowStyle, draw_shadow};

#[derive(Debug, Clone)]
pub struct RadioButtonStyle {
    pub fill: Fill,
    pub hover_fill: Fill,
    pub selected_fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
    pub shadow: Option<ShadowStyle>,
    pub sharp: bool,
}

impl Default for RadioButtonStyle {
    fn default() -> Self {
        RadioButtonStyle {
            fill: Fill::Solid(Theme::IDLE),
            hover_fill: Fill::Solid(Theme::HOVERED),
            selected_fill: Fill::Solid(Theme::ACTIVE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: 9.0,
            shadow: Some(ShadowStyle::default()),
            sharp: false,
        }
    }
}

pub struct RadioButton {
    group_id: String,
    option_id: String,
    style: Option<RadioButtonStyle>,
    interaction: Interaction,
}

impl RadioButton {
    pub fn new(group_id: impl Into<String>, option_id: impl Into<String>) -> Self {
        RadioButton {
            group_id: group_id.into(),
            option_id: option_id.into(),
            style: None,
            interaction: Interaction::default(),
        }
    }

    pub fn style(mut self, style: RadioButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<RadioButtonStyle>) {
        self.style = style;
    }

    pub fn clicked(&self) -> bool {
        self.interaction.clicked
    }
    pub fn hovered(&self) -> bool {
        self.interaction.hovered
    }
}

impl Widget for RadioButton {
    type Output = RadioButtonResponse;

    fn ui(&mut self, ui: &mut Ui) -> RadioButtonResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for RadioButton {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [18.0, 18.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> RadioButtonResponse {
        let style = self.style.clone().unwrap_or_default();
        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

        if interaction.clicked {
            ui.select(&self.group_id, &self.option_id);
        }
        let selected = ui.is_selected(&self.group_id, &self.option_id);

        let fill = if selected {
            style.selected_fill
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
            fill,
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            style.sharp,
        );

        if selected {
            // Draw inner white dot for high-end radio button aesthetic
            let dot_inset = 5.0;
            let dot_pos = [position[0] + dot_inset, position[1] + dot_inset];
            let dot_size = [size[0] - dot_inset * 2.0, size[1] - dot_inset * 2.0];
            let dot_radius = (size[0] - dot_inset * 2.0) / 2.0;
            ui.draw_rect(
                dot_pos,
                dot_size,
                Fill::Solid(Color::WHITE),
                dot_radius,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
            );
        }

        RadioButtonResponse {
            selected,
            clicked: interaction.clicked,
            hovered: interaction.hovered,
        }
    }
}
