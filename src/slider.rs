use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::{MeasurablePath, Path};
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Clone, Default)]
pub struct SliderState {
    pub value: f32,
    pub dragging: bool,
    pub hover_t: f32,
    pub drag_t: f32,
    pub velocity_x: f32,
    pub last_x: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SliderResponse {
    pub value: f32,
    pub changed: bool,
    pub dragging: bool,
    pub hovered: bool,
}

#[derive(Clone)]
pub struct SliderStyle {
    pub track_fill: Stateful<Fill>,
    pub filled_fill: Stateful<Fill>,
    pub thumb_fill: Stateful<Fill>,
    pub thumb_border_width: Stateful<f32>,
    pub thumb_border_color: Stateful<Color>,
    pub border_width: Stateful<f32>,
    pub border_color: Stateful<Color>,
    pub track_height: Stateful<f32>,
    pub thumb_size: Stateful<f32>,
    pub vertical_padding: Stateful<f32>,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub trail_color: Stateful<Color>,
    pub trail_stretch: Stateful<f32>,
    pub trail_max: Stateful<f32>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub filled_path: Stateful<Path>,
    pub thumb_path: Stateful<Path>,
    pub hover_motion: f32,
    pub drag_motion: f32,
    pub velocity_motion: f32,
    pub cursor: Stateful<CursorIcon>,
}

fn default_track_path() -> Path {
    Path::from_fn(|position, size| {
        MeasurablePath::rect(position, size, [size[0].min(size[1]) * 0.5; 4])
    })
}

impl SliderStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        let shadow = Some(theme.shadow_sm()[0]);
        SliderStyle {
            track_fill: Stateful::new(Fill::Solid(theme.hovered)),
            filled_fill: Stateful::new(Fill::Solid(theme.active)),
            thumb_fill: Stateful::new(Fill::Solid(Color::WHITE)),
            thumb_border_width: Stateful::new(1.0),
            thumb_border_color: Stateful::new(theme.border),
            border_width: Stateful::new(1.5),
            border_color: Stateful::new(theme.border_strong),
            track_height: Stateful::new(4.0),
            thumb_size: Stateful::new(16.0).pressed(18.5),
            vertical_padding: Stateful::new(8.0),
            shadow: Stateful::new(shadow).pressed(shadow.map(|s| {
                s.blur(s.blur_radius + 4.0)
                    .offset([s.offset[0], s.offset[1] + 1.5])
            })),
            trail_color: Stateful::new(theme.active.with_alpha(0.18)),
            trail_stretch: Stateful::new(0.012),
            trail_max: Stateful::new(18.0),
            sharp: false,
            path: Stateful::new(default_track_path()),
            filled_path: Stateful::new(default_track_path()),
            thumb_path: Stateful::new(default_track_path()),
            hover_motion: Motion::SNAPPY,
            drag_motion: Motion::INSTANT,
            velocity_motion: Motion::SNAPPY,
            cursor: Stateful::new(CursorIcon::EwResize),
        }
    }

    pub fn thick(theme: &Theme) -> Self {
        SliderStyle {
            track_height: Stateful::new(8.0),
            thumb_size: Stateful::new(20.0).pressed(22.5),
            border_width: Stateful::new(0.0),
            ..Self::from_theme(theme)
        }
    }
}

impl Default for SliderStyle {
    fn default() -> Self {
        SliderStyle::from_theme(&Theme::default())
    }
}

pub struct Slider {
    id: String,
    min: f32,
    max: f32,
    width: f32,
    height: Option<f32>,
    style: Option<SliderStyle>,
    overrides: StyleOverrides,
    default_value: f32,
}

impl Slider {
    pub const DEFAULT_WIDTH: f32 = 200.0;

    pub fn new(min: f32, max: f32) -> Self {
        Slider {
            id: generate_uuid(),
            min,
            max,
            width: Self::DEFAULT_WIDTH,
            height: None,
            style: None,
            overrides: StyleOverrides::default(),
            default_value: 0.0,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = size[0];
        self.height = Some(size[1]);
        self
    }

    pub fn style(mut self, style: SliderStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<SliderStyle>) {
        self.style = style;
    }

    fn resolved_style(&self, theme: &Theme) -> SliderStyle {
        let mut style = self
            .style
            .clone()
            .unwrap_or_else(|| SliderStyle::from_theme(theme));
        self.overrides.apply(
            &mut style.track_fill,
            &mut style.border_color,
            &mut style.border_width,
            &mut style.shadow,
        );
        style
    }

    pub fn default_value(mut self, default_value: f32) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn value(&self, ui: &mut Ui) -> f32 {
        self.state(ui).value
    }
    pub fn set_value(&mut self, ui: &mut Ui, value: f32) {
        let mut state = self.take_state(ui);
        state.value = value;
        self.put_state(ui, state);
    }
}

style_overrides!(Slider);

impl Widget for Slider {
    type Output = SliderResponse;

    fn ui(&mut self, ui: &mut Ui) -> SliderResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Slider {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let style = self.resolved_style(ui.theme());
        let thumb_max = style.thumb_size.values().copied().fold(0.0, f32::max);
        let track_max = style.track_height.values().copied().fold(0.0, f32::max);
        let padding_max = style.vertical_padding.values().copied().fold(0.0, f32::max);
        let natural_height = thumb_max.max(track_max) + padding_max;
        [self.width, self.height.unwrap_or(natural_height)]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> SliderResponse {
        self.check_id();
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();
        let mouse_pos = ui.mouse_position();
        let mouse_pressed = ui.mouse_pressed();
        let mouse_pressed_this_frame = ui.mouse_pressed_this_frame();

        let hovered = MeasurablePath::rect(position, size, [0.0; 4]).contains(mouse_pos)
            && !ui.is_input_blocked(mouse_pos)
            && ui.point_in_current_clip(mouse_pos);

        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        let mut state = ui.take_widget_state_or::<SliderState>(&self.id, self.initial_state());

        if hovered || state.dragging {
            ui.set_cursor_icon(
                style.cursor.resolve(
                    StateWeights::IDLE
                        .hovered(StateWeights::when(hovered))
                        .pressed(StateWeights::when(state.dragging)),
                ),
            );
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

        let hover_target = if hovered { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, style.hover_motion);

        let drag_target = if state.dragging { 1.0f32 } else { 0.0 };
        state.drag_t = animate_towards(state.drag_t, drag_target, dt, style.drag_motion);
        let progress = ((state.value - self.min) / range).clamp(0.0, 1.0);

        let target_x = position[0] + progress * size[0];
        let current_vx = if dt > 0.0001 && state.last_x > 0.0 {
            (target_x - state.last_x) / dt
        } else {
            0.0
        };
        state.velocity_x = animate_towards(state.velocity_x, current_vx, dt, style.velocity_motion);
        state.last_x = target_x;

        let value = state.value;
        let dragging = state.dragging;
        let hover_t = state.hover_t;
        let drag_t = state.drag_t;
        let velocity_x = state.velocity_x;
        ui.put_widget_state(&self.id, state);

        let weights = StateWeights::IDLE.hovered(hover_t).pressed(drag_t);
        let track_height = style.track_height.resolve(weights);
        let track_y = position[1] + (size[1] - track_height) / 2.0;

        ui.draw_shape(
            (style.path.resolve(weights))([position[0], track_y], [size[0], track_height]),
            style.track_fill.resolve(weights),
            style.border_width.resolve(weights),
            style.border_color.resolve(weights),
            0.0,
            style.sharp,
            0.0,
        );

        let filled_width = progress * size[0];

        if filled_width > 0.0 {
            ui.draw_shape(
                (style.filled_path.resolve(weights))(
                    [position[0], track_y],
                    [filled_width, track_height],
                ),
                style.filled_fill.resolve(weights),
                0.0,
                Color::TRANSPARENT,
                0.0,
                style.sharp,
                0.0,
            );
        }

        let thumb_path = style.thumb_path.resolve(weights);
        let thumb_current_size = style.thumb_size.resolve(weights);
        let thumb_center_x = target_x;
        let thumb_x = thumb_center_x - thumb_current_size / 2.0;
        let thumb_y = position[1] + (size[1] - thumb_current_size) / 2.0;
        let thumb_pos = [thumb_x, thumb_y];
        let thumb_size = [thumb_current_size, thumb_current_size];

        let trail_max = style.trail_max.resolve(weights);
        let blur_trail_len =
            (velocity_x * style.trail_stretch.resolve(weights)).clamp(-trail_max, trail_max);
        if blur_trail_len.abs() > 1.5 && dragging {
            let trail_x = if blur_trail_len > 0.0 {
                thumb_x - blur_trail_len
            } else {
                thumb_x
            };
            let trail_w = thumb_current_size + blur_trail_len.abs();
            ui.draw_shape(
                thumb_path([trail_x, thumb_y], [trail_w, thumb_current_size]),
                Fill::Solid(style.trail_color.resolve(weights)),
                0.0,
                Color::TRANSPARENT,
                4.0,
                false,
                0.0,
            );
        }

        if let Some(shadow) = &style.shadow.resolve(weights) {
            draw_shadow(shadow, thumb_pos, thumb_size, &thumb_path, ui);
        }

        ui.draw_shape(
            thumb_path(thumb_pos, thumb_size),
            style.thumb_fill.resolve(weights),
            style.thumb_border_width.resolve(weights),
            style.thumb_border_color.resolve(weights),
            0.0,
            style.sharp,
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

impl Accessible for Slider {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::Slider
    }
}
