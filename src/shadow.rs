use crate::{Color, Fill, Ui};

#[derive(Debug, Clone, Copy)]
pub struct ShadowStyle {
    pub color: Color,
    pub blur_radius: f32,
    pub offset: [f32; 2],
}

impl Default for ShadowStyle {
    fn default() -> Self {
        ShadowStyle {
            color: crate::theme::Theme::SHADOW,
            blur_radius: 10.0,
            offset: [0.0, 0.0],
        }
    }
}

pub fn draw_shadow(
    style: &ShadowStyle,
    position: [f32; 2],
    size: [f32; 2],
    corner_radius: f32,
    ui: &mut Ui,
) {
    let shadow_position = [position[0] + style.offset[0], position[1] + style.offset[1]];
    ui.draw_rect(
        shadow_position,
        size,
        Fill::Solid(style.color),
        corner_radius,
        0.0,
        Color::TRANSPARENT,
        style.blur_radius,
        false,
        0.0,
    );
}
