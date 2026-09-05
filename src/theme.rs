use crate::button::ButtonStyle;
use crate::card::CardStyle;
use crate::checkbox::CheckboxStyle;
use crate::color::Color;
use crate::fill::Fill;
use crate::shadow::ShadowStyle;
use crate::slider::SliderStyle;
use crate::switch::SwitchStyle;
use crate::text_area::TextAreaStyle;
use crate::text_input::TextInputStyle;

/// Comprehensive design token palette and theme engine for Glacex.
///
/// Defaults to a pristine, luxurious White / Light theme inspired by
/// Apple and shadcn/ui. Also includes built-in classic Unixporn community
/// palettes: Dark, Catppuccin (Mocha & Latte), Tokyo Night, Gruvbox (Dark & Light),
/// Nord, and Rosé Pine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    /// Human-readable identifier for the theme.
    pub name: &'static str,
    /// Whether this theme is dark-oriented.
    pub is_dark: bool,

    // -------------------------------------------------------------------------
    // Surfaces
    // -------------------------------------------------------------------------
    /// Root window background canvas.
    pub bg_canvas: Color,
    /// Standard card / panel surface.
    pub surface: Color,
    /// Sub-panel, inset container, or track surface.
    pub surface_subtle: Color,
    /// Elevated surface for floating modals, popovers, and tooltips.
    pub surface_elevated: Color,

    // -------------------------------------------------------------------------
    // Interactive Controls
    // -------------------------------------------------------------------------
    /// Resting control background.
    pub idle: Color,
    /// Control hover background.
    pub hovered: Color,
    /// Control pressed background.
    pub pressed: Color,
    /// Primary accent / active control background.
    pub active: Color,
    /// Hover state for already-active controls.
    pub active_hover: Color,

    // -------------------------------------------------------------------------
    // Borders
    // -------------------------------------------------------------------------
    /// Ultra-subtle hairline border for internal dividers.
    pub border_faint: Color,
    /// Standard component border.
    pub border: Color,
    /// Stronger border for hover / focus outline emphasis.
    pub border_strong: Color,
    /// Primary focus ring outline color.
    pub focus_border: Color,

    // -------------------------------------------------------------------------
    // Typography
    // -------------------------------------------------------------------------
    /// Primary high-contrast text.
    pub text_primary: Color,
    /// Secondary supporting text.
    pub text_secondary: Color,
    /// Subdued placeholder or metadata text.
    pub text_muted: Color,

    // -------------------------------------------------------------------------
    // Semantic Status
    // -------------------------------------------------------------------------
    pub success: Color,
    pub warning: Color,
    pub error: Color,

    // -------------------------------------------------------------------------
    // Shadows & Selection
    // -------------------------------------------------------------------------
    pub shadow_ambient: Color,
    pub shadow_key: Color,
    pub selection: Color,
}

impl Default for Theme {
    #[inline]
    fn default() -> Self {
        Self::LIGHT
    }
}

impl Theme {
    // =========================================================================
    // Built-in Theme Presets
    // =========================================================================

    /// Pristine White / Light Theme (Default -- Apple & shadcn/ui style).
    /// Pure white canvas, delicate zinc borders, and deep charcoal primary action.
    pub const LIGHT: Theme = Theme {
        name: "shadcn-light",
        is_dark: false,
        bg_canvas: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }, // #ffffff
        surface: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }, // #ffffff
        surface_subtle: Color {
            r: 244.0 / 255.0,
            g: 244.0 / 255.0,
            b: 245.0 / 255.0,
            a: 1.0,
        }, // #f4f4f5 (Zinc 100)
        surface_elevated: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }, // #ffffff
        idle: Color {
            r: 244.0 / 255.0,
            g: 244.0 / 255.0,
            b: 245.0 / 255.0,
            a: 1.0,
        }, // #f4f4f5 (Zinc 100)
        hovered: Color {
            r: 228.0 / 255.0,
            g: 228.0 / 255.0,
            b: 231.0 / 255.0,
            a: 1.0,
        }, // #e4e4e7 (Zinc 200)
        pressed: Color {
            r: 212.0 / 255.0,
            g: 212.0 / 255.0,
            b: 216.0 / 255.0,
            a: 1.0,
        }, // #d4d4d8 (Zinc 300)
        active: Color {
            r: 24.0 / 255.0,
            g: 24.0 / 255.0,
            b: 27.0 / 255.0,
            a: 1.0,
        }, // #18181b (Zinc 900)
        active_hover: Color {
            r: 39.0 / 255.0,
            g: 39.0 / 255.0,
            b: 42.0 / 255.0,
            a: 1.0,
        }, // #27272a (Zinc 800)
        border_faint: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.04,
        },
        border: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.08,
        },
        border_strong: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.16,
        },
        focus_border: Color {
            r: 24.0 / 255.0,
            g: 24.0 / 255.0,
            b: 27.0 / 255.0,
            a: 0.85,
        },
        text_primary: Color {
            r: 9.0 / 255.0,
            g: 9.0 / 255.0,
            b: 11.0 / 255.0,
            a: 1.0,
        }, // #09090b (Zinc 950)
        text_secondary: Color {
            r: 113.0 / 255.0,
            g: 113.0 / 255.0,
            b: 122.0 / 255.0,
            a: 1.0,
        }, // #71717a (Zinc 500)
        text_muted: Color {
            r: 161.0 / 255.0,
            g: 161.0 / 255.0,
            b: 170.0 / 255.0,
            a: 1.0,
        }, // #a1a1aa (Zinc 400)
        success: Color {
            r: 22.0 / 255.0,
            g: 163.0 / 255.0,
            b: 74.0 / 255.0,
            a: 1.0,
        }, // Emerald 600
        warning: Color {
            r: 217.0 / 255.0,
            g: 119.0 / 255.0,
            b: 6.0 / 255.0,
            a: 1.0,
        }, // Amber 600
        error: Color {
            r: 225.0 / 255.0,
            g: 29.0 / 255.0,
            b: 72.0 / 255.0,
            a: 1.0,
        }, // Rose 600
        shadow_ambient: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.03,
        },
        shadow_key: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.07,
        },
        selection: Color {
            r: 24.0 / 255.0,
            g: 24.0 / 255.0,
            b: 27.0 / 255.0,
            a: 0.12,
        },
    };

    /// Refined Linear / Vercel Dark Theme.
    /// Deep zinc-charcoal with an electric indigo accent.
    pub const DARK: Theme = Theme {
        name: "shadcn-dark",
        is_dark: true,
        bg_canvas: Color {
            r: 9.0 / 255.0,
            g: 9.0 / 255.0,
            b: 11.0 / 255.0,
            a: 1.0,
        }, // #09090b
        surface: Color {
            r: 15.0 / 255.0,
            g: 15.0 / 255.0,
            b: 18.0 / 255.0,
            a: 1.0,
        }, // #0f0f12
        surface_subtle: Color {
            r: 20.0 / 255.0,
            g: 20.0 / 255.0,
            b: 24.0 / 255.0,
            a: 1.0,
        }, // #141418
        surface_elevated: Color {
            r: 28.0 / 255.0,
            g: 28.0 / 255.0,
            b: 34.0 / 255.0,
            a: 1.0,
        }, // #1c1c22
        idle: Color {
            r: 24.0 / 255.0,
            g: 24.0 / 255.0,
            b: 28.0 / 255.0,
            a: 1.0,
        }, // #18181c
        hovered: Color {
            r: 35.0 / 255.0,
            g: 35.0 / 255.0,
            b: 41.0 / 255.0,
            a: 1.0,
        }, // #232329
        pressed: Color {
            r: 45.0 / 255.0,
            g: 44.0 / 255.0,
            b: 54.0 / 255.0,
            a: 1.0,
        }, // #2d2c36
        active: Color {
            r: 79.0 / 255.0,
            g: 70.0 / 255.0,
            b: 229.0 / 255.0,
            a: 1.0,
        }, // #4f46e5 (Indigo 600)
        active_hover: Color {
            r: 99.0 / 255.0,
            g: 102.0 / 255.0,
            b: 241.0 / 255.0,
            a: 1.0,
        }, // #6366f1 (Indigo 500)
        border_faint: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.05,
        },
        border: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.08,
        },
        border_strong: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.16,
        },
        focus_border: Color {
            r: 99.0 / 255.0,
            g: 102.0 / 255.0,
            b: 241.0 / 255.0,
            a: 1.0,
        },
        text_primary: Color {
            r: 242.0 / 255.0,
            g: 242.0 / 255.0,
            b: 245.0 / 255.0,
            a: 1.0,
        }, // #f2f2f5
        text_secondary: Color {
            r: 161.0 / 255.0,
            g: 161.0 / 255.0,
            b: 170.0 / 255.0,
            a: 1.0,
        }, // Zinc 400
        text_muted: Color {
            r: 113.0 / 255.0,
            g: 113.0 / 255.0,
            b: 122.0 / 255.0,
            a: 1.0,
        }, // Zinc 500
        success: Color {
            r: 34.0 / 255.0,
            g: 197.0 / 255.0,
            b: 94.0 / 255.0,
            a: 1.0,
        }, // Emerald 500
        warning: Color {
            r: 245.0 / 255.0,
            g: 158.0 / 255.0,
            b: 11.0 / 255.0,
            a: 1.0,
        }, // Amber 500
        error: Color {
            r: 244.0 / 255.0,
            g: 63.0 / 255.0,
            b: 94.0 / 255.0,
            a: 1.0,
        }, // Rose 500
        shadow_ambient: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.28,
        },
        shadow_key: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.40,
        },
        selection: Color {
            r: 99.0 / 255.0,
            g: 102.0 / 255.0,
            b: 241.0 / 255.0,
            a: 0.30,
        },
    };

    /// Catppuccin Mocha -- Soothing, dark pastel warmth.
    pub const CATPPUCCIN_MOCHA: Theme = Theme {
        name: "catppuccin-mocha",
        is_dark: true,
        bg_canvas: Color {
            r: 30.0 / 255.0,
            g: 30.0 / 255.0,
            b: 46.0 / 255.0,
            a: 1.0,
        }, // #1e1e2e (Base)
        surface: Color {
            r: 24.0 / 255.0,
            g: 24.0 / 255.0,
            b: 37.0 / 255.0,
            a: 1.0,
        }, // #181825 (Mantle)
        surface_subtle: Color {
            r: 49.0 / 255.0,
            g: 50.0 / 255.0,
            b: 68.0 / 255.0,
            a: 1.0,
        }, // #313244 (Surface 0)
        surface_elevated: Color {
            r: 69.0 / 255.0,
            g: 71.0 / 255.0,
            b: 90.0 / 255.0,
            a: 1.0,
        }, // #45475a (Surface 1)
        idle: Color {
            r: 49.0 / 255.0,
            g: 50.0 / 255.0,
            b: 68.0 / 255.0,
            a: 1.0,
        }, // Surface 0
        hovered: Color {
            r: 69.0 / 255.0,
            g: 71.0 / 255.0,
            b: 90.0 / 255.0,
            a: 1.0,
        }, // Surface 1
        pressed: Color {
            r: 88.0 / 255.0,
            g: 91.0 / 255.0,
            b: 112.0 / 255.0,
            a: 1.0,
        }, // Surface 2
        active: Color {
            r: 203.0 / 255.0,
            g: 166.0 / 255.0,
            b: 247.0 / 255.0,
            a: 1.0,
        }, // #cba6f7 (Mauve)
        active_hover: Color {
            r: 180.0 / 255.0,
            g: 190.0 / 255.0,
            b: 254.0 / 255.0,
            a: 1.0,
        }, // #b4befe (Lavender)
        border_faint: Color {
            r: 205.0 / 255.0,
            g: 214.0 / 255.0,
            b: 244.0 / 255.0,
            a: 0.06,
        },
        border: Color {
            r: 205.0 / 255.0,
            g: 214.0 / 255.0,
            b: 244.0 / 255.0,
            a: 0.10,
        },
        border_strong: Color {
            r: 205.0 / 255.0,
            g: 214.0 / 255.0,
            b: 244.0 / 255.0,
            a: 0.20,
        },
        focus_border: Color {
            r: 203.0 / 255.0,
            g: 166.0 / 255.0,
            b: 247.0 / 255.0,
            a: 1.0,
        },
        text_primary: Color {
            r: 205.0 / 255.0,
            g: 214.0 / 255.0,
            b: 244.0 / 255.0,
            a: 1.0,
        }, // #cdd6f4 (Text)
        text_secondary: Color {
            r: 166.0 / 255.0,
            g: 173.0 / 255.0,
            b: 200.0 / 255.0,
            a: 1.0,
        }, // #a6adc8 (Subtext 0)
        text_muted: Color {
            r: 108.0 / 255.0,
            g: 112.0 / 255.0,
            b: 134.0 / 255.0,
            a: 1.0,
        }, // #6c7086 (Overlay 0)
        success: Color {
            r: 166.0 / 255.0,
            g: 227.0 / 255.0,
            b: 161.0 / 255.0,
            a: 1.0,
        }, // #a6e3a1 (Green)
        warning: Color {
            r: 249.0 / 255.0,
            g: 226.0 / 255.0,
            b: 175.0 / 255.0,
            a: 1.0,
        }, // #f9e2af (Yellow)
        error: Color {
            r: 243.0 / 255.0,
            g: 139.0 / 255.0,
            b: 168.0 / 255.0,
            a: 1.0,
        }, // #f38ba8 (Red)
        shadow_ambient: Color {
            r: 17.0 / 255.0,
            g: 17.0 / 255.0,
            b: 27.0 / 255.0,
            a: 0.35,
        },
        shadow_key: Color {
            r: 17.0 / 255.0,
            g: 17.0 / 255.0,
            b: 27.0 / 255.0,
            a: 0.55,
        },
        selection: Color {
            r: 203.0 / 255.0,
            g: 166.0 / 255.0,
            b: 247.0 / 255.0,
            a: 0.25,
        },
    };

    /// Catppuccin Latte -- Cozy, light pastel warmth.
    pub const CATPPUCCIN_LATTE: Theme = Theme {
        name: "catppuccin-latte",
        is_dark: false,
        bg_canvas: Color {
            r: 239.0 / 255.0,
            g: 241.0 / 255.0,
            b: 245.0 / 255.0,
            a: 1.0,
        }, // #eff1f5 (Base)
        surface: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }, // Pure white cards on latte
        surface_subtle: Color {
            r: 230.0 / 255.0,
            g: 233.0 / 255.0,
            b: 239.0 / 255.0,
            a: 1.0,
        }, // #e6e9ef (Mantle)
        surface_elevated: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        idle: Color {
            r: 230.0 / 255.0,
            g: 233.0 / 255.0,
            b: 239.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 204.0 / 255.0,
            g: 208.0 / 255.0,
            b: 218.0 / 255.0,
            a: 1.0,
        }, // #ccd0da (Surface 0)
        pressed: Color {
            r: 188.0 / 255.0,
            g: 192.0 / 255.0,
            b: 204.0 / 255.0,
            a: 1.0,
        }, // #bcc0cc (Surface 1)
        active: Color {
            r: 136.0 / 255.0,
            g: 57.0 / 255.0,
            b: 239.0 / 255.0,
            a: 1.0,
        }, // #8839ef (Mauve)
        active_hover: Color {
            r: 114.0 / 255.0,
            g: 135.0 / 255.0,
            b: 253.0 / 255.0,
            a: 1.0,
        }, // #7287fd (Lavender)
        border_faint: Color {
            r: 76.0 / 255.0,
            g: 79.0 / 255.0,
            b: 105.0 / 255.0,
            a: 0.05,
        },
        border: Color {
            r: 76.0 / 255.0,
            g: 79.0 / 255.0,
            b: 105.0 / 255.0,
            a: 0.10,
        },
        border_strong: Color {
            r: 76.0 / 255.0,
            g: 79.0 / 255.0,
            b: 105.0 / 255.0,
            a: 0.20,
        },
        focus_border: Color {
            r: 136.0 / 255.0,
            g: 57.0 / 255.0,
            b: 239.0 / 255.0,
            a: 0.90,
        },
        text_primary: Color {
            r: 76.0 / 255.0,
            g: 79.0 / 255.0,
            b: 105.0 / 255.0,
            a: 1.0,
        }, // #4c4f69 (Text)
        text_secondary: Color {
            r: 108.0 / 255.0,
            g: 111.0 / 255.0,
            b: 133.0 / 255.0,
            a: 1.0,
        }, // #6c6f85 (Subtext 0)
        text_muted: Color {
            r: 156.0 / 255.0,
            g: 160.0 / 255.0,
            b: 176.0 / 255.0,
            a: 1.0,
        }, // #9ca0b0 (Overlay 0)
        success: Color {
            r: 64.0 / 255.0,
            g: 160.0 / 255.0,
            b: 43.0 / 255.0,
            a: 1.0,
        }, // #40a02b (Green)
        warning: Color {
            r: 223.0 / 255.0,
            g: 142.0 / 255.0,
            b: 29.0 / 255.0,
            a: 1.0,
        }, // #df8e1d (Yellow)
        error: Color {
            r: 210.0 / 255.0,
            g: 15.0 / 255.0,
            b: 57.0 / 255.0,
            a: 1.0,
        }, // #d20f39 (Red)
        shadow_ambient: Color {
            r: 76.0 / 255.0,
            g: 79.0 / 255.0,
            b: 105.0 / 255.0,
            a: 0.04,
        },
        shadow_key: Color {
            r: 76.0 / 255.0,
            g: 79.0 / 255.0,
            b: 105.0 / 255.0,
            a: 0.08,
        },
        selection: Color {
            r: 136.0 / 255.0,
            g: 57.0 / 255.0,
            b: 239.0 / 255.0,
            a: 0.15,
        },
    };

    /// Tokyo Night -- Iconic cyberpunk midnight palette.
    pub const TOKYO_NIGHT: Theme = Theme {
        name: "tokyo-night",
        is_dark: true,
        bg_canvas: Color {
            r: 26.0 / 255.0,
            g: 27.0 / 255.0,
            b: 38.0 / 255.0,
            a: 1.0,
        }, // #1a1b26 (Night)
        surface: Color {
            r: 36.0 / 255.0,
            g: 40.0 / 255.0,
            b: 59.0 / 255.0,
            a: 1.0,
        }, // #24283b (Storm)
        surface_subtle: Color {
            r: 31.0 / 255.0,
            g: 35.0 / 255.0,
            b: 53.0 / 255.0,
            a: 1.0,
        }, // #1f2335
        surface_elevated: Color {
            r: 41.0 / 255.0,
            g: 46.0 / 255.0,
            b: 66.0 / 255.0,
            a: 1.0,
        }, // #292e42
        idle: Color {
            r: 36.0 / 255.0,
            g: 40.0 / 255.0,
            b: 59.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 47.0 / 255.0,
            g: 53.0 / 255.0,
            b: 77.0 / 255.0,
            a: 1.0,
        }, // #2f354d
        pressed: Color {
            r: 59.0 / 255.0,
            g: 66.0 / 255.0,
            b: 97.0 / 255.0,
            a: 1.0,
        }, // #3b4261
        active: Color {
            r: 122.0 / 255.0,
            g: 162.0 / 255.0,
            b: 247.0 / 255.0,
            a: 1.0,
        }, // #7aa2f7 (Blue)
        active_hover: Color {
            r: 187.0 / 255.0,
            g: 154.0 / 255.0,
            b: 247.0 / 255.0,
            a: 1.0,
        }, // #bb9af7 (Magenta)
        border_faint: Color {
            r: 192.0 / 255.0,
            g: 202.0 / 255.0,
            b: 245.0 / 255.0,
            a: 0.06,
        },
        border: Color {
            r: 192.0 / 255.0,
            g: 202.0 / 255.0,
            b: 245.0 / 255.0,
            a: 0.10,
        },
        border_strong: Color {
            r: 192.0 / 255.0,
            g: 202.0 / 255.0,
            b: 245.0 / 255.0,
            a: 0.20,
        },
        focus_border: Color {
            r: 122.0 / 255.0,
            g: 162.0 / 255.0,
            b: 247.0 / 255.0,
            a: 1.0,
        },
        text_primary: Color {
            r: 192.0 / 255.0,
            g: 202.0 / 255.0,
            b: 245.0 / 255.0,
            a: 1.0,
        }, // #c0caf5
        text_secondary: Color {
            r: 169.0 / 255.0,
            g: 177.0 / 255.0,
            b: 214.0 / 255.0,
            a: 1.0,
        }, // #a9b1d6
        text_muted: Color {
            r: 86.0 / 255.0,
            g: 95.0 / 255.0,
            b: 137.0 / 255.0,
            a: 1.0,
        }, // #565f89
        success: Color {
            r: 158.0 / 255.0,
            g: 206.0 / 255.0,
            b: 106.0 / 255.0,
            a: 1.0,
        }, // #9ece6a
        warning: Color {
            r: 224.0 / 255.0,
            g: 175.0 / 255.0,
            b: 104.0 / 255.0,
            a: 1.0,
        }, // #e0af68
        error: Color {
            r: 247.0 / 255.0,
            g: 118.0 / 255.0,
            b: 142.0 / 255.0,
            a: 1.0,
        }, // #f7768e
        shadow_ambient: Color {
            r: 15.0 / 255.0,
            g: 15.0 / 255.0,
            b: 23.0 / 255.0,
            a: 0.35,
        },
        shadow_key: Color {
            r: 15.0 / 255.0,
            g: 15.0 / 255.0,
            b: 23.0 / 255.0,
            a: 0.55,
        },
        selection: Color {
            r: 122.0 / 255.0,
            g: 162.0 / 255.0,
            b: 247.0 / 255.0,
            a: 0.25,
        },
    };

    /// Gruvbox Dark -- Warm retro groove charcoal and orange.
    pub const GRUVBOX_DARK: Theme = Theme {
        name: "gruvbox-dark",
        is_dark: true,
        bg_canvas: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 1.0,
        }, // #282828 (bg0)
        surface: Color {
            r: 50.0 / 255.0,
            g: 48.0 / 255.0,
            b: 47.0 / 255.0,
            a: 1.0,
        }, // #32302f (dark0_soft)
        surface_subtle: Color {
            r: 60.0 / 255.0,
            g: 56.0 / 255.0,
            b: 54.0 / 255.0,
            a: 1.0,
        }, // #3c3836 (bg1)
        surface_elevated: Color {
            r: 80.0 / 255.0,
            g: 73.0 / 255.0,
            b: 69.0 / 255.0,
            a: 1.0,
        }, // #504945 (bg2)
        idle: Color {
            r: 60.0 / 255.0,
            g: 56.0 / 255.0,
            b: 54.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 80.0 / 255.0,
            g: 73.0 / 255.0,
            b: 69.0 / 255.0,
            a: 1.0,
        },
        pressed: Color {
            r: 102.0 / 255.0,
            g: 92.0 / 255.0,
            b: 84.0 / 255.0,
            a: 1.0,
        }, // #665c54
        active: Color {
            r: 254.0 / 255.0,
            g: 128.0 / 255.0,
            b: 25.0 / 255.0,
            a: 1.0,
        }, // #fe8019 (Orange)
        active_hover: Color {
            r: 250.0 / 255.0,
            g: 189.0 / 255.0,
            b: 47.0 / 255.0,
            a: 1.0,
        }, // #fabd2f (Yellow)
        border_faint: Color {
            r: 251.0 / 255.0,
            g: 241.0 / 255.0,
            b: 199.0 / 255.0,
            a: 0.06,
        },
        border: Color {
            r: 251.0 / 255.0,
            g: 241.0 / 255.0,
            b: 199.0 / 255.0,
            a: 0.10,
        },
        border_strong: Color {
            r: 251.0 / 255.0,
            g: 241.0 / 255.0,
            b: 199.0 / 255.0,
            a: 0.20,
        },
        focus_border: Color {
            r: 254.0 / 255.0,
            g: 128.0 / 255.0,
            b: 25.0 / 255.0,
            a: 1.0,
        },
        text_primary: Color {
            r: 251.0 / 255.0,
            g: 241.0 / 255.0,
            b: 199.0 / 255.0,
            a: 1.0,
        }, // #fbf1c7 (fg0)
        text_secondary: Color {
            r: 235.0 / 255.0,
            g: 219.0 / 255.0,
            b: 178.0 / 255.0,
            a: 1.0,
        }, // #ebdbb2 (fg1)
        text_muted: Color {
            r: 146.0 / 255.0,
            g: 131.0 / 255.0,
            b: 116.0 / 255.0,
            a: 1.0,
        }, // #928374 (gray)
        success: Color {
            r: 184.0 / 255.0,
            g: 187.0 / 255.0,
            b: 38.0 / 255.0,
            a: 1.0,
        }, // #b8bb26 (Green)
        warning: Color {
            r: 250.0 / 255.0,
            g: 189.0 / 255.0,
            b: 47.0 / 255.0,
            a: 1.0,
        }, // #fabd2f (Yellow)
        error: Color {
            r: 251.0 / 255.0,
            g: 73.0 / 255.0,
            b: 52.0 / 255.0,
            a: 1.0,
        }, // #fb4934 (Red)
        shadow_ambient: Color {
            r: 29.0 / 255.0,
            g: 32.0 / 255.0,
            b: 33.0 / 255.0,
            a: 0.35,
        },
        shadow_key: Color {
            r: 29.0 / 255.0,
            g: 32.0 / 255.0,
            b: 33.0 / 255.0,
            a: 0.55,
        },
        selection: Color {
            r: 254.0 / 255.0,
            g: 128.0 / 255.0,
            b: 25.0 / 255.0,
            a: 0.25,
        },
    };

    /// Gruvbox Light -- Warm retro groove light paper and rust accent.
    pub const GRUVBOX_LIGHT: Theme = Theme {
        name: "gruvbox-light",
        is_dark: false,
        bg_canvas: Color {
            r: 251.0 / 255.0,
            g: 241.0 / 255.0,
            b: 199.0 / 255.0,
            a: 1.0,
        }, // #fbf1c7 (bg0)
        surface: Color {
            r: 249.0 / 255.0,
            g: 245.0 / 255.0,
            b: 215.0 / 255.0,
            a: 1.0,
        }, // #f9f5d7 (light0_hard)
        surface_subtle: Color {
            r: 235.0 / 255.0,
            g: 219.0 / 255.0,
            b: 178.0 / 255.0,
            a: 1.0,
        }, // #ebdbb2
        surface_elevated: Color {
            r: 249.0 / 255.0,
            g: 245.0 / 255.0,
            b: 215.0 / 255.0,
            a: 1.0,
        },
        idle: Color {
            r: 235.0 / 255.0,
            g: 219.0 / 255.0,
            b: 178.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 213.0 / 255.0,
            g: 196.0 / 255.0,
            b: 161.0 / 255.0,
            a: 1.0,
        }, // #d5c4a1
        pressed: Color {
            r: 189.0 / 255.0,
            g: 174.0 / 255.0,
            b: 147.0 / 255.0,
            a: 1.0,
        }, // #bdae93
        active: Color {
            r: 175.0 / 255.0,
            g: 58.0 / 255.0,
            b: 3.0 / 255.0,
            a: 1.0,
        }, // #af3a03 (Rust)
        active_hover: Color {
            r: 214.0 / 255.0,
            g: 93.0 / 255.0,
            b: 14.0 / 255.0,
            a: 1.0,
        }, // #d65d0e
        border_faint: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 0.06,
        },
        border: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 0.12,
        },
        border_strong: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 0.22,
        },
        focus_border: Color {
            r: 175.0 / 255.0,
            g: 58.0 / 255.0,
            b: 3.0 / 255.0,
            a: 0.90,
        },
        text_primary: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 1.0,
        }, // #282828 (fg0)
        text_secondary: Color {
            r: 60.0 / 255.0,
            g: 56.0 / 255.0,
            b: 54.0 / 255.0,
            a: 1.0,
        }, // #3c3836 (fg1)
        text_muted: Color {
            r: 124.0 / 255.0,
            g: 111.0 / 255.0,
            b: 100.0 / 255.0,
            a: 1.0,
        }, // #7c6f64
        success: Color {
            r: 121.0 / 255.0,
            g: 116.0 / 255.0,
            b: 14.0 / 255.0,
            a: 1.0,
        }, // Green
        warning: Color {
            r: 181.0 / 255.0,
            g: 118.0 / 255.0,
            b: 20.0 / 255.0,
            a: 1.0,
        }, // Yellow
        error: Color {
            r: 157.0 / 255.0,
            g: 0.0 / 255.0,
            b: 6.0 / 255.0,
            a: 1.0,
        }, // Red
        shadow_ambient: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 0.04,
        },
        shadow_key: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 0.08,
        },
        selection: Color {
            r: 175.0 / 255.0,
            g: 58.0 / 255.0,
            b: 3.0 / 255.0,
            a: 0.15,
        },
    };

    /// Nord -- Arctic, north-bluish palette.
    pub const NORD: Theme = Theme {
        name: "nord",
        is_dark: true,
        bg_canvas: Color {
            r: 46.0 / 255.0,
            g: 52.0 / 255.0,
            b: 64.0 / 255.0,
            a: 1.0,
        }, // #2e3440 (Polar Night 0)
        surface: Color {
            r: 59.0 / 255.0,
            g: 66.0 / 255.0,
            b: 82.0 / 255.0,
            a: 1.0,
        }, // #3b4252 (Polar Night 1)
        surface_subtle: Color {
            r: 67.0 / 255.0,
            g: 76.0 / 255.0,
            b: 94.0 / 255.0,
            a: 1.0,
        }, // #434c5e (Polar Night 2)
        surface_elevated: Color {
            r: 76.0 / 255.0,
            g: 86.0 / 255.0,
            b: 106.0 / 255.0,
            a: 1.0,
        }, // #4c566a (Polar Night 3)
        idle: Color {
            r: 59.0 / 255.0,
            g: 66.0 / 255.0,
            b: 82.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 67.0 / 255.0,
            g: 76.0 / 255.0,
            b: 94.0 / 255.0,
            a: 1.0,
        },
        pressed: Color {
            r: 76.0 / 255.0,
            g: 86.0 / 255.0,
            b: 106.0 / 255.0,
            a: 1.0,
        },
        active: Color {
            r: 136.0 / 255.0,
            g: 192.0 / 255.0,
            b: 208.0 / 255.0,
            a: 1.0,
        }, // #88c0d0 (Frost Cyan)
        active_hover: Color {
            r: 129.0 / 255.0,
            g: 161.0 / 255.0,
            b: 193.0 / 255.0,
            a: 1.0,
        }, // #81a1c1 (Frost Blue)
        border_faint: Color {
            r: 216.0 / 255.0,
            g: 222.0 / 255.0,
            b: 233.0 / 255.0,
            a: 0.06,
        },
        border: Color {
            r: 216.0 / 255.0,
            g: 222.0 / 255.0,
            b: 233.0 / 255.0,
            a: 0.10,
        },
        border_strong: Color {
            r: 216.0 / 255.0,
            g: 222.0 / 255.0,
            b: 233.0 / 255.0,
            a: 0.20,
        },
        focus_border: Color {
            r: 136.0 / 255.0,
            g: 192.0 / 255.0,
            b: 208.0 / 255.0,
            a: 1.0,
        },
        text_primary: Color {
            r: 236.0 / 255.0,
            g: 239.0 / 255.0,
            b: 244.0 / 255.0,
            a: 1.0,
        }, // #eceff4 (Snow Storm 0)
        text_secondary: Color {
            r: 216.0 / 255.0,
            g: 222.0 / 255.0,
            b: 233.0 / 255.0,
            a: 1.0,
        }, // #d8dee9 (Snow Storm 2)
        text_muted: Color {
            r: 123.0 / 255.0,
            g: 136.0 / 255.0,
            b: 161.0 / 255.0,
            a: 1.0,
        },
        success: Color {
            r: 163.0 / 255.0,
            g: 190.0 / 255.0,
            b: 140.0 / 255.0,
            a: 1.0,
        }, // #a3be8c (Green)
        warning: Color {
            r: 235.0 / 255.0,
            g: 203.0 / 255.0,
            b: 139.0 / 255.0,
            a: 1.0,
        }, // #ebcb8b (Yellow)
        error: Color {
            r: 191.0 / 255.0,
            g: 97.0 / 255.0,
            b: 106.0 / 255.0,
            a: 1.0,
        }, // #bf616a (Red)
        shadow_ambient: Color {
            r: 36.0 / 255.0,
            g: 41.0 / 255.0,
            b: 51.0 / 255.0,
            a: 0.35,
        },
        shadow_key: Color {
            r: 36.0 / 255.0,
            g: 41.0 / 255.0,
            b: 51.0 / 255.0,
            a: 0.55,
        },
        selection: Color {
            r: 136.0 / 255.0,
            g: 192.0 / 255.0,
            b: 208.0 / 255.0,
            a: 0.25,
        },
    };

    /// Rosé Pine -- Soho vignette, moody rose & pine.
    pub const ROSE_PINE: Theme = Theme {
        name: "rose-pine",
        is_dark: true,
        bg_canvas: Color {
            r: 25.0 / 255.0,
            g: 23.0 / 255.0,
            b: 36.0 / 255.0,
            a: 1.0,
        }, // #191724 (Base)
        surface: Color {
            r: 31.0 / 255.0,
            g: 29.0 / 255.0,
            b: 46.0 / 255.0,
            a: 1.0,
        }, // #1f1d2e (Surface)
        surface_subtle: Color {
            r: 38.0 / 255.0,
            g: 35.0 / 255.0,
            b: 58.0 / 255.0,
            a: 1.0,
        }, // #26233a (Overlay)
        surface_elevated: Color {
            r: 42.0 / 255.0,
            g: 40.0 / 255.0,
            b: 62.0 / 255.0,
            a: 1.0,
        },
        idle: Color {
            r: 38.0 / 255.0,
            g: 35.0 / 255.0,
            b: 58.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 49.0 / 255.0,
            g: 47.0 / 255.0,
            b: 72.0 / 255.0,
            a: 1.0,
        },
        pressed: Color {
            r: 57.0 / 255.0,
            g: 53.0 / 255.0,
            b: 82.0 / 255.0,
            a: 1.0,
        },
        active: Color {
            r: 235.0 / 255.0,
            g: 111.0 / 255.0,
            b: 146.0 / 255.0,
            a: 1.0,
        }, // #eb6f92 (Love / Rose)
        active_hover: Color {
            r: 196.0 / 255.0,
            g: 167.0 / 255.0,
            b: 231.0 / 255.0,
            a: 1.0,
        }, // #c4a7e7 (Iris)
        border_faint: Color {
            r: 224.0 / 255.0,
            g: 222.0 / 255.0,
            b: 244.0 / 255.0,
            a: 0.06,
        },
        border: Color {
            r: 224.0 / 255.0,
            g: 222.0 / 255.0,
            b: 244.0 / 255.0,
            a: 0.10,
        },
        border_strong: Color {
            r: 224.0 / 255.0,
            g: 222.0 / 255.0,
            b: 244.0 / 255.0,
            a: 0.20,
        },
        focus_border: Color {
            r: 235.0 / 255.0,
            g: 111.0 / 255.0,
            b: 146.0 / 255.0,
            a: 1.0,
        },
        text_primary: Color {
            r: 224.0 / 255.0,
            g: 222.0 / 255.0,
            b: 244.0 / 255.0,
            a: 1.0,
        }, // #e0def4 (Text)
        text_secondary: Color {
            r: 144.0 / 255.0,
            g: 140.0 / 255.0,
            b: 170.0 / 255.0,
            a: 1.0,
        }, // #908caa (Subtle)
        text_muted: Color {
            r: 110.0 / 255.0,
            g: 106.0 / 255.0,
            b: 134.0 / 255.0,
            a: 1.0,
        }, // #6e6a86 (Muted)
        success: Color {
            r: 156.0 / 255.0,
            g: 207.0 / 255.0,
            b: 216.0 / 255.0,
            a: 1.0,
        }, // #9ccfd8 (Foam)
        warning: Color {
            r: 246.0 / 255.0,
            g: 193.0 / 255.0,
            b: 119.0 / 255.0,
            a: 1.0,
        }, // #f6c177 (Gold)
        error: Color {
            r: 235.0 / 255.0,
            g: 111.0 / 255.0,
            b: 146.0 / 255.0,
            a: 1.0,
        }, // #eb6f92 (Love)
        shadow_ambient: Color {
            r: 20.0 / 255.0,
            g: 18.0 / 255.0,
            b: 30.0 / 255.0,
            a: 0.35,
        },
        shadow_key: Color {
            r: 20.0 / 255.0,
            g: 18.0 / 255.0,
            b: 30.0 / 255.0,
            a: 0.55,
        },
        selection: Color {
            r: 235.0 / 255.0,
            g: 111.0 / 255.0,
            b: 146.0 / 255.0,
            a: 0.25,
        },
    };

    // =========================================================================
    // Preset Theme Accessors
    // =========================================================================

    pub fn light() -> Self {
        Self::LIGHT
    }

    pub fn dark() -> Self {
        Self::DARK
    }

    pub fn catppuccin_mocha() -> Self {
        Self::CATPPUCCIN_MOCHA
    }

    pub fn catppuccin_latte() -> Self {
        Self::CATPPUCCIN_LATTE
    }

    pub fn tokyo_night() -> Self {
        Self::TOKYO_NIGHT
    }

    pub fn gruvbox_dark() -> Self {
        Self::GRUVBOX_DARK
    }

    pub fn gruvbox_light() -> Self {
        Self::GRUVBOX_LIGHT
    }

    pub fn nord() -> Self {
        Self::NORD
    }

    pub fn rose_pine() -> Self {
        Self::ROSE_PINE
    }

    /// Slice of all 9 curated presets for live theme switchers or demos.
    pub fn all() -> &'static [Theme] {
        &[
            Self::LIGHT,
            Self::DARK,
            Self::CATPPUCCIN_MOCHA,
            Self::CATPPUCCIN_LATTE,
            Self::TOKYO_NIGHT,
            Self::GRUVBOX_DARK,
            Self::GRUVBOX_LIGHT,
            Self::NORD,
            Self::ROSE_PINE,
        ]
    }

    /// Looks up a preset theme by case-insensitive name or slug.
    pub fn from_name(name: &str) -> Option<Theme> {
        let slug = name.trim().to_lowercase().replace(' ', "-");
        match slug.as_str() {
            "light" | "shadcn-light" | "white" => Some(Self::LIGHT),
            "dark" | "shadcn-dark" | "linear" => Some(Self::DARK),
            "catppuccin" | "catppuccin-mocha" | "mocha" => Some(Self::CATPPUCCIN_MOCHA),
            "catppuccin-latte" | "latte" => Some(Self::CATPPUCCIN_LATTE),
            "tokyo-night" | "tokyonight" => Some(Self::TOKYO_NIGHT),
            "gruvbox" | "gruvbox-dark" => Some(Self::GRUVBOX_DARK),
            "gruvbox-light" => Some(Self::GRUVBOX_LIGHT),
            "nord" => Some(Self::NORD),
            "rose-pine" | "rosepine" => Some(Self::ROSE_PINE),
            _ => None,
        }
    }

    // =========================================================================
    // Static Constants for Default White Theme (Compatibility)
    // =========================================================================

    pub const BG_CANVAS: Color = Self::LIGHT.bg_canvas;
    pub const SURFACE: Color = Self::LIGHT.surface;
    pub const SURFACE_SUBTLE: Color = Self::LIGHT.surface_subtle;
    pub const SURFACE_ELEVATED: Color = Self::LIGHT.surface_elevated;

    pub const IDLE: Color = Self::LIGHT.idle;
    pub const HOVERED: Color = Self::LIGHT.hovered;
    pub const PRESSED: Color = Self::LIGHT.pressed;
    pub const ACTIVE: Color = Self::LIGHT.active;
    pub const ACTIVE_HOVER: Color = Self::LIGHT.active_hover;

    pub const BORDER_FAINT: Color = Self::LIGHT.border_faint;
    pub const BORDER: Color = Self::LIGHT.border;
    pub const BORDER_STRONG: Color = Self::LIGHT.border_strong;
    pub const FOCUS_BORDER: Color = Self::LIGHT.focus_border;

    pub const TEXT_PRIMARY: Color = Self::LIGHT.text_primary;
    pub const TEXT_SECONDARY: Color = Self::LIGHT.text_secondary;
    pub const TEXT_MUTED: Color = Self::LIGHT.text_muted;
    pub const TEXT_DIM: Color = Self::LIGHT.text_muted;

    pub const SUCCESS: Color = Self::LIGHT.success;
    pub const WARNING: Color = Self::LIGHT.warning;
    pub const ERROR: Color = Self::LIGHT.error;

    pub const SHADOW_AMBIENT: Color = Self::LIGHT.shadow_ambient;
    pub const SHADOW_KEY: Color = Self::LIGHT.shadow_key;
    pub const SURFACE_SHADOW: Color = Self::LIGHT.shadow_ambient;
    pub const SHADOW: Color = Self::LIGHT.shadow_key;

    pub const SELECTION: Color = Self::LIGHT.selection;

    // =========================================================================
    // Sizing & Grid Constants (4px base)
    // =========================================================================

    pub const CONTROL_HEIGHT_SM: f32 = 28.0;
    pub const CONTROL_HEIGHT_MD: f32 = 36.0;
    pub const CONTROL_HEIGHT_LG: f32 = 44.0;

    pub const CONTROL_PAD_X: f32 = 14.0;
    pub const CONTROL_PAD_Y: f32 = 9.0;

    pub const RADIUS_XS: f32 = 3.0;
    pub const RADIUS_SM: f32 = 5.0;
    pub const RADIUS_MD: f32 = 7.0;
    pub const RADIUS_LG: f32 = 12.0;
    pub const RADIUS_FULL: f32 = 9999.0;

    pub const SPACE_1: f32 = 4.0;
    pub const SPACE_2: f32 = 8.0;
    pub const SPACE_3: f32 = 12.0;
    pub const SPACE_4: f32 = 16.0;
    pub const SPACE_6: f32 = 24.0;
    pub const SPACE_8: f32 = 32.0;

    // =========================================================================
    // Instance Helpers & Style Builders
    // =========================================================================

    pub fn state_color(&self, active: bool, pressed: bool, hovered: bool) -> Color {
        if active {
            self.active
        } else if pressed {
            self.pressed
        } else if hovered {
            self.hovered
        } else {
            self.idle
        }
    }

    /// Dynamic small shadow for controls matching this theme.
    pub fn shadow_sm(&self) -> [ShadowStyle; 2] {
        [
            ShadowStyle {
                color: self.shadow_ambient,
                blur_radius: 3.0,
                offset: [0.0, 1.0],
            },
            ShadowStyle {
                color: self.shadow_key,
                blur_radius: 6.0,
                offset: [0.0, 2.0],
            },
        ]
    }

    /// Dynamic medium shadow for cards/panels matching this theme.
    pub fn shadow_md(&self) -> [ShadowStyle; 2] {
        [
            ShadowStyle {
                color: self.shadow_ambient,
                blur_radius: 12.0,
                offset: [0.0, 3.0],
            },
            ShadowStyle {
                color: self.shadow_key,
                blur_radius: 4.0,
                offset: [0.0, 2.0],
            },
        ]
    }

    /// Dynamic large shadow for floating overlays matching this theme.
    pub fn shadow_lg(&self) -> [ShadowStyle; 2] {
        [
            ShadowStyle {
                color: self.shadow_ambient,
                blur_radius: 24.0,
                offset: [0.0, 6.0],
            },
            ShadowStyle {
                color: self.shadow_key,
                blur_radius: 8.0,
                offset: [0.0, 3.0],
            },
        ]
    }

    /// Constructs a standard button style customized for this theme.
    pub fn button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(self.surface_subtle),
            hover_fill: Fill::Solid(self.hovered),
            pressed_fill: Fill::Solid(self.pressed),
            text_color: self.text_primary,
            border_width: 1.0,
            border_color: self.border,
            corner_radius: 6.0,
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }

    /// Constructs a primary CTA button style customized for this theme.
    pub fn primary_button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(self.active),
            hover_fill: Fill::Solid(self.active_hover),
            pressed_fill: Fill::Solid(self.active.darken(0.12)),
            text_color: if self.is_dark && self.active.r > 0.7 && self.active.g > 0.7 {
                Color::BLACK
            } else {
                Color::WHITE
            },
            border_width: 1.0,
            border_color: Color::WHITE.with_alpha(0.15),
            corner_radius: 6.0,
            shadow: Some(ShadowStyle {
                color: self.active.with_alpha(0.20),
                blur_radius: 4.0,
                offset: [0.0, 1.5],
            }),
            sharp: false,
        }
    }

    /// Constructs an outline button style customized for this theme.
    pub fn outline_button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(Color::TRANSPARENT),
            hover_fill: Fill::Solid(self.surface_subtle),
            pressed_fill: Fill::Solid(self.hovered),
            text_color: self.text_primary,
            border_width: 1.0,
            border_color: self.border_strong,
            corner_radius: 6.0,
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }

    /// Constructs a ghost button style customized for this theme.
    pub fn ghost_button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(Color::TRANSPARENT),
            hover_fill: Fill::Solid(self.surface_subtle),
            pressed_fill: Fill::Solid(self.hovered),
            text_color: self.text_secondary,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 6.0,
            shadow: None,
            sharp: false,
        }
    }

    /// Constructs a danger button style customized for this theme.
    pub fn danger_button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(self.error),
            hover_fill: Fill::Solid(self.error.lighten(0.08)),
            pressed_fill: Fill::Solid(self.error.darken(0.12)),
            text_color: Color::WHITE,
            border_width: 1.0,
            border_color: self.error.darken(0.15),
            corner_radius: 6.0,
            shadow: Some(ShadowStyle {
                color: self.error.with_alpha(0.25),
                blur_radius: 4.0,
                offset: [0.0, 1.5],
            }),
            sharp: false,
        }
    }

    /// Constructs a card container style customized for this theme.
    pub fn card_style(&self) -> CardStyle {
        CardStyle {
            fill: Fill::Solid(self.surface),
            border_width: 1.0,
            border_color: self.border,
            corner_radius: 10.0,
            padding: [20.0, 20.0],
            shadow: Some(self.shadow_sm()[0]),
        }
    }

    /// Constructs an inset subtle card style customized for this theme.
    pub fn card_subtle_style(&self) -> CardStyle {
        CardStyle {
            fill: Fill::Solid(self.surface_subtle),
            border_width: 1.0,
            border_color: self.border_faint,
            corner_radius: 8.0,
            padding: [16.0, 16.0],
            shadow: None,
        }
    }

    /// Constructs an elevated card style customized for this theme.
    pub fn card_elevated_style(&self) -> CardStyle {
        CardStyle {
            fill: Fill::Solid(self.surface_elevated),
            border_width: 1.0,
            border_color: self.border_strong,
            corner_radius: 12.0,
            padding: [20.0, 20.0],
            shadow: Some(self.shadow_md()[0]),
        }
    }

    /// Constructs a checkbox style customized for this theme.
    pub fn checkbox_style(&self) -> CheckboxStyle {
        CheckboxStyle {
            fill: Fill::Solid(if self.is_dark {
                self.surface
            } else {
                Color::WHITE
            }),
            hover_fill: Fill::Solid(self.surface_subtle),
            checked_fill: Fill::Solid(self.active),
            check_color: if self.is_dark && self.active.r > 0.8 && self.active.g > 0.8 {
                Color::BLACK
            } else {
                Color::WHITE
            },
            border_width: 1.0,
            border_color: self.border_strong,
            corner_radius: 4.0,
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }

    /// Constructs a switch toggle style customized for this theme.
    pub fn switch_style(&self) -> SwitchStyle {
        SwitchStyle {
            track_off_fill: Fill::Solid(self.hovered),
            track_on_fill: Fill::Solid(self.active),
            thumb_fill: Fill::Solid(Color::WHITE),
            border_width: 1.0,
            border_color: self.border,
            corner_radius: 11.0,
            shadow: Some(self.shadow_sm()[0]),
        }
    }

    /// Constructs a slider style customized for this theme.
    pub fn slider_style(&self) -> SliderStyle {
        SliderStyle {
            track_fill: Fill::Solid(self.hovered),
            filled_fill: Fill::Solid(self.active),
            thumb_fill: Fill::Solid(Color::WHITE),
            border_width: 1.5,
            border_color: self.border_strong,
            track_height: 4.0,
            thumb_size: 16.0,
            shadow: Some(self.shadow_sm()[0]),
        }
    }

    /// Constructs a text input style customized for this theme.
    pub fn input_style(&self) -> TextInputStyle {
        TextInputStyle {
            fill: Fill::Solid(self.surface),
            text_color: self.text_primary,
            placeholder_color: self.text_muted,
            border_width: 1.0,
            border_color: self.border_strong,
            focus_border_color: self.focus_border,
            corner_radius: 6.0,
            selection_color: self.selection,
            cursor_color: self.active,
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }

    /// Constructs a text area editor style customized for this theme.
    pub fn text_area_style(&self) -> TextAreaStyle {
        TextAreaStyle {
            fill: Fill::Solid(self.surface),
            text_color: self.text_primary,
            border_width: 1.0,
            border_color: self.border_strong,
            focus_border_color: self.focus_border,
            corner_radius: 6.0,
            selection_color: self.selection,
            cursor_color: self.active,
            thumb_fill: Fill::Solid(self.text_muted.with_alpha(0.35)),
            thumb_dragging_fill: Fill::Solid(self.text_muted.with_alpha(0.65)),
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }
}
