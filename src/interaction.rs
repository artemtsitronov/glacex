use crate::geometry::contains;
use crate::ui::Ui;

#[derive(Debug, Clone, Copy, Default)]
pub struct Interaction {
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
}

impl Interaction {
    pub fn update(position: [f32; 2], size: [f32; 2], corner_radius: f32, ui: &mut Ui) -> Self {
        let mouse_pos = ui.mouse_position();
        let blocked = ui.is_input_blocked(mouse_pos);
        let outside_clip = !ui.point_in_current_clip(mouse_pos);
        let hovered =
            !blocked && !outside_clip && contains(position, size, corner_radius, mouse_pos);
        let pressed = hovered && ui.mouse_pressed();
        let clicked = hovered && ui.mouse_pressed_this_frame();
        Interaction {
            hovered,
            pressed,
            clicked,
        }
    }
}
