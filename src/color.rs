//! A single, GPU-friendly color type used everywhere in this crate — widget
//! styling, the theme palette, gradients, and the raw vertex data the GPU
//! consumes all share this one representation instead of scattering
//! ad-hoc `[f32; 4]` arrays around.
//!
//! `Color` is `#[repr(C)]` and implements `bytemuck::Pod`/`Zeroable`, so it
//! can be embedded directly in GPU-bound structs (see [`crate::shapes::RectInstance`])
//! with the exact same memory layout as a `[f32; 4]`.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const CYAN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const MAGENTA: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    // --- constructors ---

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }

    pub fn linear_rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    pub fn linear_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn hsv(h: f32, s: f32, v: f32) -> Color {
        let c = v * s;
        let h_prime = (h % 360.0) / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
        let (r1, g1, b1) = match h_prime as u32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        let m = v - c;
        Color {
            r: r1 + m,
            g: g1 + m,
            b: b1 + m,
            a: 1.0,
        }
    }

    /// Parses a `"#rrggbb"` or `"rrggbb"` string. Invalid input falls back
    /// to opaque black rather than panicking.
    pub fn hex_str(s: &str) -> Color {
        let s = s.strip_prefix('#').unwrap_or(s);
        let value = u32::from_str_radix(s, 16).unwrap_or(0);
        Color::hex(value)
    }

    /// `0xRRGGBB`, opaque.
    pub fn hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color::rgb(r, g, b)
    }

    /// `0xRRGGBBAA`.
    pub fn hex_alpha(hex: u32) -> Color {
        let r = ((hex >> 24) & 0xFF) as u8;
        let g = ((hex >> 16) & 0xFF) as u8;
        let b = ((hex >> 8) & 0xFF) as u8;
        let a = (hex & 0xFF) as f32 / 255.0;
        Color::rgba(r, g, b, a)
    }

    // --- builders ---

    /// Returns this color with a different alpha, leaving r/g/b untouched.
    /// Handy for reusing a base color at different opacities, e.g.
    /// `Color::WHITE.with_alpha(0.3)`.
    pub fn with_alpha(self, a: f32) -> Color {
        Color { a, ..self }
    }

    /// Linearly interpolates between `self` (t=0) and `other` (t=1),
    /// component-wise including alpha. `t` is not clamped.
    pub fn lerp(self, other: Color, t: f32) -> Color {
        Color {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    // --- conversions ---

    /// 0-255 bytes, alpha included (also scaled to 0-255).
    pub fn to_rgba_bytes(&self) -> [u8; 4] {
        [
            (self.r.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.g.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.b.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.a.clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }

    /// 0-255 bytes, no alpha.
    pub fn to_rgb_bytes(&self) -> [u8; 3] {
        let [r, g, b, _] = self.to_rgba_bytes();
        [r, g, b]
    }

    pub fn to_linear_rgb(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn to_linear_rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    // --helpers//
    pub fn darken(&self, amount: f32) -> Color {
        Color {
            r: (self.r * (1.0 - amount)).max(0.0),
            g: (self.g * (1.0 - amount)).max(0.0),
            b: (self.b * (1.0 - amount)).max(0.0),
            a: self.a,
        }
    }

    pub fn lighten(&self, amount: f32) -> Color {
        Color {
            r: self.r + (1.0 - self.r) * amount,
            g: self.g + (1.0 - self.g) * amount,
            b: self.b + (1.0 - self.b) * amount,
            a: self.a,
        }
    }
}

impl Default for Color {
    /// Fully transparent black — the same as [`Color::TRANSPARENT`], and
    /// the same value `bytemuck::Zeroable::zeroed()` produces.
    fn default() -> Self {
        Color::TRANSPARENT
    }
}

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Color::linear_rgba(value[0], value[1], value[2], value[3])
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        color.to_linear_rgba()
    }
}
