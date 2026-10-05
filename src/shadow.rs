use crate::{Color, Fill, Path, Theme, Ui};

// single shadow layer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowStyle {
    pub color: Color,
    pub blur_radius: f32,
    pub offset: [f32; 2],
    pub spread: f32,
}

impl ShadowStyle {
    pub const fn new(color: Color, blur_radius: f32, offset: [f32; 2]) -> Self {
        ShadowStyle {
            color,
            blur_radius,
            offset,
            spread: 0.0,
        }
    }

    pub fn from_theme(theme: &Theme) -> Self {
        ShadowStyle::new(theme.shadow_key, 8.0, [0.0, 2.0])
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn blur(mut self, blur_radius: f32) -> Self {
        self.blur_radius = blur_radius;
        self
    }

    pub fn offset(mut self, offset: [f32; 2]) -> Self {
        self.offset = offset;
        self
    }

    pub fn spread(mut self, spread: f32) -> Self {
        self.spread = spread;
        self
    }

    pub fn lerp(self, other: ShadowStyle, t: f32) -> ShadowStyle {
        let t = t.clamp(0.0, 1.0);
        let mix = |a: f32, b: f32| a + (b - a) * t;
        ShadowStyle {
            color: self.color.lerp(other.color, t),
            blur_radius: mix(self.blur_radius, other.blur_radius),
            offset: [
                mix(self.offset[0], other.offset[0]),
                mix(self.offset[1], other.offset[1]),
            ],
            spread: mix(self.spread, other.spread),
        }
    }

    pub fn lerp_opt(a: Option<ShadowStyle>, b: Option<ShadowStyle>, t: f32) -> Option<ShadowStyle> {
        match (a, b) {
            (Some(a), Some(b)) => Some(a.lerp(b, t)),
            (Some(a), None) => Some(a.lerp(a.color(a.color.with_alpha(0.0)), t)),
            (None, Some(b)) => Some(b.color(b.color.with_alpha(0.0)).lerp(b, t)),
            (None, None) => None,
        }
    }
}

impl Default for ShadowStyle {
    fn default() -> Self {
        ShadowStyle::from_theme(&Theme::default())
    }
}

// one shadow layer behind a surface
pub fn draw_shadow(
    style: &ShadowStyle,
    position: [f32; 2],
    size: [f32; 2],
    path: &Path,
    ui: &mut Ui,
) {
    if style.color.a < 0.001 {
        return;
    }
    let shadow_position = [
        position[0] + style.offset[0] - style.spread,
        position[1] + style.offset[1] - style.spread,
    ];
    let shadow_size = [
        (size[0] + style.spread * 2.0).max(0.0),
        (size[1] + style.spread * 2.0).max(0.0),
    ];
    ui.draw_shape(
        path(shadow_position, shadow_size),
        Fill::Solid(style.color),
        0.0,
        Color::TRANSPARENT,
        style.blur_radius,
        false,
        0.0,
    );
}

// ambient + key shadow
pub fn draw_shadow_layers(
    layers: &[ShadowStyle; 2],
    position: [f32; 2],
    size: [f32; 2],
    path: &Path,
    ui: &mut Ui,
) {
    draw_shadow(&layers[0], position, size, path, ui);
    draw_shadow(&layers[1], position, size, path, ui);
}
