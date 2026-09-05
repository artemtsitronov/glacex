use crate::{Color, Fill, Ui};

/// Soft drop shadow configuration for a single shadow layer.
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

/// Named shadow levels matching Apple/Vercel/Linear depth semantics.
/// Each level pairs an ambient (wide, soft) layer with a key (tight, crisp) layer.
pub struct Shadow;

impl Shadow {
    /// Hairline: no shadow. Use for flat controls flush with the surface.
    pub const NONE: ShadowStyle = ShadowStyle {
        color: Color::TRANSPARENT,
        blur_radius: 0.0,
        offset: [0.0, 0.0],
    };

    /// Level 1: resting control lift. Tight key shadow only.
    /// Use for buttons, checkboxes, sliders in idle state.
    pub fn sm() -> [ShadowStyle; 2] {
        [
            ShadowStyle {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.12,
                },
                blur_radius: 3.0,
                offset: [0.0, 1.0],
            },
            ShadowStyle {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.24,
                },
                blur_radius: 6.0,
                offset: [0.0, 2.0],
            },
        ]
    }

    /// Level 2: card / panel elevation.
    /// Wider ambient + crisper key gives perceivable depth without drama.
    pub fn md() -> [ShadowStyle; 2] {
        [
            ShadowStyle {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.20,
                },
                blur_radius: 12.0,
                offset: [0.0, 3.0],
            },
            ShadowStyle {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.32,
                },
                blur_radius: 4.0,
                offset: [0.0, 2.0],
            },
        ]
    }

    /// Level 3: elevated popover / tooltip. Deeper, more dramatic lift.
    pub fn lg() -> [ShadowStyle; 2] {
        [
            ShadowStyle {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.28,
                },
                blur_radius: 24.0,
                offset: [0.0, 6.0],
            },
            ShadowStyle {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.36,
                },
                blur_radius: 8.0,
                offset: [0.0, 3.0],
            },
        ]
    }
}

/// Draws a single shadow layer quad behind a surface.
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

/// Draws a two-layer shadow (ambient + key) for richer, more natural depth.
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
