use crate::color::Color;
use crate::fill::Fill;
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, StatefulWidget, Widget};

#[derive(Default)]
pub struct SwitchState {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SwitchResponse {
    pub enabled: bool,
    pub clicked: bool,
    pub hovered: bool,
}

#[derive(Debug, Clone)]
pub struct SwitchStyle {
    pub track_off_fill: Fill,
    pub track_on_fill: Fill,
    pub thumb_fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
    pub shadow: Option<ShadowStyle>,
}

impl Default for SwitchStyle {
    fn default() -> Self {
        SwitchStyle {
            track_off_fill: Fill::Solid(Theme::IDLE),
            track_on_fill: Fill::Solid(Theme::ACTIVE),
            thumb_fill: Fill::Solid(Color::WHITE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: 11.0,
            shadow: Some(ShadowStyle::default()),
        }
    }
}

pub struct Switch {
    id: String,
    style: Option<SwitchStyle>,
    interaction: Interaction,
    default_enabled: bool,
}

impl Switch {
    pub fn new(id: impl Into<String>) -> Self {
        Switch {
            id: id.into(),
            style: None,
            interaction: Interaction::default(),
            default_enabled: false,
        }
    }

    pub fn style(mut self, style: SwitchStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<SwitchStyle>) {
        self.style = style;
    }

    pub fn default_enabled(mut self, default_enabled: bool) -> Self {
        self.default_enabled = default_enabled;
        self
    }

    pub fn clicked(&self) -> bool {
        self.interaction.clicked
    }

    pub fn hovered(&self) -> bool {
        self.interaction.hovered
    }
}

impl Widget for Switch {
    type Output = SwitchResponse;

    fn ui(&mut self, ui: &mut Ui) -> SwitchResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Switch {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [40.0, 22.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> SwitchResponse {
        let style = self.style.clone().unwrap_or_default();
        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

        let state = ui.widget_state_or::<SwitchState>(&self.id, self.initial_state());
        if interaction.clicked {
            state.enabled = !state.enabled;
        }
        let enabled = state.enabled;

        let track_fill = if enabled {
            style.track_on_fill
        } else if interaction.hovered {
            Fill::Solid(Theme::HOVERED)
        } else {
            style.track_off_fill
        };

        if let Some(shadow) = &style.shadow {
            draw_shadow(shadow, position, size, style.corner_radius, ui);
        }

        // Draw track
        ui.draw_rect(
            position,
            size,
            track_fill,
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            false,
        );

        // Draw sliding knob / thumb
        let padding = 2.0;
        let knob_size = size[1] - padding * 2.0;
        let knob_x = if enabled {
            position[0] + size[0] - knob_size - padding
        } else {
            position[0] + padding
        };
        let knob_pos = [knob_x, position[1] + padding];
        let knob_radius = knob_size / 2.0;

        ui.draw_rect(
            knob_pos,
            [knob_size, knob_size],
            style.thumb_fill,
            knob_radius,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
        );

        SwitchResponse {
            enabled,
            clicked: interaction.clicked,
            hovered: interaction.hovered,
        }
    }
}

impl StatefulWidget for Switch {
    type State = SwitchState;

    fn state_id(&self) -> &str {
        &self.id
    }

    fn initial_state(&self) -> SwitchState {
        SwitchState {
            enabled: self.default_enabled,
            ..Default::default()
        }
    }
}
