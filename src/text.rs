use crate::color::Color;
use glyphon::{Family, Weight};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontFamily {
    Default,
    Mono,
    Custom(&'static str),
}

impl FontFamily {
    pub fn to_glyphon(self) -> Family<'static> {
        match self {
            FontFamily::Default => Family::Name("Geist"),
            FontFamily::Mono => Family::Name("Geist Mono"),
            FontFamily::Custom(name) => Family::Name(name),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum FontWeight {
    #[default]
    Regular,
    Medium,
    SemiBold,
    Bold,
}

impl FontWeight {
    pub fn to_glyphon(self) -> Weight {
        match self {
            FontWeight::Regular => Weight::NORMAL,
            FontWeight::Medium => Weight::MEDIUM,
            FontWeight::SemiBold => Weight::SEMIBOLD,
            FontWeight::Bold => Weight::BOLD,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStyle {
    pub font_family: FontFamily,
    pub color: Option<Color>,
    pub font_size: f32,
    pub line_height: f32,
    pub weight: FontWeight,
}

impl TextStyle {
    pub const fn new() -> Self {
        Self {
            font_family: FontFamily::Default,
            color: None,
            font_size: 14.0,
            line_height: 20.0,
            weight: FontWeight::Regular,
        }
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn or_color(mut self, color: Color) -> Self {
        self.color = Some(self.color.unwrap_or(color));
        self
    }
    pub fn size(mut self, font_size: f32, line_height: f32) -> Self {
        self.font_size = font_size;
        self.line_height = line_height;
        self
    }
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }
    pub fn mono(mut self) -> Self {
        self.font_family = FontFamily::Mono;
        self
    }
    pub fn family(mut self, font_family: FontFamily) -> Self {
        self.font_family = font_family;
        self
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum FontError {
    Io(std::io::Error),
    Invalid,
}
