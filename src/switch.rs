use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::{MeasurablePath, Path};
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
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
}

#[derive(Debug, Clone, Copy)]
pub struct SwitchResponse {
    pub enabled: bool,
    pub clicked: bool,
    pub hovered: bool,
}

#[derive(Clone)]
pub struct SwitchStyle {
    pub track_fill: Stateful<Fill>,
    pub thumb_fill: Stateful<Fill>,
    pub thumb_border_width: Stateful<f32>,
    pub thumb_border_color: Stateful<Color>,
    pub thumb_padding: Stateful<f32>,
    pub thumb_size: Stateful<Option<[f32; 2]>>,
    pub thumb_shadow: Stateful<Option<ShadowStyle>>,
    pub thumb_path: Stateful<Path>,
    pub trail_color: Stateful<Color>,
    pub trail_stretch: Stateful<f32>,
    pub trail_max: Stateful<f32>,
    pub border_width: Stateful<f32>,
    pub border_color: Stateful<Color>,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub size: Stateful<[f32; 2]>,
    pub toggle_motion: f32,
    pub hover_motion: f32,
    pub cursor: Stateful<CursorIcon>,
}

fn pill_path() -> Path {
    Path::from_fn(|position, size| {
        MeasurablePath::rect(position, size, [size[0].min(size[1]) * 0.5; 4])
    })
}

impl SwitchStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        SwitchStyle {
            track_fill: Stateful::new(Fill::Solid(theme.hovered))
                .selected(Fill::Solid(theme.active)),
            // blends toward a contrasting color near on, avoids white on white on near white themes
            thumb_fill: Stateful::new(Fill::Solid(Color::WHITE))
                .selected(Fill::Solid(theme.on_active())),
            thumb_border_width: Stateful::new(0.0),
            thumb_border_color: Stateful::new(Color::TRANSPARENT),
            thumb_padding: Stateful::new(2.0),
            thumb_size: Stateful::new(None),
            thumb_shadow: Stateful::new(Some(ShadowStyle::new(
                Color::rgba(0, 0, 0, 0.28),
                3.0,
                [0.0, 1.0],
            ))),
            thumb_path: Stateful::new(pill_path()),
            trail_color: Stateful::new(Color::WHITE.with_alpha(0.22)),
            trail_stretch: Stateful::new(0.010),
            trail_max: Stateful::new(10.0),
            border_width: Stateful::new(1.0),
            border_color: Stateful::new(theme.border)
                .hovered(theme.border_strong)
                .selected(theme.border.lerp(theme.active, 0.5)),
            shadow: Stateful::new(Some(theme.shadow_sm()[0])),
            sharp: false,
            path: Stateful::new(Path::rect([11.0; 4])),
            size: Stateful::new([40.0, 22.0]),
            toggle_motion: Motion::SNAPPY,
            hover_motion: Motion::SNAPPY,
            cursor: Stateful::new(CursorIcon::Pointer),
        }
    }

    pub fn square(theme: &Theme) -> Self {
        SwitchStyle {
            path: Stateful::new(Path::rect([theme.metrics.radius_xs; 4])),
            thumb_path: Stateful::new(Path::rect([theme.metrics.radius_xs; 4])),
            ..Self::from_theme(theme)
        }
    }
}

impl Default for SwitchStyle {
    fn default() -> Self {
        SwitchStyle::from_theme(&Theme::default())
    }
}

pub struct Switch {
    id: String,
    style: Option<SwitchStyle>,
    overrides: StyleOverrides,
    interaction: Interaction,
    default_enabled: bool,
    width: Option<f32>,
    height: Option<f32>,
}

impl Switch {
    pub fn new() -> Self {
        Switch {
            id: generate_uuid(),
            style: None,
            overrides: StyleOverrides::default(),
            interaction: Interaction::default(),
            default_enabled: false,
            width: None,
            height: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
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

    fn resolved_style(&self, theme: &Theme) -> SwitchStyle {
        let mut style = self
            .style
            .clone()
            .unwrap_or_else(|| SwitchStyle::from_theme(theme));
        self.overrides.apply(
            &mut style.track_fill,
            &mut style.border_color,
            &mut style.border_width,
            &mut style.shadow,
        );
        style
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

    pub fn enabled(&self, ui: &mut Ui) -> bool {
        self.state(ui).enabled
    }
    pub fn set_enabled(&mut self, ui: &mut Ui, enabled: bool) {
        let mut state = self.take_state(ui);
        state.enabled = enabled;
        self.put_state(ui, state);
    }
}

style_overrides!(Switch);

impl Widget for Switch {
    type Output = SwitchResponse;

    fn ui(&mut self, ui: &mut Ui) -> SwitchResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Switch {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let style = self.resolved_style(ui.theme());
        let previous = ui.widget_state_or::<SwitchState>(&self.id, self.initial_state());
        let size = style.size.resolve(
            StateWeights::IDLE
                .hovered(previous.hover_t)
                .selected(previous.anim_progress),
        );
        [
            self.width.unwrap_or(size[0]),
            self.height.unwrap_or(size[1]),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> SwitchResponse {
        self.check_id();
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let previous = ui.widget_state_or::<SwitchState>(&self.id, self.initial_state());
        let hit_path = style.path.resolve(
            StateWeights::IDLE
                .hovered(previous.hover_t)
                .selected(previous.anim_progress),
        );
        let interaction = Interaction::update(&hit_path(position, size), ui);
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
            let enabled = ui
                .widget_state_or::<SwitchState>(&self.id, self.initial_state())
                .enabled;
            ui.set_cursor_icon(
                style.cursor.resolve(
                    StateWeights::IDLE
                        .hovered(1.0)
                        .pressed(StateWeights::when(interaction.pressed))
                        .selected(StateWeights::when(enabled)),
                ),
            );
        }

        let dt = ui.dt();
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

        state.anim_progress = animate_towards(
            state.anim_progress,
            target_progress,
            dt,
            style.toggle_motion,
        );

        let hover_target = if interaction.hovered { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, style.hover_motion);

        let progress = state.anim_progress;
        let velocity = (progress - state.prev_progress) / dt.max(0.001);
        state.prev_progress = progress;

        let hover_t = state.hover_t;

        let weights = StateWeights::IDLE.hovered(hover_t).selected(progress);
        let path = style.path.resolve(weights);
        let thumb_path = style.thumb_path.resolve(weights);

        if let Some(shadow) = &style.shadow.resolve(weights) {
            draw_shadow(shadow, position, size, &path, ui);
        }

        ui.draw_shape(
            path(position, size),
            style.track_fill.resolve(weights),
            style.border_width.resolve(weights),
            style.border_color.resolve(weights),
            0.0,
            style.sharp,
            0.0,
        );

        let padding = style.thumb_padding.resolve(weights);
        let knob_default = (size[1] - padding * 2.0).max(0.0);
        let knob = style
            .thumb_size
            .resolve(weights)
            .unwrap_or([knob_default, knob_default]);
        let min_x = position[0] + padding;
        let max_x = position[0] + size[0] - knob[0] - padding;
        let knob_x = min_x + (max_x - min_x) * progress;
        let knob_y = position[1] + (size[1] - knob[1]) / 2.0;
        let knob_pos = [knob_x, knob_y];

        let speed = velocity.abs();
        let trail_stretch = style.trail_stretch.resolve(weights);
        if speed > 8.0 && trail_stretch > 0.0 {
            let stretch_amount = (speed * trail_stretch).min(style.trail_max.resolve(weights));
            let stretch_w = knob[0] + stretch_amount;
            let stretch_x = if velocity > 0.0 {
                knob_x - stretch_amount
            } else {
                knob_x
            };
            ui.draw_shape(
                thumb_path([stretch_x, knob_y], [stretch_w, knob[1]]),
                Fill::Solid(style.trail_color.resolve(weights)),
                0.0,
                Color::TRANSPARENT,
                3.0,
                false,
                0.0,
            );
        }

        if let Some(thumb_shadow) = &style.thumb_shadow.resolve(weights) {
            draw_shadow(thumb_shadow, knob_pos, knob, &thumb_path, ui);
        }

        ui.draw_shape(
            thumb_path(knob_pos, knob),
            style.thumb_fill.resolve(weights),
            style.thumb_border_width.resolve(weights),
            style.thumb_border_color.resolve(weights),
            0.0,
            style.sharp,
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

impl Accessible for Switch {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::Switch
    }
}
