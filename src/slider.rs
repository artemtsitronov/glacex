use crate::animation::animate_towards;
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
        let style = self.style.clone().unwrap_or_default();
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
        let track_width = (size[0] - style.thumb_size).max(1.0);

        if state.dragging {
            let rel_x =
                (mouse_pos[0] - (position[0] + style.thumb_size / 2.0)).clamp(0.0, track_width);
            let normalized = rel_x / track_width;
            let new_value = self.min + normalized * range;
            if (new_value - state.value).abs() > f32::EPSILON {
                state.value = new_value;
                changed = true;
            }
        } else {
            state.value = state.value.clamp(self.min, self.max);
        }

        // Animate hover glow on thumb
        let hover_target = if hovered || state.dragging {
            1.0f32
        } else {
            0.0
        };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, 0.07);

        let value = state.value;
        let dragging = state.dragging;
        let hover_t = state.hover_t;
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

        let progress = ((value - self.min) / range).clamp(0.0, 1.0);
        let filled_width = progress * track_width + style.thumb_size / 2.0;

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

        // Thumb knob with animated hover glow ring
        let thumb_center_x = position[0] + progress * size[0];
        let thumb_x = thumb_center_x - style.thumb_size / 2.0;
        let thumb_y = position[1] + (size[1] - style.thumb_size) / 2.0;
        let thumb_pos = [thumb_x, thumb_y];
        let thumb_size = [style.thumb_size, style.thumb_size];
        let thumb_radius = style.thumb_size / 2.0;

        // Soft glow halo on hover/drag
        if hover_t > 0.01 {
            let glow_size = style.thumb_size + 8.0 * hover_t;
            let glow_offset = (glow_size - style.thumb_size) / 2.0;
            let glow_pos = [thumb_x - glow_offset, thumb_y - glow_offset];
            ui.draw_rect(
                glow_pos,
                [glow_size, glow_size],
                Fill::Solid(Theme::ACTIVE.with_alpha(0.18 * hover_t)),
                glow_size / 2.0,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }

        if let Some(shadow) = &style.shadow {
            draw_shadow(shadow, thumb_pos, thumb_size, thumb_radius, ui);
        }

        ui.draw_rect(
            thumb_pos,
            thumb_size,
            style.thumb_fill,
            thumb_radius,
            1.0,
            Theme::BORDER,
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
