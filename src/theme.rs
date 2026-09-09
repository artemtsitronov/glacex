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

    /// Pristine White / Light Theme (Default -- Apple & Linear style).
    /// Soft platinum canvas, pure white elevated cards, and electric indigo primary action.
    pub const LIGHT: Theme = Theme {
        name: "shadcn-light",
        is_dark: false,
        bg_canvas: Color {
            r: 243.0 / 255.0,
            g: 244.0 / 255.0,
            b: 247.0 / 255.0,
            a: 1.0,
        }, // #f3f4f7 (macOS Sonoma / Linear Light Canvas)
        surface: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }, // #ffffff (Pure White floating cards)
        surface_subtle: Color {
            r: 235.0 / 255.0,
            g: 237.0 / 255.0,
            b: 242.0 / 255.0,
            a: 1.0,
        }, // #ebedf2 (Soft inset controls/inputs)
        surface_elevated: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }, // #ffffff (Elevated overlays)
        idle: Color {
            r: 235.0 / 255.0,
            g: 237.0 / 255.0,
            b: 242.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 226.0 / 255.0,
            g: 229.0 / 255.0,
            b: 236.0 / 255.0,
            a: 1.0,
        }, // #e2e5ec
        pressed: Color {
            r: 216.0 / 255.0,
            g: 220.0 / 255.0,
            b: 228.0 / 255.0,
            a: 1.0,
        }, // #d8dce4
        active: Color {
            r: 79.0 / 255.0,
            g: 70.0 / 255.0,
            b: 229.0 / 255.0,
            a: 1.0,
        }, // #4f46e5 (Electric Indigo)
        active_hover: Color {
            r: 67.0 / 255.0,
            g: 56.0 / 255.0,
            b: 202.0 / 255.0,
            a: 1.0,
        }, // #4338ca
        border_faint: Color::TRANSPARENT,
        border: Color::TRANSPARENT,
        border_strong: Color::TRANSPARENT,
        focus_border: Color {
            r: 79.0 / 255.0,
            g: 70.0 / 255.0,
            b: 229.0 / 255.0,
            a: 0.50,
        },
        text_primary: Color {
            r: 15.0 / 255.0,
            g: 23.0 / 255.0,
            b: 42.0 / 255.0,
            a: 1.0,
        }, // #0f172a (Slate 900)
        text_secondary: Color {
            r: 71.0 / 255.0,
            g: 85.0 / 255.0,
            b: 105.0 / 255.0,
            a: 1.0,
        }, // #475569 (Slate 600)
        text_muted: Color {
            r: 148.0 / 255.0,
            g: 163.0 / 255.0,
            b: 184.0 / 255.0,
            a: 1.0,
        }, // #94a3b8 (Slate 400)
        success: Color {
            r: 16.0 / 255.0,
            g: 185.0 / 255.0,
            b: 129.0 / 255.0,
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
            r: 15.0 / 255.0,
            g: 23.0 / 255.0,
            b: 42.0 / 255.0,
            a: 0.06,
        },
        shadow_key: Color {
            r: 15.0 / 255.0,
            g: 23.0 / 255.0,
            b: 42.0 / 255.0,
            a: 0.12,
        },
        selection: Color {
            r: 79.0 / 255.0,
            g: 70.0 / 255.0,
            b: 229.0 / 255.0,
            a: 0.15,
        },
    };

    /// Pure Pitch-Black OLED / Linear Dark Theme.
    /// Deep pure black canvas (#000000), rich elevated obsidian cards (#0e0f13, #16171d),
    /// zero borders and radiant high-contrast actions.
    pub const DARK: Theme = Theme {
        name: "shadcn-dark",
        is_dark: true,
        bg_canvas: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }, // #000000 (Pure Pitch Black OLED)
        surface: Color {
            r: 14.0 / 255.0,
            g: 15.0 / 255.0,
            b: 19.0 / 255.0,
            a: 1.0,
        }, // #0e0f13 (Obsidian Zinc)
        surface_subtle: Color {
            r: 22.0 / 255.0,
            g: 23.0 / 255.0,
            b: 29.0 / 255.0,
            a: 1.0,
        }, // #16171d
        surface_elevated: Color {
            r: 30.0 / 255.0,
            g: 32.0 / 255.0,
            b: 40.0 / 255.0,
            a: 1.0,
        }, // #1e2028
        idle: Color {
            r: 22.0 / 255.0,
            g: 23.0 / 255.0,
            b: 29.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 32.0 / 255.0,
            g: 34.0 / 255.0,
            b: 43.0 / 255.0,
            a: 1.0,
        }, // #20222b
        pressed: Color {
            r: 42.0 / 255.0,
            g: 45.0 / 255.0,
            b: 57.0 / 255.0,
            a: 1.0,
        }, // #2a2d39
        active: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }, // #ffffff (Pure White radiant action)
        active_hover: Color {
            r: 240.0 / 255.0,
            g: 240.0 / 255.0,
            b: 245.0 / 255.0,
            a: 1.0,
        }, // #f0f0f5
        border_faint: Color::TRANSPARENT,
        border: Color::TRANSPARENT,
        border_strong: Color::TRANSPARENT,
        focus_border: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.60,
        },
        text_primary: Color {
            r: 248.0 / 255.0,
            g: 250.0 / 255.0,
            b: 252.0 / 255.0,
            a: 1.0,
        }, // #f8fafc (Pure White)
        text_secondary: Color {
            r: 148.0 / 255.0,
            g: 163.0 / 255.0,
            b: 184.0 / 255.0,
            a: 1.0,
        }, // #94a3b8
        text_muted: Color {
            r: 100.0 / 255.0,
            g: 116.0 / 255.0,
            b: 139.0 / 255.0,
            a: 1.0,
        }, // #64748b
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
            a: 0.65,
        },
        shadow_key: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.85,
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
            r: 17.0 / 255.0,
            g: 17.0 / 255.0,
            b: 27.0 / 255.0,
            a: 1.0,
        }, // #11111b (Crust)
        surface: Color {
            r: 30.0 / 255.0,
            g: 30.0 / 255.0,
            b: 46.0 / 255.0,
            a: 1.0,
        }, // #1e1e2e (Base)
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
        },
        hovered: Color {
            r: 69.0 / 255.0,
            g: 71.0 / 255.0,
            b: 90.0 / 255.0,
            a: 1.0,
        },
        pressed: Color {
            r: 88.0 / 255.0,
            g: 91.0 / 255.0,
            b: 112.0 / 255.0,
            a: 1.0,
        },
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
        border_faint: Color::TRANSPARENT,
        border: Color::TRANSPARENT,
        border_strong: Color::TRANSPARENT,
        focus_border: Color {
            r: 203.0 / 255.0,
            g: 166.0 / 255.0,
            b: 247.0 / 255.0,
            a: 0.60,
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
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.45,
        },
        shadow_key: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.70,
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
            r: 220.0 / 255.0,
            g: 224.0 / 255.0,
            b: 232.0 / 255.0,
            a: 1.0,
        }, // #dce0e8 (Crust)
        surface: Color {
            r: 239.0 / 255.0,
            g: 241.0 / 255.0,
            b: 245.0 / 255.0,
            a: 1.0,
        }, // #eff1f5 (Base - floating cards)
        surface_subtle: Color {
            r: 204.0 / 255.0,
            g: 208.0 / 255.0,
            b: 218.0 / 255.0,
            a: 1.0,
        }, // #ccd0da (Surface 0)
        surface_elevated: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        idle: Color {
            r: 204.0 / 255.0,
            g: 208.0 / 255.0,
            b: 218.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 188.0 / 255.0,
            g: 192.0 / 255.0,
            b: 204.0 / 255.0,
            a: 1.0,
        },
        pressed: Color {
            r: 172.0 / 255.0,
            g: 176.0 / 255.0,
            b: 190.0 / 255.0,
            a: 1.0,
        },
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
        border_faint: Color::TRANSPARENT,
        border: Color::TRANSPARENT,
        border_strong: Color::TRANSPARENT,
        focus_border: Color {
            r: 136.0 / 255.0,
            g: 57.0 / 255.0,
            b: 239.0 / 255.0,
            a: 0.60,
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
            a: 0.06,
        },
        shadow_key: Color {
            r: 76.0 / 255.0,
            g: 79.0 / 255.0,
            b: 105.0 / 255.0,
            a: 0.12,
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
            r: 22.0 / 255.0,
            g: 22.0 / 255.0,
            b: 30.0 / 255.0,
            a: 1.0,
        }, // #16161e (Night)
        surface: Color {
            r: 31.0 / 255.0,
            g: 35.0 / 255.0,
            b: 53.0 / 255.0,
            a: 1.0,
        }, // #1f2335 (Storm)
        surface_subtle: Color {
            r: 41.0 / 255.0,
            g: 46.0 / 255.0,
            b: 66.0 / 255.0,
            a: 1.0,
        }, // #292e42
        surface_elevated: Color {
            r: 52.0 / 255.0,
            g: 59.0 / 255.0,
            b: 88.0 / 255.0,
            a: 1.0,
        }, // #343b58
        idle: Color {
            r: 41.0 / 255.0,
            g: 46.0 / 255.0,
            b: 66.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 52.0 / 255.0,
            g: 59.0 / 255.0,
            b: 88.0 / 255.0,
            a: 1.0,
        },
        pressed: Color {
            r: 65.0 / 255.0,
            g: 74.0 / 255.0,
            b: 110.0 / 255.0,
            a: 1.0,
        },
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
        border_faint: Color::TRANSPARENT,
        border: Color::TRANSPARENT,
        border_strong: Color::TRANSPARENT,
        focus_border: Color {
            r: 122.0 / 255.0,
            g: 162.0 / 255.0,
            b: 247.0 / 255.0,
            a: 0.60,
        },
        text_primary: Color {
            r: 192.0 / 255.0,
            g: 202.0 / 255.0,
            b: 245.0 / 255.0,
            a: 1.0,
        }, // #c0caf5
        text_secondary: Color {
            r: 154.0 / 255.0,
            g: 165.0 / 255.0,
            b: 206.0 / 255.0,
            a: 1.0,
        }, // #9aa5ce
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
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.45,
        },
        shadow_key: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.70,
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
            r: 29.0 / 255.0,
            g: 32.0 / 255.0,
            b: 33.0 / 255.0,
            a: 1.0,
        }, // #1d2021 (Hard Dark)
        surface: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 1.0,
        }, // #282828 (Medium Dark)
        surface_subtle: Color {
            r: 50.0 / 255.0,
            g: 48.0 / 255.0,
            b: 47.0 / 255.0,
            a: 1.0,
        }, // #32302f
        surface_elevated: Color {
            r: 60.0 / 255.0,
            g: 56.0 / 255.0,
            b: 54.0 / 255.0,
            a: 1.0,
        }, // #3c3836
        idle: Color {
            r: 50.0 / 255.0,
            g: 48.0 / 255.0,
            b: 47.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 60.0 / 255.0,
            g: 56.0 / 255.0,
            b: 54.0 / 255.0,
            a: 1.0,
        },
        pressed: Color {
            r: 80.0 / 255.0,
            g: 73.0 / 255.0,
            b: 69.0 / 255.0,
            a: 1.0,
        },
        active: Color {
            r: 250.0 / 255.0,
            g: 189.0 / 255.0,
            b: 47.0 / 255.0,
            a: 1.0,
        }, // #fabd2f (Yellow)
        active_hover: Color {
            r: 254.0 / 255.0,
            g: 128.0 / 255.0,
            b: 25.0 / 255.0,
            a: 1.0,
        }, // #fe8019 (Orange)
        border_faint: Color::TRANSPARENT,
        border: Color::TRANSPARENT,
        border_strong: Color::TRANSPARENT,
        focus_border: Color {
            r: 250.0 / 255.0,
            g: 189.0 / 255.0,
            b: 47.0 / 255.0,
            a: 0.60,
        },
        text_primary: Color {
            r: 235.0 / 255.0,
            g: 219.0 / 255.0,
            b: 178.0 / 255.0,
            a: 1.0,
        }, // #ebdbb2
        text_secondary: Color {
            r: 213.0 / 255.0,
            g: 196.0 / 255.0,
            b: 161.0 / 255.0,
            a: 1.0,
        }, // #d5c4a1
        text_muted: Color {
            r: 146.0 / 255.0,
            g: 131.0 / 255.0,
            b: 116.0 / 255.0,
            a: 1.0,
        }, // #928374
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
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.45,
        },
        shadow_key: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.70,
        },
        selection: Color {
            r: 250.0 / 255.0,
            g: 189.0 / 255.0,
            b: 47.0 / 255.0,
            a: 0.25,
        },
    };

    /// Gruvbox Light -- Warm retro groove light paper and rust accent.
    pub const GRUVBOX_LIGHT: Theme = Theme {
        name: "gruvbox-light",
        is_dark: false,
        bg_canvas: Color {
            r: 235.0 / 255.0,
            g: 219.0 / 255.0,
            b: 178.0 / 255.0,
            a: 1.0,
        }, // #ebdbb2
        surface: Color {
            r: 251.0 / 255.0,
            g: 241.0 / 255.0,
            b: 199.0 / 255.0,
            a: 1.0,
        }, // #fbf1c7 (Floating cards)
        surface_subtle: Color {
            r: 242.0 / 255.0,
            g: 229.0 / 255.0,
            b: 188.0 / 255.0,
            a: 1.0,
        }, // #f2e5bc
        surface_elevated: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        idle: Color {
            r: 242.0 / 255.0,
            g: 229.0 / 255.0,
            b: 188.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 224.0 / 255.0,
            g: 209.0 / 255.0,
            b: 168.0 / 255.0,
            a: 1.0,
        },
        pressed: Color {
            r: 204.0 / 255.0,
            g: 189.0 / 255.0,
            b: 148.0 / 255.0,
            a: 1.0,
        },
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
        },
        border_faint: Color::TRANSPARENT,
        border: Color::TRANSPARENT,
        border_strong: Color::TRANSPARENT,
        focus_border: Color {
            r: 175.0 / 255.0,
            g: 58.0 / 255.0,
            b: 3.0 / 255.0,
            a: 0.60,
        },
        text_primary: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 1.0,
        }, // #282828
        text_secondary: Color {
            r: 80.0 / 255.0,
            g: 73.0 / 255.0,
            b: 69.0 / 255.0,
            a: 1.0,
        }, // #504945
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
        },
        warning: Color {
            r: 181.0 / 255.0,
            g: 118.0 / 255.0,
            b: 20.0 / 255.0,
            a: 1.0,
        },
        error: Color {
            r: 157.0 / 255.0,
            g: 0.0 / 255.0,
            b: 6.0 / 255.0,
            a: 1.0,
        },
        shadow_ambient: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 0.06,
        },
        shadow_key: Color {
            r: 40.0 / 255.0,
            g: 40.0 / 255.0,
            b: 40.0 / 255.0,
            a: 0.12,
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
            r: 36.0 / 255.0,
            g: 41.0 / 255.0,
            b: 51.0 / 255.0,
            a: 1.0,
        }, // #242933 (Deep Polar Night)
        surface: Color {
            r: 46.0 / 255.0,
            g: 52.0 / 255.0,
            b: 64.0 / 255.0,
            a: 1.0,
        }, // #2e3440 (Nord 0 - floating cards)
        surface_subtle: Color {
            r: 59.0 / 255.0,
            g: 66.0 / 255.0,
            b: 82.0 / 255.0,
            a: 1.0,
        }, // #3b4252 (Nord 1)
        surface_elevated: Color {
            r: 67.0 / 255.0,
            g: 76.0 / 255.0,
            b: 94.0 / 255.0,
            a: 1.0,
        }, // #434c5e (Nord 2)
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
        border_faint: Color::TRANSPARENT,
        border: Color::TRANSPARENT,
        border_strong: Color::TRANSPARENT,
        focus_border: Color {
            r: 136.0 / 255.0,
            g: 192.0 / 255.0,
            b: 208.0 / 255.0,
            a: 0.60,
        },
        text_primary: Color {
            r: 236.0 / 255.0,
            g: 239.0 / 255.0,
            b: 244.0 / 255.0,
            a: 1.0,
        }, // #eceff4 (Snow Storm)
        text_secondary: Color {
            r: 216.0 / 255.0,
            g: 222.0 / 255.0,
            b: 233.0 / 255.0,
            a: 1.0,
        }, // #d8dee9
        text_muted: Color {
            r: 123.0 / 255.0,
            g: 136.0 / 255.0,
            b: 161.0 / 255.0,
            a: 1.0,
        }, // #7b88a1
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
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.45,
        },
        shadow_key: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.70,
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
            r: 20.0 / 255.0,
            g: 18.0 / 255.0,
            b: 30.0 / 255.0,
            a: 1.0,
        }, // #14121e (Deep Base)
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
            r: 50.0 / 255.0,
            g: 46.0 / 255.0,
            b: 71.0 / 255.0,
            a: 1.0,
        }, // #322e47
        idle: Color {
            r: 38.0 / 255.0,
            g: 35.0 / 255.0,
            b: 58.0 / 255.0,
            a: 1.0,
        },
        hovered: Color {
            r: 50.0 / 255.0,
            g: 46.0 / 255.0,
            b: 71.0 / 255.0,
            a: 1.0,
        },
        pressed: Color {
            r: 64.0 / 255.0,
            g: 59.0 / 255.0,
            b: 90.0 / 255.0,
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
        border_faint: Color::TRANSPARENT,
        border: Color::TRANSPARENT,
        border_strong: Color::TRANSPARENT,
        focus_border: Color {
            r: 235.0 / 255.0,
            g: 111.0 / 255.0,
            b: 146.0 / 255.0,
            a: 0.60,
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
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.45,
        },
        shadow_key: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.70,
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
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 12.0,
            padding: [16.0, 9.0],
            shadow: Some(ShadowStyle {
                color: self.shadow_ambient,
                blur_radius: 8.0,
                offset: [0.0, 2.0],
            }),
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
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 12.0,
            padding: [16.0, 9.0],
            shadow: Some(ShadowStyle {
                color: self.active.with_alpha(0.28),
                blur_radius: 12.0,
                offset: [0.0, 3.0],
            }),
            sharp: false,
        }
    }

    /// Constructs an outline button style customized for this theme.
    pub fn outline_button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(self.surface_subtle),
            hover_fill: Fill::Solid(self.hovered),
            pressed_fill: Fill::Solid(self.pressed),
            text_color: self.text_primary,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 12.0,
            padding: [16.0, 9.0],
            shadow: Some(ShadowStyle {
                color: self.shadow_ambient,
                blur_radius: 6.0,
                offset: [0.0, 1.5],
            }),
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
            corner_radius: 12.0,
            padding: [16.0, 9.0],
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
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 12.0,
            padding: [16.0, 9.0],
            shadow: Some(ShadowStyle {
                color: self.error.with_alpha(0.30),
                blur_radius: 12.0,
                offset: [0.0, 3.0],
            }),
            sharp: false,
        }
    }

    /// Constructs a card container style customized for this theme.
    pub fn card_style(&self) -> CardStyle {
        CardStyle {
            fill: Fill::Solid(self.surface),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 16.0,
            padding: [22.0, 22.0],
            shadow: Some(ShadowStyle {
                color: self.shadow_ambient,
                blur_radius: 24.0,
                offset: [0.0, 6.0],
            }),
        }
    }

    /// Constructs an inset subtle card style customized for this theme.
    pub fn card_subtle_style(&self) -> CardStyle {
        CardStyle {
            fill: Fill::Solid(self.surface_subtle),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 14.0,
            padding: [18.0, 18.0],
            shadow: None,
        }
    }

    /// Constructs an elevated card style customized for this theme.
    pub fn card_elevated_style(&self) -> CardStyle {
        CardStyle {
            fill: Fill::Solid(self.surface_elevated),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 20.0,
            padding: [24.0, 24.0],
            shadow: Some(ShadowStyle {
                color: self.shadow_key,
                blur_radius: 32.0,
                offset: [0.0, 10.0],
            }),
        }
    }

    /// Constructs a checkbox style customized for this theme.
    pub fn checkbox_style(&self) -> CheckboxStyle {
        CheckboxStyle {
            fill: Fill::Solid(self.surface_subtle),
            hover_fill: Fill::Solid(self.hovered),
            checked_fill: Fill::Solid(self.active),
            check_color: if self.is_dark && self.active.r > 0.8 && self.active.g > 0.8 {
                Color::BLACK
            } else {
                Color::WHITE
            },
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 7.0,
            shadow: Some(ShadowStyle {
                color: self.shadow_ambient,
                blur_radius: 6.0,
                offset: [0.0, 2.0],
            }),
            sharp: false,
        }
    }

    /// Constructs a switch toggle style customized for this theme.
    pub fn switch_style(&self) -> SwitchStyle {
        SwitchStyle {
            track_off_fill: Fill::Solid(self.surface_subtle),
            track_on_fill: Fill::Solid(self.active),
            thumb_fill: Fill::Solid(Color::WHITE),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 999.0,
            shadow: Some(ShadowStyle {
                color: self.shadow_ambient,
                blur_radius: 8.0,
                offset: [0.0, 2.0],
            }),
        }
    }

    /// Constructs a slider style customized for this theme.
    pub fn slider_style(&self) -> SliderStyle {
        SliderStyle {
            track_fill: Fill::Solid(self.surface_subtle),
            filled_fill: Fill::Solid(self.active),
            thumb_fill: Fill::Solid(Color::WHITE),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            track_height: 6.0,
            thumb_size: 18.0,
            shadow: Some(ShadowStyle {
                color: self.shadow_ambient,
                blur_radius: 8.0,
                offset: [0.0, 2.0],
            }),
        }
    }

    /// Constructs a text input style customized for this theme.
    pub fn input_style(&self) -> TextInputStyle {
        TextInputStyle {
            fill: Fill::Solid(self.surface_subtle),
            text_color: self.text_primary,
            placeholder_color: self.text_muted,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            focus_border_color: self.focus_border,
            corner_radius: 12.0,
            padding: [14.0, 10.0],
            selection_color: self.selection,
            cursor_color: self.active,
            shadow: None,
            sharp: false,
        }
    }

    /// Constructs a text area editor style customized for this theme.
    pub fn text_area_style(&self) -> TextAreaStyle {
        TextAreaStyle {
            fill: Fill::Solid(self.surface_subtle),
            text_color: self.text_primary,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            focus_border_color: self.focus_border,
            corner_radius: 12.0,
            padding: [14.0, 12.0],
            selection_color: self.selection,
            cursor_color: self.active,
            thumb_fill: Fill::Solid(self.text_muted.with_alpha(0.35)),
            thumb_dragging_fill: Fill::Solid(self.text_muted.with_alpha(0.65)),
            shadow: None,
            sharp: false,
        }
    }
}
