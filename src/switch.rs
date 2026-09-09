use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Default)]
pub struct SwitchState {
    pub enabled: bool,
    pub anim_progress: f32,
    pub hover_t: f32,
    pub initialized: bool,
    pub prev_progress: f32,
    /// Glow halo intensity (0 = none, 1 = full). Pulses in when enabled.
    pub glow_t: f32,
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
    width: Option<f32>,
    height: Option<f32>,
}

impl Switch {
    pub fn new(id: impl Into<String>) -> Self {
        Switch {
            id: id.into(),
            style: None,
            interaction: Interaction::default(),
            default_enabled: false,
            width: None,
            height: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = Some(size[0]);
        self.height = Some(size[1]);
        self
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
        [self.width.unwrap_or(40.0), self.height.unwrap_or(22.0)]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> SwitchResponse {
        let theme = *ui.theme();
        let style = self.style.clone().unwrap_or_else(|| theme.switch_style());
        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        if interaction.hovered {
            ui.set_cursor_icon(CursorIcon::Pointer);
        }

        let dt = ui.dt();
        let state = ui.widget_state_or::<SwitchState>(&self.id, self.initial_state());

        if !state.initialized {
            state.anim_progress = if state.enabled { 1.0 } else { 0.0 };
            state.initialized = true;
            state.prev_progress = state.anim_progress;
            state.glow_t = state.anim_progress;
        }

        if interaction.clicked {
            state.enabled = !state.enabled;
        }

        let enabled = state.enabled;
        let target_progress = if enabled { 1.0 } else { 0.0 };

        // Knob flip uses SNAPPY (45ms half-life) — crisp, physical, no overshoot.
        state.anim_progress =
            animate_towards(state.anim_progress, target_progress, dt, Motion::SNAPPY);

        let hover_target = if interaction.hovered { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, Motion::SNAPPY);

        // Glow fades in when enabled (FLUID = 60ms, intentionally slower than
        // the knob so the glow "blooms" behind the movement).
        let glow_target = if enabled { 1.0f32 } else { 0.0 };
        state.glow_t = animate_towards(state.glow_t, glow_target, dt, Motion::FLUID);

        let progress = state.anim_progress;
        let velocity = (progress - state.prev_progress) / dt.max(0.001);
        state.prev_progress = progress;

        let hover_t = state.hover_t;
        let glow_t = state.glow_t;

        // Extract the "on" color for the glow halo before track_fill consumes it
        let on_color = if let Fill::Solid(c) = &style.track_on_fill {
            *c
        } else {
            theme.active
        };

        // Track color cross-fade
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

        let padding = 2.0;
        let knob_size = size[1] - padding * 2.0;
        let min_x = position[0] + padding;
        let max_x = position[0] + size[0] - knob_size - padding;
        let knob_x = min_x + (max_x - min_x) * progress;
        let knob_pos = [knob_x, position[1] + padding];
        let knob_radius = knob_size / 2.0;

        // Motion-blur stretch trail: widens in the direction of travel
        let speed = velocity.abs();
        if speed > 8.0 {
            let stretch_amount = (speed * 0.010).min(10.0);
            let stretch_w = knob_size + stretch_amount;
            let stretch_x = if velocity > 0.0 {
                knob_x - stretch_amount
            } else {
                knob_x
            };
            ui.draw_rect(
                [stretch_x, position[1] + padding],
                [stretch_w, knob_size],
                Fill::Solid(Color::WHITE.with_alpha(0.22)),
                knob_radius,
                0.0,
                Color::TRANSPARENT,
                3.0,
                false,
                0.0,
            );
        }

        // Active glow halo behind the knob (only visible when enabled)
        if glow_t > 0.01 {
            let halo_size = knob_size + 10.0 * glow_t;
            let halo_offset = (halo_size - knob_size) / 2.0;
            ui.draw_rect(
                [knob_x - halo_offset, position[1] + padding - halo_offset],
                [halo_size, halo_size],
                Fill::Solid(on_color.with_alpha(0.30 * glow_t)),
                halo_size / 2.0,
                0.0,
                Color::TRANSPARENT,
                4.0,
                false,
                0.0,
            );
        }

        // Thumb drop-shadow
        let thumb_shadow = ShadowStyle {
            color: Color::rgba(0, 0, 0, 0.28),
            blur_radius: 3.0,
            offset: [0.0, 1.0],
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
            glow_t: if self.default_enabled { 1.0 } else { 0.0 },
        }
    }
}

impl Accessible for Switch {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::Switch
    }
}
