use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::{MeasurablePath, Path};
use crate::interaction::Interaction;
use crate::misc::scale_rect;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Debug, Clone, Copy)]
pub struct CheckboxState {
    pub checked: bool,
    // 0 unchecked 1 fully checked, drives checkmark draw and box fill
    pub anim_progress: f32,
    pub hover_t: f32,
    // 0 baseline 1 fully popped, scale pop spring on check
    pub pop_t: f32,
    pub initialized: bool,
}

impl Default for CheckboxState {
    fn default() -> Self {
        CheckboxState {
            checked: false,
            anim_progress: 0.0,
            hover_t: 0.0,
            pop_t: 0.0,
            initialized: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CheckboxResponse {
    pub checked: bool,
    pub clicked: bool,
    pub hovered: bool,
}

#[derive(Clone)]
pub struct CheckboxStyle {
    pub fill: Stateful<Fill>,
    pub check_color: Stateful<Color>,
    pub check_thickness: Stateful<f32>,
    pub check_scale: Stateful<f32>,
    pub border_width: Stateful<f32>,
    pub border_color: Stateful<Color>,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub size: Stateful<[f32; 2]>,
    pub pop_scale: Stateful<f32>,
    pub check_motion: f32,
    pub hover_motion: f32,
    pub pop_motion: f32,
    pub cursor: Stateful<CursorIcon>,
}

impl CheckboxStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        let idle_fill = if theme.is_dark {
            theme.surface
        } else {
            Color::WHITE
        };
        CheckboxStyle {
            fill: Stateful::new(Fill::Solid(idle_fill))
                .hovered(Fill::Solid(theme.surface_subtle))
                .selected(Fill::Solid(theme.active)),
            check_color: Stateful::new(theme.on_active()),
            check_thickness: Stateful::new(2.0),
            check_scale: Stateful::new(1.0),
            border_width: Stateful::new(1.0),
            border_color: Stateful::new(theme.border_strong)
                .selected(theme.border_strong.lerp(theme.active, 0.5)),
            shadow: Stateful::new(Some(theme.shadow_sm()[0])),
            sharp: false,
            path: Stateful::new(Path::rect([4.0; 4])),
            size: Stateful::new([18.0, 18.0]),
            pop_scale: Stateful::new(0.08),
            check_motion: Motion::FLUID,
            hover_motion: Motion::SNAPPY,
            pop_motion: Motion::SNAPPY,
            cursor: Stateful::new(CursorIcon::Pointer),
        }
    }

    pub fn circle(theme: &Theme) -> Self {
        CheckboxStyle {
            path: Stateful::new(Path::ellipse(0.0)),
            ..Self::from_theme(theme)
        }
    }
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        CheckboxStyle::from_theme(&Theme::default())
    }
}

pub struct Checkbox {
    id: String,
    interaction: Interaction,
    style: Option<CheckboxStyle>,
    overrides: StyleOverrides,
    default_checked: bool,
    width: Option<f32>,
    height: Option<f32>,
}

impl Checkbox {
    pub fn new() -> Self {
        Checkbox {
            id: generate_uuid(),
            interaction: Interaction::default(),
            style: None,
            overrides: StyleOverrides::default(),
            default_checked: false,
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

    pub fn style(mut self, style: CheckboxStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<CheckboxStyle>) {
        self.style = style;
    }

    fn resolved_style(&self, theme: &Theme) -> CheckboxStyle {
        let mut style = self
            .style
            .clone()
            .unwrap_or_else(|| CheckboxStyle::from_theme(theme));
        self.overrides.apply(
            &mut style.fill,
            &mut style.border_color,
            &mut style.border_width,
            &mut style.shadow,
        );
        style
    }

    pub fn default_checked(mut self, checked: bool) -> Self {
        self.default_checked = checked;
        self
    }

    pub fn clicked(&self) -> bool {
        self.interaction.clicked
    }
    pub fn hovered(&self) -> bool {
        self.interaction.hovered
    }

    pub fn is_checked(&self, ui: &mut Ui) -> bool {
        self.state(ui).checked
    }
    pub fn check(&mut self, ui: &mut Ui, value: bool) {
        let mut state = self.take_state(ui);
        state.checked = value;
        self.put_state(ui, state);
    }
}

style_overrides!(Checkbox);

impl Widget for Checkbox {
    type Output = CheckboxResponse;

    fn ui(&mut self, ui: &mut Ui) -> CheckboxResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

// two strokes draw in sequence, left arm then right arm, t goes 0 to 1
#[allow(clippy::too_many_arguments)]
fn draw_checkmark(
    cx: f32,
    cy: f32,
    t: f32,
    color: Color,
    size: f32,
    thickness: f32,
    check_scale: f32,
    ui: &mut Ui,
) {
    let box_scale = size / 18.0; // normalized to an 18px box
    let scale = box_scale * check_scale;
    let alpha = t.clamp(0.0, 1.0);
    let c = color.with_alpha(color.a * alpha);
    let stroke_w = thickness * box_scale;

    let left = [cx - 4.0 * scale, cy + 0.5 * scale];
    let valley = [cx - 1.2 * scale, cy + 3.2 * scale];
    let right = [cx + 4.6 * scale, cy - 3.4 * scale];

    let left_t_raw = (t / 0.42).clamp(0.0, 1.0);
    let left_t = ease_out_cubic(left_t_raw);
    if left_t > 0.01 {
        let end = lerp2(left, valley, left_t);
        draw_stroke(left, end, stroke_w, c, ui);
    }

    if t > 0.32 {
        let right_t_raw = ((t - 0.32) / 0.68).clamp(0.0, 1.0);
        let right_t = ease_out_cubic(right_t_raw);
        let end = lerp2(valley, right, right_t);
        draw_stroke(valley, end, stroke_w, c, ui);
    }
}

#[inline]
fn lerp2(a: [f32; 2], b: [f32; 2], t: f32) -> [f32; 2] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]
}

#[inline]
fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

fn draw_stroke(a: [f32; 2], b: [f32; 2], width: f32, color: Color, ui: &mut Ui) {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let length = (dx * dx + dy * dy).sqrt();
    if length < 0.2 {
        return;
    }
    let angle = dy.atan2(dx);
    let center = [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0];
    let position = [center[0] - length / 2.0, center[1] - width / 2.0];

    ui.draw_shape(
        MeasurablePath::rect(position, [length, width], [width / 2.0; 4]),
        Fill::Solid(color),
        0.0,
        Color::TRANSPARENT,
        0.0,
        false,
        angle,
    );
}

impl Measurable for Checkbox {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let style = self.resolved_style(ui.theme());
        let previous = *ui.widget_state_or(&self.id, self.initial_state());
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

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> CheckboxResponse {
        self.check_id();
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();

        let previous = *ui.widget_state_or(&self.id, self.initial_state());
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
            ui.set_cursor_icon(
                style.cursor.resolve(
                    StateWeights::IDLE
                        .hovered(1.0)
                        .pressed(StateWeights::when(interaction.pressed))
                        .selected(StateWeights::when(previous.checked)),
                ),
            );
        }

        let state = ui.widget_state_or(&self.id, self.initial_state());

        if !state.initialized {
            state.anim_progress = if state.checked { 1.0 } else { 0.0 };
            state.initialized = true;
        }

        if interaction.clicked {
            state.checked = !state.checked;
            state.pop_t = 1.0;
        }

        let checked = state.checked;

        let target_anim = if checked { 1.0 } else { 0.0 };
        state.anim_progress =
            animate_towards(state.anim_progress, target_anim, dt, style.check_motion);

        let hover_target = if interaction.hovered { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, style.hover_motion);

        state.pop_t = animate_towards(state.pop_t, 0.0, dt, style.pop_motion);

        let anim_t = state.anim_progress;
        let hover_t = state.hover_t;
        let pop_t = state.pop_t;

        // inflates the draw rect from its center by pop_scale at peak pop
        let pop_scale = style
            .pop_scale
            .resolve(StateWeights::IDLE.hovered(hover_t).selected(anim_t));
        let (draw_pos, draw_size) = scale_rect(position, size, 1.0 + pop_t * pop_scale);

        let weights = StateWeights::IDLE.hovered(hover_t).selected(anim_t);
        let path = style.path.resolve(weights);

        if let Some(shadow) = &style.shadow.resolve(weights) {
            draw_shadow(shadow, draw_pos, draw_size, &path, ui);
        }
        ui.draw_shape(
            path(draw_pos, draw_size),
            style.fill.resolve(weights),
            style.border_width.resolve(weights),
            style.border_color.resolve(weights),
            0.0,
            style.sharp,
            0.0,
        );

        if anim_t > 0.01 {
            let cx = draw_pos[0] + draw_size[0] * 0.5;
            let cy = draw_pos[1] + draw_size[1] * 0.5;
            draw_checkmark(
                cx,
                cy,
                anim_t,
                style.check_color.resolve(weights),
                draw_size[0].min(draw_size[1]),
                style.check_thickness.resolve(weights),
                style.check_scale.resolve(weights),
                ui,
            );
        }

        CheckboxResponse {
            checked,
            clicked: interaction.clicked,
            hovered: interaction.hovered,
        }
    }
}

impl StatefulWidget for Checkbox {
    type State = CheckboxState;

    fn state_id(&self) -> &str {
        &self.id
    }

    fn initial_state(&self) -> CheckboxState {
        CheckboxState {
            checked: self.default_checked,
            anim_progress: if self.default_checked { 1.0 } else { 0.0 },
            hover_t: 0.0,
            pop_t: 0.0,
            initialized: true,
        }
    }
}

impl Accessible for Checkbox {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::CheckBox
    }
}
