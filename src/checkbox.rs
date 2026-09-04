use crate::animation::animate_towards;
use crate::color::Color;
use crate::fill::Fill;
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, StatefulWidget, Widget};
use winit::window::CursorIcon;

pub struct CheckboxState {
    pub checked: bool,
    pub anim_progress: f32,
    pub initialized: bool,
}

impl Default for CheckboxState {
    fn default() -> Self {
        CheckboxState {
            checked: false,
            anim_progress: 0.0,
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
}

impl Checkbox {
    pub fn new(id: impl Into<String>) -> Self {
        Checkbox {
            id: id.into(),
            interaction: Interaction::default(),
            style: None,
            default_checked: false,
        }
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

/// Draw a crisp diagonal checkmark using sub-pixel-aligned axis-rects that
/// optically trace the two strokes of a tick:
///
///   Stroke A (short, bottom-left descend):  goes down-left, 2 px wide
///   Stroke B (long, top-right ascend):       goes up-right,  2 px wide
///
/// Each stroke is approximated by stacking 1-pixel-tall slices offset by 1px
/// each step to create the 45-degree diagonal appearance on a pixel grid.
fn draw_checkmark(cx: f32, cy: f32, t: f32, color: Color, ui: &mut Ui) {
    // Tick geometry tuned for an 18x18 checkbox, centered at (cx, cy).
    // The tick bottom-valley sits slightly below center.
    // All values in logical pixels.
    //
    // Left leg: descends from top-left toward the valley
    //   Rightward offset += 1 per row step to make it look diagonal
    // Right leg: ascends from valley toward top-right
    //
    // We approximate diagonals with fine horizontal slices (h=1.8px each)
    // shifted by 1.4px per step for a ~36-degree left leg and ~57-degree right leg.

    let stroke_w = 2.2_f32;
    let alpha = t.clamp(0.0, 1.0);
    let c = color.with_alpha(color.a * alpha);

    // Valley position (the bottom of the V)
    let vx = cx - 1.2;
    let vy = cy + 2.2;

    // Left short leg — 3 segments, descending right to valley
    let step_h = 1.8_f32;
    let step_dx = 1.3_f32; // horizontal shift per step (gives ~36deg)

    // Progress gates: left leg draws during first 40%, right leg during remaining 60%
    let left_segs = 3_usize;
    let right_segs = 5_usize;
    let left_full = 0.4_f32;

    // Draw left leg
    for i in 0..left_segs {
        let seg_start = (i as f32) / (left_segs as f32) * left_full;
        let seg_alpha = ((t - seg_start) / (left_full / left_segs as f32)).clamp(0.0, 1.0);
        if seg_alpha <= 0.0 {
            continue;
        }
        let sx = vx - (left_segs - i) as f32 * step_dx + step_dx;
        let sy = vy - (left_segs - i) as f32 * step_h + step_h;
        let sc = c.with_alpha(c.a * seg_alpha);
        ui.draw_rect(
            [sx, sy],
            [stroke_w, step_h],
            Fill::Solid(sc),
            0.8,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
        );
    }

    // Draw right leg — longer, ascending left to top-right
    let right_step_h = 1.8_f32;
    let right_step_dx = 1.6_f32;
    let right_start_t = left_full;

    for i in 0..right_segs {
        let seg_start = right_start_t + (i as f32) / (right_segs as f32) * (1.0 - right_start_t);
        let seg_alpha =
            ((t - seg_start) / ((1.0 - right_start_t) / right_segs as f32)).clamp(0.0, 1.0);
        if seg_alpha <= 0.0 {
            continue;
        }
        let sx = vx + i as f32 * right_step_dx;
        let sy = vy - i as f32 * right_step_h;
        let sc = c.with_alpha(c.a * seg_alpha);
        ui.draw_rect(
            [sx, sy],
            [stroke_w, right_step_h],
            Fill::Solid(sc),
            0.8,
            0.0,
            Color::TRANSPARENT,
            0.0,
            false,
        );
    }
}

impl Measurable for Checkbox {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [18.0, 18.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> CheckboxResponse {
        let style = self.style.clone().unwrap_or_default();
        let dt = ui.dt();

        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

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
        }
        let checked = state.checked;
        let target_anim = if checked { 1.0 } else { 0.0 };
        state.anim_progress = animate_towards(state.anim_progress, target_anim, dt, 0.055);
        let anim_t = state.anim_progress;

        // Blend background fill smoothly
        let fill = if anim_t > 0.01 {
            if let (Fill::Solid(idle_col), Fill::Solid(chk_col)) =
                (&style.fill, &style.checked_fill)
            {
                Fill::Solid(idle_col.lerp(*chk_col, anim_t))
            } else if checked {
                style.checked_fill
            } else {
                style.fill
            }
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
            fill,
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            style.sharp,
        );

        // Draw animated checkmark when partially or fully checked
        if anim_t > 0.01 {
            let cx = position[0] + size[0] * 0.5;
            let cy = position[1] + size[1] * 0.5;
            draw_checkmark(cx, cy, anim_t, style.check_color, ui);
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
            initialized: true,
        }
    }
}
