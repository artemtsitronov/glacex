use crate::color::Color;
use crate::stateful::Stateful;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Border {
    pub color: Color,
    pub width: f32,
}

impl Border {
    pub const NONE: Border = Border {
        width: 0.0,
        color: Color::TRANSPARENT,
    };

    pub fn new(width: f32, color: Color) -> Self {
        Border { color, width }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn with_color(self, color: Color) -> Self {
        Border { color, ..self }
    }

    pub fn with_width(self, width: f32) -> Self {
        Border { width, ..self }
    }

    pub fn lerp(self, other: Border, t: f32) -> Border {
        Border {
            color: self.color.lerp(other.color, t),
            width: self.width + (other.width - self.width) * t,
        }
    }
}

impl Default for Border {
    fn default() -> Self {
        Border::NONE
    }
}

impl Stateful<Border> {
    pub fn with_color(&self, color: &Stateful<Color>) -> Self {
        self.zip(color, |border, color| border.with_color(*color))
    }

    pub fn with_width(&self, width: &Stateful<f32>) -> Self {
        self.zip(width, |border, width| border.with_width(*width))
    }
}
