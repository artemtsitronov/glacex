use crate::color::Color;

/// Central palette. Widgets read from here instead of hardcoding
/// `Color::rgb(...)` values directly, so the whole app's look changes from
/// one place.
pub struct Theme;

impl Theme {
    pub const IDLE: Color = Color {
        r: 42.0 / 255.0,
        g: 42.0 / 255.0,
        b: 46.0 / 255.0,
        a: 1.0,
    };
    pub const HOVERED: Color = Color {
        r: 56.0 / 255.0,
        g: 56.0 / 255.0,
        b: 63.0 / 255.0,
        a: 1.0,
    };
    pub const ACTIVE: Color = Color {
        r: 76.0 / 255.0,
        g: 95.0 / 255.0,
        b: 213.0 / 255.0,
        a: 1.0,
    };
    pub const BORDER: [f32; 4] = [0.0, 0.0, 0.0, 0.1];
    pub const SHADOW: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.35,
    };

    /// The standard idle/hovered/active color rule, shared by Button and
    /// Checkbox. `active` means pressed for Button, checked for Checkbox —
    /// whichever "on" state that widget has.
    pub fn state_color(active: bool, hovered: bool) -> Color {
        if active {
            Self::ACTIVE
        } else if hovered {
            Self::HOVERED
        } else {
            Self::IDLE
        }
    }
}
