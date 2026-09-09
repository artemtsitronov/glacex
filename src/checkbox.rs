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

#[derive(Debug, Clone, Copy)]
pub struct CheckboxState {
    pub checked: bool,
    /// 0 = unchecked, 1 = fully checked (drives checkmark draw and box fill).
    pub anim_progress: f32,
    /// 0 = resting, 1 = hovering.
    pub hover_t: f32,
    /// 0 = baseline, 1 = fully popped (used for scale-pop spring on check).
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

#[derive(Debug, Clone)]
pub struct CheckboxStyle {
    pub fill: Fill,
    pub hover_fill: Fill,
    pub checked_fill: Fill,
    pub check_color: Color,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
    pub shadow: Option<ShadowStyle>,
    pub sharp: bool,
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        CheckboxStyle {
            fill: Fill::Solid(Theme::IDLE),
            hover_fill: Fill::Solid(Theme::HOVERED),
            checked_fill: Fill::Solid(Theme::ACTIVE),
            check_color: Color::WHITE,
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: 5.0,
            shadow: Some(ShadowStyle::default()),
            sharp: false,
        }
    }
}

pub struct Checkbox {
    id: String,
    interaction: Interaction,
    style: Option<CheckboxStyle>,
    default_checked: bool,
    width: Option<f32>,
    height: Option<f32>,
}

impl Checkbox {
    pub fn new(id: impl Into<String>) -> Self {
        Checkbox {
            id: id.into(),
            interaction: Interaction::default(),
            style: None,
            default_checked: false,
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

    pub fn style(mut self, style: CheckboxStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<CheckboxStyle>) {
        self.style = style;
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
}

impl Widget for Checkbox {
    type Output = CheckboxResponse;

    fn ui(&mut self, ui: &mut Ui) -> CheckboxResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

/// Draws the animated checkmark as two strokes that "draw on" in sequence.
/// `t` goes 0 → 1: left arm draws first (0..0.40), right arm follows (0.30..1.0).
/// Using a cubic ease on each arm gives a natural pen-stroke deceleration.
fn draw_checkmark(cx: f32, cy: f32, t: f32, color: Color, size: f32, ui: &mut Ui) {
    let scale = size / 18.0; // normalize to the default 18px box
    let alpha = t.clamp(0.0, 1.0);
    let c = color.with_alpha(color.a * alpha);
    let stroke_w = 2.0 * scale;

    // Anchor points (designed for a 18px box centered at cx, cy)
    let left = [cx - 4.0 * scale, cy + 0.5 * scale];
    let valley = [cx - 1.2 * scale, cy + 3.2 * scale];
    let right = [cx + 4.6 * scale, cy - 3.4 * scale];

    // Left arm: t ∈ 0..0.42 — ease-out-cubic so it starts fast, decelerates
    let left_t_raw = (t / 0.42).clamp(0.0, 1.0);
    let left_t = ease_out_cubic(left_t_raw);
    if left_t > 0.01 {
        let end = lerp2(left, valley, left_t);
        draw_stroke(left, end, stroke_w, c, ui);
    }

    // Right arm: t ∈ 0.32..1.0 — same curve
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

    ui.draw_rect(
        position,
        [length, width],
        Fill::Solid(color),
        width / 2.0,
        0.0,
        Color::TRANSPARENT,
        0.0,
        false,
        angle,
    );
}

impl Measurable for Checkbox {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.width.unwrap_or(18.0), self.height.unwrap_or(18.0)]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> CheckboxResponse {
        let theme = *ui.theme();
        let style = self.style.clone().unwrap_or_else(|| theme.checkbox_style());
        let dt = ui.dt();

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

        let state = ui.widget_state_or(&self.id, self.initial_state());

        if !state.initialized {
            state.anim_progress = if state.checked { 1.0 } else { 0.0 };
            state.initialized = true;
        }

        if interaction.clicked {
            state.checked = !state.checked;
            // Kick the pop spring: push pop_t to 1, it will decay back
            state.pop_t = 1.0;
        }

        let checked = state.checked;

        // Check-draw animation: FLUID half-life feels like ink flowing onto paper
        let target_anim = if checked { 1.0 } else { 0.0 };
        state.anim_progress = animate_towards(state.anim_progress, target_anim, dt, Motion::FLUID);

        // Hover
        let hover_target = if interaction.hovered { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, Motion::SNAPPY);

        // Scale-pop: decays back to 0 (rest) after being kicked to 1 on click.
        // Using a very fast half-life (INSTANT) so it snaps back quickly — the
        // overshoot below is what creates the "pop" feel.
        state.pop_t = animate_towards(state.pop_t, 0.0, dt, Motion::SNAPPY);

        let anim_t = state.anim_progress;
        let hover_t = state.hover_t;
        let pop_t = state.pop_t;

        // Scale the box: +8% at peak pop, then settle back to 1.0
        // The scale is applied by inflating the draw rect from its center.
        let scale = 1.0 + pop_t * 0.08;
        let inflated_w = size[0] * scale;
        let inflated_h = size[1] * scale;
        let draw_pos = [
            position[0] - (inflated_w - size[0]) / 2.0,
            position[1] - (inflated_h - size[1]) / 2.0,
        ];
        let draw_size = [inflated_w, inflated_h];

        // Fill: idle → hover → checked, all cross-faded
        let fill = if let (Fill::Solid(idle_col), Fill::Solid(hov_col), Fill::Solid(chk_col)) =
            (&style.fill, &style.hover_fill, &style.checked_fill)
        {
            let idle_or_hover = idle_col.lerp(*hov_col, hover_t);
            Fill::Solid(idle_or_hover.lerp(*chk_col, anim_t))
        } else if checked {
            style.checked_fill
        } else {
            style.fill
        };

        // Border tightens toward active as it fills
        let border_color = if anim_t > 0.01 {
            style.border_color.lerp(theme.active, anim_t * 0.5)
        } else if hover_t > 0.01 {
            style.border_color.lerp(theme.border_strong, hover_t)
        } else {
            style.border_color
        };

        // Subtle focus glow ring when checked (half-intensity, very soft)
        if anim_t > 0.05 {
            let glow_r = draw_size[0].max(draw_size[1]) / 2.0 + 4.0 * anim_t;
            let glow_pos = [
                draw_pos[0] + draw_size[0] / 2.0 - glow_r,
                draw_pos[1] + draw_size[1] / 2.0 - glow_r,
            ];
            ui.draw_rect(
                glow_pos,
                [glow_r * 2.0, glow_r * 2.0],
                Fill::Solid(theme.active.with_alpha(0.12 * anim_t)),
                glow_r,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }

        if let Some(shadow) = &style.shadow {
            draw_shadow(shadow, draw_pos, draw_size, style.corner_radius * scale, ui);
        }
        ui.draw_rect(
            draw_pos,
            draw_size,
            fill,
            style.corner_radius * scale,
            style.border_width,
            border_color,
            0.0,
            style.sharp,
            0.0,
        );

        if anim_t > 0.01 {
            let cx = draw_pos[0] + draw_size[0] * 0.5;
            let cy = draw_pos[1] + draw_size[1] * 0.5;
            draw_checkmark(cx, cy, anim_t, style.check_color, draw_size[0], ui);
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
