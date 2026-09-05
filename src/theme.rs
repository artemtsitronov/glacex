use crate::color::Color;

/// Central design token system for Glacex.
///
/// Inspired by the surface hierarchy of Linear, the restraint of Vercel/Geist,
/// and the color semantics of shadcn/ui. Every value is intentional.
/// All interactive states, surfaces, borders, shadows, and typography
/// follow a consistent 4px-grid spacing and a 4-step surface ladder.
pub struct Theme;

impl Theme {
    // -------------------------------------------------------------------------
    // Surface Ladder (4-step, dark-first)
    // Depth is communicated through surface lightness, not shadow weight.
    // -------------------------------------------------------------------------

    /// Root window background. Near-black with a faint blue undertone to
    /// avoid harsh optical halation against white text (Linear pattern).
    pub const BG_CANVAS: Color = Color {
        r: 9.0 / 255.0,
        g: 9.0 / 255.0,
        b: 11.0 / 255.0, // #09090b
        a: 1.0,
    };

    /// Standard panel / card surface. One step above canvas.
    pub const SURFACE: Color = Color {
        r: 15.0 / 255.0,
        g: 15.0 / 255.0,
        b: 18.0 / 255.0, // #0f0f12
        a: 1.0,
    };

    /// Secondary / subtle surface for inset or grouped content.
    pub const SURFACE_SUBTLE: Color = Color {
        r: 20.0 / 255.0,
        g: 20.0 / 255.0,
        b: 24.0 / 255.0, // #141418
        a: 1.0,
    };

    /// Elevated surface for tooltips, popovers, and modals.
    pub const SURFACE_ELEVATED: Color = Color {
        r: 28.0 / 255.0,
        g: 28.0 / 255.0,
        b: 34.0 / 255.0, // #1c1c22
        a: 1.0,
    };

    // -------------------------------------------------------------------------
    // Interactive Control States
    // -------------------------------------------------------------------------

    /// Idle / resting control background.
    pub const IDLE: Color = Color {
        r: 24.0 / 255.0,
        g: 24.0 / 255.0,
        b: 28.0 / 255.0, // #18181c
        a: 1.0,
    };

    /// Hovered control background. Subtle lift without jarring contrast jump.
    pub const HOVERED: Color = Color {
        r: 35.0 / 255.0,
        g: 35.0 / 255.0,
        b: 41.0 / 255.0, // #232329
        a: 1.0,
    };

    /// Pressed control background. Deeper than hover, lighter than active accent.
    pub const PRESSED: Color = Color {
        r: 45.0 / 255.0,
        g: 44.0 / 255.0,
        b: 54.0 / 255.0, // #2d2c36
        a: 1.0,
    };

    /// Primary accent. Electric indigo -- used only for active states,
    /// focus rings, and primary CTAs. One chromatic accent, used sparingly.
    pub const ACTIVE: Color = Color {
        r: 79.0 / 255.0,
        g: 70.0 / 255.0,
        b: 229.0 / 255.0, // #4f46e5 -- Indigo 600
        a: 1.0,
    };

    /// Lighter accent for hover over an already-active element.
    pub const ACTIVE_HOVER: Color = Color {
        r: 99.0 / 255.0,
        g: 102.0 / 255.0,
        b: 241.0 / 255.0, // #6366f1 -- Indigo 500
        a: 1.0,
    };

    // -------------------------------------------------------------------------
    // Borders (semi-transparent white, Linear pattern)
    // -------------------------------------------------------------------------

    /// Ultra-subtle hairline. Use for passive structural dividers.
    pub const BORDER_FAINT: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.05,
    };

    /// Standard hairline border. Use for most controls in resting state.
    pub const BORDER: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.08,
    };

    /// Stronger border for hover/focus state contrast boost.
    pub const BORDER_STRONG: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.16,
    };

    /// Focus ring color. Full indigo, high saturation for keyboard accessibility.
    pub const FOCUS_BORDER: Color = Color {
        r: 99.0 / 255.0,
        g: 102.0 / 255.0,
        b: 241.0 / 255.0, // #6366f1
        a: 1.0,
    };

    // -------------------------------------------------------------------------
    // Typography
    // -------------------------------------------------------------------------

    /// Primary body text. Off-white reduces eye strain vs pure #fff.
    pub const TEXT_PRIMARY: Color = Color {
        r: 242.0 / 255.0,
        g: 242.0 / 255.0,
        b: 245.0 / 255.0, // #f2f2f5
        a: 1.0,
    };

    /// Secondary / supporting text. Reduced opacity for visual hierarchy.
    pub const TEXT_SECONDARY: Color = Color {
        r: 161.0 / 255.0,
        g: 161.0 / 255.0,
        b: 170.0 / 255.0, // Zinc 400
        a: 1.0,
    };

    /// Muted / placeholder text. Lowest readable hierarchy level.
    pub const TEXT_MUTED: Color = Color {
        r: 113.0 / 255.0,
        g: 113.0 / 255.0,
        b: 122.0 / 255.0, // Zinc 500
        a: 1.0,
    };

    /// Alias kept for backward compatibility.
    pub const TEXT_DIM: Color = Self::TEXT_MUTED;

    // -------------------------------------------------------------------------
    // Semantic Status Colors
    // -------------------------------------------------------------------------

    /// Success / positive. Emerald 500 -- legible on dark.
    pub const SUCCESS: Color = Color {
        r: 34.0 / 255.0,
        g: 197.0 / 255.0,
        b: 94.0 / 255.0,
        a: 1.0,
    };

    /// Warning / caution. Amber 500.
    pub const WARNING: Color = Color {
        r: 245.0 / 255.0,
        g: 158.0 / 255.0,
        b: 11.0 / 255.0,
        a: 1.0,
    };

    /// Error / destructive. Rose 500. Slightly cooler than a harsh red.
    pub const ERROR: Color = Color {
        r: 244.0 / 255.0,
        g: 63.0 / 255.0,
        b: 94.0 / 255.0,
        a: 1.0,
    };

    // -------------------------------------------------------------------------
    // Shadow Tokens
    // Glacex uses two-layer shadows: ambient (large, soft) + key (small, crisp).
    // -------------------------------------------------------------------------

    /// Ambient shadow layer -- large, very soft, low opacity.
    pub const SHADOW_AMBIENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.28,
    };

    /// Key light shadow layer -- smaller, tighter, slightly more opaque.
    pub const SHADOW_KEY: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.40,
    };

    /// Alias for component-level shadows (cards, panels).
    pub const SURFACE_SHADOW: Color = Self::SHADOW_AMBIENT;

    /// Alias for heavier button / control shadows.
    pub const SHADOW: Color = Self::SHADOW_KEY;

    // -------------------------------------------------------------------------
    // Selection
    // -------------------------------------------------------------------------

    /// Text selection highlight. Indigo at 30% alpha -- visible but not jarring.
    pub const SELECTION: Color = Color {
        r: 99.0 / 255.0,
        g: 102.0 / 255.0,
        b: 241.0 / 255.0,
        a: 0.30,
    };

    // -------------------------------------------------------------------------
    // Sizing Constants (4px grid, control heights)
    // -------------------------------------------------------------------------

    /// Height of small controls (badges, dense inputs): 28px.
    pub const CONTROL_HEIGHT_SM: f32 = 28.0;
    /// Height of standard controls (buttons, inputs): 36px.
    pub const CONTROL_HEIGHT_MD: f32 = 36.0;
    /// Height of large / touch-friendly controls: 44px.
    pub const CONTROL_HEIGHT_LG: f32 = 44.0;

    /// Standard horizontal padding inside controls: 12px.
    pub const CONTROL_PAD_X: f32 = 12.0;
    /// Standard vertical padding inside controls: 8px.
    pub const CONTROL_PAD_Y: f32 = 8.0;

    // -------------------------------------------------------------------------
    // Radius Scale (shadcn-inspired, pixel values)
    // -------------------------------------------------------------------------

    /// Extra-small radius for inner/inset elements: 3px.
    pub const RADIUS_XS: f32 = 3.0;
    /// Small radius for checkboxes, tags, small badges: 5px.
    pub const RADIUS_SM: f32 = 5.0;
    /// Default radius for buttons, inputs, switches: 7px.
    pub const RADIUS_MD: f32 = 7.0;
    /// Large radius for cards, panels, modals: 12px.
    pub const RADIUS_LG: f32 = 12.0;
    /// Full pill / circular: effectively infinite.
    pub const RADIUS_FULL: f32 = 9999.0;

    // -------------------------------------------------------------------------
    // Spacing Scale (4px base grid)
    // -------------------------------------------------------------------------

    pub const SPACE_1: f32 = 4.0;
    pub const SPACE_2: f32 = 8.0;
    pub const SPACE_3: f32 = 12.0;
    pub const SPACE_4: f32 = 16.0;
    pub const SPACE_6: f32 = 24.0;
    pub const SPACE_8: f32 = 32.0;

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    /// Returns the control fill for the given interaction state.
    /// Includes a distinct PRESSED state between hovered and active.
    pub fn state_color(active: bool, pressed: bool, hovered: bool) -> Color {
        if active {
            Self::ACTIVE
        } else if pressed {
            Self::PRESSED
        } else if hovered {
            Self::HOVERED
        } else {
            Self::IDLE
        }
    }
}
