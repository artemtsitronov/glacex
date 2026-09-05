use crate::color::Color;
use crate::fill::Fill;
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, StatefulWidget, Widget};
use winit::window::CursorIcon;

use crate::animation::{Motion, animate_towards};

#[derive(Default)]
pub struct SwitchState {
    pub enabled: bool,
    pub anim_progress: f32,
    pub hover_t: f32,
    pub initialized: bool,
    pub prev_progress: f32,
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
        let theme = *ui.theme();
        let style = self.style.clone().unwrap_or_else(|| theme.switch_style());
        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

        if interaction.hovered {
            ui.set_cursor_icon(CursorIcon::Pointer);
        }

        let dt = ui.dt();
        let (enabled, progress, hover_t, knob_velocity) = {
            let state = ui.widget_state_or::<SwitchState>(&self.id, self.initial_state());

            if !state.initialized {
                state.anim_progress = if state.enabled { 1.0 } else { 0.0 };
                state.initialized = true;
                state.prev_progress = state.anim_progress;
            }

            if interaction.clicked {
                state.enabled = !state.enabled;
            }
            let enabled = state.enabled;
            let target_progress = if enabled { 1.0 } else { 0.0 };
            // Apple/Linear fluid switch animation curve
            state.anim_progress =
                animate_towards(state.anim_progress, target_progress, dt, Motion::FLUID);
            let hover_target = if interaction.hovered { 1.0f32 } else { 0.0 };
            state.hover_t = animate_towards(state.hover_t, hover_target, dt, Motion::SNAPPY);

            let progress = state.anim_progress;
            let velocity = (progress - state.prev_progress).abs();
            state.prev_progress = progress;

            (enabled, progress, state.hover_t, velocity)
        };

        // Smooth cross-fade between idle, hover, and active states
        let track_fill = if let (Fill::Solid(off_col), Fill::Solid(on_col)) =
            (&style.track_off_fill, &style.track_on_fill)
        {
            let idle_or_hover = off_col.lerp(theme.hovered, hover_t);
            Fill::Solid(idle_or_hover.lerp(*on_col, progress))
        } else if enabled {
            style.track_on_fill
        } else {
            style.track_off_fill
        };

        let border_color = if progress > 0.01 {
            style.border_color.lerp(theme.active, progress * 0.5)
        } else if hover_t > 0.01 {
            style.border_color.lerp(theme.border_strong, hover_t)
        } else {
            style.border_color
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
            border_color,
            0.0,
            false,
            0.0,
        );

        // Draw sliding knob / thumb with fluid glide and soft thumb shadow
        let padding = 2.0;
        let knob_size = size[1] - padding * 2.0;
        let min_x = position[0] + padding;
        let max_x = position[0] + size[0] - knob_size - padding;
        let knob_x = min_x + (max_x - min_x) * progress;
        let knob_pos = [knob_x, position[1] + padding];
        let knob_radius = knob_size / 2.0;

        // Subtle motion trail on knob during active sliding
        if knob_velocity > 0.005 {
            let stretch_w = knob_size + knob_velocity * 12.0;
            let stretch_x = if progress > 0.5 {
                knob_x - (stretch_w - knob_size)
            } else {
                knob_x
            };
            ui.draw_rect(
                [stretch_x, position[1] + padding],
                [stretch_w, knob_size],
                Fill::Solid(Color::WHITE.with_alpha(0.20)),
                knob_radius,
                0.0,
                Color::TRANSPARENT,
                3.0,
                false,
                0.0,
            );
        }

        // Micro thumb drop shadow for physical elevation
        let thumb_shadow = ShadowStyle {
            color: Color::rgba(0, 0, 0, 0.35),
            blur_radius: 4.0,
            offset: [0.0, 1.5],
        };
        draw_shadow(
            &thumb_shadow,
            knob_pos,
            [knob_size, knob_size],
            knob_radius,
            ui,
        );

        ui.draw_rect(
            knob_pos,
            [knob_size, knob_size],
            style.thumb_fill,
            knob_radius,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
            0.0,
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
            anim_progress: if self.default_enabled { 1.0 } else { 0.0 },
            hover_t: 0.0,
            initialized: true,
            prev_progress: if self.default_enabled { 1.0 } else { 0.0 },
        }
    }
}
