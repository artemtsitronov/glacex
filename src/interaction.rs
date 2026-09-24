use crate::geometry::MeasurablePath;
use crate::ui::Ui;

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
