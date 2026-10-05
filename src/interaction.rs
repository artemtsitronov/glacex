use crate::animation::animate_towards;
use crate::geometry::MeasurablePath;
use crate::ui::Ui;
use crate::widget::is_anonymous;

#[derive(Debug, Clone, Copy, Default)]
pub struct Interaction {
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
}

impl Interaction {
    pub fn update(shape: &MeasurablePath, ui: &mut Ui) -> Self {
        let mouse_pos = ui.mouse_position();
        let blocked = ui.is_input_blocked(mouse_pos);
        let outside_clip = !ui.point_in_current_clip(mouse_pos);
        let hovered = !blocked && !outside_clip && shape.contains(mouse_pos);
        let pressed = hovered && ui.mouse_pressed();
        let clicked = hovered && ui.mouse_pressed_this_frame();
        Interaction {
            hovered,
            pressed,
            clicked,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HoverState {
    pub hover_t: f32,
}

pub fn animate_hover(ui: &mut Ui, id: &str, hovered: bool, motion: f32) -> f32 {
    let target = if hovered { 1.0 } else { 0.0 };
    if is_anonymous(id) {
        return target;
    }
    let dt = ui.dt();
    let state = ui.widget_state::<HoverState>(id);
    state.hover_t = animate_towards(state.hover_t, target, dt, motion);
    state.hover_t
}

pub fn stored_hover(ui: &mut Ui, id: &str) -> f32 {
    if is_anonymous(id) {
        return 0.0;
    }
    ui.widget_state::<HoverState>(id).hover_t
}
