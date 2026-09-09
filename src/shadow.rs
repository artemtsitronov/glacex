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
            color: crate::theme::Theme::SHADOW_KEY,
            blur_radius: 8.0,
            offset: [0.0, 2.0],
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
    if style.color.a < 0.001 {
        return;
    }
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

pub fn draw_shadow_layers(
    layers: &[ShadowStyle; 2],
    position: [f32; 2],
    size: [f32; 2],
    corner_radius: f32,
    ui: &mut Ui,
) {
    draw_shadow(&layers[0], position, size, corner_radius, ui);
    draw_shadow(&layers[1], position, size, corner_radius, ui);
}
