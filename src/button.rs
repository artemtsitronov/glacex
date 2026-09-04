use crate::Fill;
use crate::animation::animate_towards;
use crate::color::Color;
use crate::geometry::center_text_in;
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, StatefulWidget, Widget};
use std::default::Default;
use winit::window::CursorIcon;

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
            corner_radius: 8.0,
            shadow: Some(ShadowStyle::default()),
            sharp: false,
        }
    }
}

/// Per-button animation state — persists between frames.
pub struct ButtonState {
    /// 0.0 = resting, 1.0 = fully hovered/pressed.
    pub hover_t: f32,
    pub press_t: f32,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState {
            hover_t: 0.0,
            press_t: 0.0,
        }
    }
}

pub struct Button {
    label: String,
    interaction: Interaction,
    style: Option<ButtonStyle>,
    tooltip: Option<String>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Button {
            label: label.into(),
            interaction: Interaction::default(),
            style: None,
            tooltip: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<ButtonStyle>) {
        self.style = style;
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
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
        let dt = ui.dt();

        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

        // Animate hover and press smoothly
        let state_id = format!(
            "__btn_anim_{}_{}_{}",
            self.label, position[0] as i32, position[1] as i32
        );
        let state = ui.widget_state::<ButtonState>(&state_id);

        let hover_target = if interaction.hovered { 1.0f32 } else { 0.0 };
        let press_target = if interaction.pressed { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, 0.055);
        state.press_t = animate_towards(state.press_t, press_target, dt, 0.04);
        let hover_t = state.hover_t;
        let press_t = state.press_t;

        // Blend fill colors based on animation progress
        let color = if let (Fill::Solid(base), Fill::Solid(hov), Fill::Solid(prs)) =
            (&style.fill, &style.hover_fill, &style.pressed_fill)
        {
            let blended = base.lerp(*hov, hover_t);
            Fill::Solid(blended.lerp(*prs, press_t))
        } else if interaction.pressed {
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
            0.0,
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

        if interaction.hovered {
            ui.set_cursor_icon(CursorIcon::Pointer);
            if let Some(tooltip) = &self.tooltip {
                ui.show_tooltip(tooltip.clone());
            }
        }

        interaction
    }
}

impl StatefulWidget for Button {
    type State = ButtonState;

    fn state_id(&self) -> &str {
        &self.label
    }

    fn initial_state(&self) -> ButtonState {
        ButtonState::default()
    }
}
