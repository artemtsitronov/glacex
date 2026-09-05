use crate::animation::{Motion, animate_towards};
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
    pub hover_t: f32,
    pub initialized: bool,
}

impl Default for CheckboxState {
    fn default() -> Self {
        CheckboxState {
            checked: false,
            anim_progress: 0.0,
            hover_t: 0.0,
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

fn draw_checkmark(cx: f32, cy: f32, t: f32, color: Color, ui: &mut Ui) {
    let alpha = t.clamp(0.0, 1.0);
    let c = color.with_alpha(color.a * alpha);
    let stroke_w = 2.2;

    let left = [cx - 4.2, cy + 0.3];
    let valley = [cx - 1.4, cy + 3.4];
    let right = [cx + 4.8, cy - 3.6];

    // Left downward short leg completes during 0.0..=0.35 progress
    let left_t = (t / 0.35).clamp(0.0, 1.0);
    if left_t > 0.01 {
        let current_end = [
            left[0] + (valley[0] - left[0]) * left_t,
            left[1] + (valley[1] - left[1]) * left_t,
        ];
        draw_stroke(left, current_end, stroke_w, c, ui);
    }

    // Right upward long leg completes during 0.30..=1.0 progress
    if t > 0.30 {
        let right_t = ((t - 0.30) / 0.70).clamp(0.0, 1.0);
        let current_end = [
            valley[0] + (right[0] - valley[0]) * right_t,
            valley[1] + (right[1] - valley[1]) * right_t,
        ];
        draw_stroke(valley, current_end, stroke_w, c, ui);
    }
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
        width / 2.0, // rounded caps
        0.0,
        Color::TRANSPARENT,
        0.0,
        false,
        angle,
    );
}

impl Measurable for Checkbox {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [18.0, 18.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> CheckboxResponse {
        let theme = *ui.theme();
        let style = self.style.clone().unwrap_or_else(|| theme.checkbox_style());
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
        // Snappy stroke write animation (45ms half-life)
        state.anim_progress = animate_towards(state.anim_progress, target_anim, dt, Motion::SNAPPY);
        let hover_target = if interaction.hovered { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, Motion::SNAPPY);

        let anim_t = state.anim_progress;
        let hover_t = state.hover_t;

        // Blend background fill smoothly
        let fill = if let (Fill::Solid(idle_col), Fill::Solid(chk_col)) =
            (&style.fill, &style.checked_fill)
        {
            let idle_or_hover = idle_col.lerp(theme.hovered, hover_t);
            Fill::Solid(idle_or_hover.lerp(*chk_col, anim_t))
        } else if checked {
            style.checked_fill
        } else {
            style.fill
        };

        // Smooth subtle border transition
        let border_color = if anim_t > 0.01 {
            style.border_color.lerp(theme.active, anim_t * 0.4)
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
            fill,
            style.corner_radius,
            style.border_width,
            border_color,
            0.0,
            style.sharp,
            0.0,
        );

        // Draw progressive animated checkmark stroke
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
            hover_t: 0.0,
            initialized: true,
        }
    }
}
