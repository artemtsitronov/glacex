use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::contains;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, StatefulWidget, Widget};
use winit::window::CursorIcon;

#[derive(Clone, Default)]
pub struct SliderState {
    pub value: f32,
    pub dragging: bool,
    /// Animated hover glow on the thumb, 0.0..=1.0
    pub hover_t: f32,
    /// Animated drag scale, 0.0..=1.0
    pub drag_t: f32,
    /// Velocity of the thumb in pixels/sec for motion blur
    pub velocity_x: f32,
    /// Last thumb center X position
    pub last_x: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SliderResponse {
    pub value: f32,
    pub changed: bool,
    pub dragging: bool,
    pub hovered: bool,
}

#[derive(Debug, Clone)]
pub struct SliderStyle {
    pub track_fill: Fill,
    pub filled_fill: Fill,
    pub thumb_fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub track_height: f32,
    pub thumb_size: f32,
    pub shadow: Option<ShadowStyle>,
}

impl Default for SliderStyle {
    fn default() -> Self {
        SliderStyle {
            track_fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            filled_fill: Fill::Solid(Theme::ACTIVE),
            thumb_fill: Fill::Solid(Color::WHITE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            track_height: 6.0,
            thumb_size: 16.0,
            shadow: Some(ShadowStyle::default()),
        }
    }
}

pub struct Slider {
    id: String,
    min: f32,
    max: f32,
    width: f32,
    style: Option<SliderStyle>,
    default_value: f32,
}

impl Slider {
    pub fn new(id: impl Into<String>, min: f32, max: f32, width: f32) -> Self {
        Slider {
            id: id.into(),
            min,
            max,
            width,
            style: None,
            default_value: 0.0,
        }
    }

    pub fn style(mut self, style: SliderStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<SliderStyle>) {
        self.style = style;
    }

    pub fn default_value(mut self, default_value: f32) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn state(&self, ui: &mut Ui) -> SliderState {
        ui.widget_state_or::<SliderState>(
            &self.id,
            SliderState {
                value: self.default_value,
                ..Default::default()
            },
        )
        .clone()
    }
}

impl Widget for Slider {
    type Output = SliderResponse;

    fn ui(&mut self, ui: &mut Ui) -> SliderResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Slider {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        let style = self.style.clone().unwrap_or_default();
        [self.width, style.thumb_size.max(style.track_height) + 8.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> SliderResponse {
        let theme = *ui.theme();
        let style = self.style.clone().unwrap_or_else(|| theme.slider_style());
        let dt = ui.dt();
        let mouse_pos = ui.mouse_position();
        let mouse_pressed = ui.mouse_pressed();
        let mouse_pressed_this_frame = ui.mouse_pressed_this_frame();

        let hovered = contains(position, size, 0.0, mouse_pos)
            && !ui.is_input_blocked(mouse_pos)
            && ui.point_in_current_clip(mouse_pos);

        let mut state = ui.take_widget_state_or::<SliderState>(&self.id, self.initial_state());

        if hovered || state.dragging {
            ui.set_cursor_icon(CursorIcon::EwResize);
        }

        let mut changed = false;
        if hovered && mouse_pressed_this_frame {
            state.dragging = true;
        }

        if !mouse_pressed {
            state.dragging = false;
        }

        let range = (self.max - self.min).max(0.0001);

        if state.dragging {
            let rel_x = (mouse_pos[0] - position[0]).clamp(0.0, size[0]);
            let normalized = rel_x / size[0];
            let new_value = self.min + normalized * range;
            if (new_value - state.value).abs() > f32::EPSILON {
                state.value = new_value;
                changed = true;
            }
        } else {
            state.value = state.value.clamp(self.min, self.max);
        }

        // Animate hover glow on thumb with Motion::SNAPPY
        let hover_target = if hovered { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, Motion::SNAPPY);

        // Animate dragging scale expansion with Motion::INSTANT
        let drag_target = if state.dragging { 1.0f32 } else { 0.0 };
        state.drag_t = animate_towards(state.drag_t, drag_target, dt, Motion::INSTANT);
        let progress = ((state.value - self.min) / range).clamp(0.0, 1.0);

        // Velocity calculation for dynamic motion blur
        let target_x = position[0] + progress * size[0];
        let current_vx = if dt > 0.0001 && state.last_x > 0.0 {
            (target_x - state.last_x) / dt
        } else {
            0.0
        };
        state.velocity_x = animate_towards(state.velocity_x, current_vx, dt, Motion::SNAPPY);
        state.last_x = target_x;

        let value = state.value;
        let dragging = state.dragging;
        let hover_t = state.hover_t;
        let drag_t = state.drag_t;
        let velocity_x = state.velocity_x;
        ui.put_widget_state(&self.id, state);

        // Visual layout
        let track_y = position[1] + (size[1] - style.track_height) / 2.0;
        let track_radius = style.track_height / 2.0;

        // Background track
        ui.draw_rect(
            [position[0], track_y],
            [size[0], style.track_height],
            style.track_fill,
            track_radius,
            style.border_width,
            style.border_color,
            0.0,
            false,
            0.0,
        );

        let filled_width = progress * size[0];

        // Filled active track
        if filled_width > 0.0 {
            ui.draw_rect(
                [position[0], track_y],
                [filled_width, style.track_height],
                style.filled_fill,
                track_radius,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }

        // Tactile thumb knob with interactive drag scale & glow halo
        let thumb_base_size = style.thumb_size;
        let thumb_current_size = thumb_base_size + drag_t * 2.5;
        let thumb_center_x = target_x;
        let thumb_x = thumb_center_x - thumb_current_size / 2.0;
        let thumb_y = position[1] + (size[1] - thumb_current_size) / 2.0;
        let thumb_pos = [thumb_x, thumb_y];
        let thumb_size = [thumb_current_size, thumb_current_size];
        let thumb_radius = thumb_current_size / 2.0;

        // Motion blur trail effect when dragged quickly (Apple / Framer grade)
        let blur_trail_len = (velocity_x * 0.012).clamp(-18.0, 18.0);
        if blur_trail_len.abs() > 1.5 && dragging {
            let trail_x = if blur_trail_len > 0.0 {
                thumb_x - blur_trail_len
            } else {
                thumb_x
            };
            let trail_w = thumb_current_size + blur_trail_len.abs();
            ui.draw_rect(
                [trail_x, thumb_y],
                [trail_w, thumb_current_size],
                Fill::Solid(theme.active.with_alpha(0.18)),
                thumb_radius,
                0.0,
                Color::TRANSPARENT,
                4.0, // motion blur stretch
                false,
                0.0,
            );
        }

        // Soft glow halo on hover/drag (Stripe / Vercel grade)
        let active_halo_t = hover_t.max(drag_t);
        if active_halo_t > 0.01 {
            let glow_size = thumb_current_size + 12.0 * active_halo_t;
            let glow_offset = (glow_size - thumb_current_size) / 2.0;
            let glow_pos = [thumb_x - glow_offset, thumb_y - glow_offset];
            ui.draw_rect(
                glow_pos,
                [glow_size, glow_size],
                Fill::Solid(theme.active.with_alpha(0.24 * active_halo_t)),
                glow_size / 2.0,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }

        if let Some(shadow) = &style.shadow {
            let mut s = *shadow;
            s.blur_radius += drag_t * 4.0;
            s.offset[1] += drag_t * 1.5;
            draw_shadow(&s, thumb_pos, thumb_size, thumb_radius, ui);
        }

        ui.draw_rect(
            thumb_pos,
            thumb_size,
            style.thumb_fill,
            thumb_radius,
            1.0,
            theme.border,
            0.0,
            false,
            0.0,
        );

        SliderResponse {
            value,
            changed,
            dragging,
            hovered,
        }
    }
}

impl StatefulWidget for Slider {
    type State = SliderState;

    fn state_id(&self) -> &str {
        &self.id
    }

    fn initial_state(&self) -> SliderState {
        SliderState {
            value: self.default_value,
            ..Default::default()
        }
    }
}
