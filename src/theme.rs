use crate::color::Color;

/// Central palette inspired by refined, modern dark aesthetics (Linear, Vercel, Raycast).
/// Clean slate/zinc grays, high contrast borders, premium accents, and subtle depths.
pub struct Theme;

impl Theme {
    // Backgrounds & Surfaces
    pub const BG_CANVAS: Color = Color {
        r: 9.0 / 255.0,
        g: 9.0 / 255.0,
        b: 11.0 / 255.0, // #09090b
        a: 1.0,
    };

    pub const SURFACE: Color = Color {
        r: 18.0 / 255.0,
        g: 18.0 / 255.0,
        b: 22.0 / 255.0, // #121216
        a: 1.0,
    };

    pub const SURFACE_SUBTLE: Color = Color {
        r: 24.0 / 255.0,
        g: 24.0 / 255.0,
        b: 27.0 / 255.0, // #18181b
        a: 1.0,
    };

    pub const SURFACE_ELEVATED: Color = Color {
        r: 32.0 / 255.0,
        g: 32.0 / 255.0,
        b: 38.0 / 255.0, // #202026
        a: 1.0,
    };

    // Interactive States (Buttons, Controls)
    pub const IDLE: Color = Color {
        r: 28.0 / 255.0,
        g: 28.0 / 255.0,
        b: 33.0 / 255.0, // #1c1c21
        a: 1.0,
    };

    pub const HOVERED: Color = Color {
        r: 39.0 / 255.0,
        g: 39.0 / 255.0,
        b: 45.0 / 255.0, // #27272d
        a: 1.0,
    };

    pub const ACTIVE: Color = Color {
        r: 79.0 / 255.0,
        g: 70.0 / 255.0,
        b: 229.0 / 255.0, // Indigo 600 - crisp linear primary
        a: 1.0,
    };

    pub const ACTIVE_HOVER: Color = Color {
        r: 99.0 / 255.0,
        g: 102.0 / 255.0,
        b: 241.0 / 255.0, // Indigo 500
        a: 1.0,
    };

    // Borders
    pub const BORDER: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.08, // Subtle hairline border
    };

    pub const BORDER_STRONG: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.16,
    };

    pub const FOCUS_BORDER: Color = Color {
        r: 99.0 / 255.0,
        g: 102.0 / 255.0,
        b: 241.0 / 255.0, // Vibrant electric indigo
        a: 0.9,
    };

    // Text & Foreground
    pub const TEXT_PRIMARY: Color = Color {
        r: 250.0 / 255.0,
        g: 250.0 / 255.0,
        b: 250.0 / 255.0,
        a: 1.0,
    };

    pub const TEXT_MUTED: Color = Color {
        r: 161.0 / 255.0,
        g: 161.0 / 255.0,
        b: 170.0 / 255.0, // Zinc 400
        a: 1.0,
    };

    pub const TEXT_DIM: Color = Color {
        r: 113.0 / 255.0,
        g: 113.0 / 255.0,
        b: 122.0 / 255.0, // Zinc 500
        a: 1.0,
    };

    // Accents & Badges
    pub const SUCCESS: Color = Color {
        r: 34.0 / 255.0,
        g: 197.0 / 255.0,
        b: 94.0 / 255.0, // Emerald 500
        a: 1.0,
    };

    pub const WARNING: Color = Color {
        r: 245.0 / 255.0,
        g: 158.0 / 255.0,
        b: 11.0 / 255.0, // Amber 500
        a: 1.0,
    };

    pub const ERROR: Color = Color {
        r: 239.0 / 255.0,
        g: 68.0 / 255.0,
        b: 68.0 / 255.0, // Rose/Red 500
        a: 1.0,
    };

    // Shadows
    pub const SHADOW: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.45,
    };

    pub const SURFACE_SHADOW: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.35,
    };

    // Selection
    pub const SELECTION: Color = Color {
        r: 99.0 / 255.0,
        g: 102.0 / 255.0,
        b: 241.0 / 255.0,
        a: 0.35,
    };

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
