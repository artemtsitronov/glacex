// one color type for styling, themes, gradients and gpu vertex data
// repr(C) + Pod so it sits in gpu structs like a [f32; 4]

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

    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }

    pub const fn linear_rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    pub const fn linear_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub const fn hsv(h: f32, s: f32, v: f32) -> Color {
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

    pub fn hex_str(s: &str) -> Color {
        let s = s.strip_prefix('#').unwrap_or(s);
        let value = u32::from_str_radix(s, 16).unwrap_or(0);
        Color::hex(value)
    }

    // 0xRRGGBB opaque
    pub const fn hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color::rgb(r, g, b)
    }

    // 0xRRGGBBAA
    pub const fn hex_alpha(hex: u32) -> Color {
        let r = ((hex >> 24) & 0xFF) as u8;
        let g = ((hex >> 16) & 0xFF) as u8;
        let b = ((hex >> 8) & 0xFF) as u8;
        let a = (hex & 0xFF) as f32 / 255.0;
        Color::rgba(r, g, b, a)
    }

    pub fn with_alpha(self, a: f32) -> Color {
        Color { a, ..self }
    }

    pub fn lerp(self, other: Color, t: f32) -> Color {
        Color {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    pub fn to_rgba_bytes(&self) -> [u8; 4] {
        [
            (self.r.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.g.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.b.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.a.clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }

    // no alpha
    pub fn to_rgb_bytes(&self) -> [u8; 3] {
        let [r, g, b, _] = self.to_rgba_bytes();
        [r, g, b]
    }

    pub fn to_hex(&self) -> String {
        let [r, g, b] = self.to_rgb_bytes();
        format!("#{r:02x}{g:02x}{b:02x}")
    }

    pub fn to_linear_rgb(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn to_linear_rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_round_trips() {
        assert_eq!(Color::hex(0xff8000).to_hex(), "#ff8000");
        assert_eq!(Color::hex_str("#00ff00"), Color::GREEN);
        assert_eq!(Color::hex_str("0000ff"), Color::BLUE);
    }

    #[test]
    fn bad_hex_str_is_black() {
        assert_eq!(Color::hex_str("nope"), Color::BLACK);
    }

    #[test]
    fn hex_alpha_reads_last_byte() {
        let c = Color::hex_alpha(0xff000080);
        assert_eq!(c.to_rgba_bytes(), [255, 0, 0, 128]);
    }

    #[test]
    fn hsv_primaries() {
        assert_eq!(Color::hsv(0.0, 1.0, 1.0), Color::RED);
        assert_eq!(Color::hsv(120.0, 1.0, 1.0), Color::GREEN);
        assert_eq!(Color::hsv(240.0, 1.0, 1.0), Color::BLUE);
        assert_eq!(Color::hsv(360.0, 1.0, 1.0), Color::RED);
    }

    #[test]
    fn lerp_halfway() {
        let mid = Color::BLACK.lerp(Color::WHITE, 0.5);
        assert_eq!(mid.to_rgb_bytes(), [128, 128, 128]);
    }

    #[test]
    fn darken_and_lighten() {
        assert_eq!(Color::WHITE.darken(1.0), Color::BLACK);
        assert_eq!(Color::BLACK.lighten(1.0), Color::WHITE);
        assert_eq!(Color::RED.with_alpha(0.2).darken(0.5).a, 0.2);
    }

    #[test]
    fn bytes_are_clamped() {
        let c = Color::linear_rgba(2.0, -1.0, 0.5, 1.0);
        assert_eq!(c.to_rgba_bytes(), [255, 0, 128, 255]);
    }
}
