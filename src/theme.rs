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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    pub name: &'static str,
    pub is_dark: bool,

    pub bg_canvas: Color,
    pub surface: Color,
    pub surface_subtle: Color,
    pub surface_elevated: Color,

    pub idle: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub active: Color,
    pub active_hover: Color,

    pub border_faint: Color,
    pub border: Color,
    pub border_strong: Color,
    pub focus_border: Color,

    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,

    pub success: Color,
    pub warning: Color,
    pub error: Color,

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
    pub const LIGHT: Theme = Theme {
        name: "shadcn-light",
        is_dark: false,
        bg_canvas: Color::rgb(255, 255, 255),
        surface: Color::rgb(255, 255, 255),
        surface_subtle: Color::rgb(244, 244, 245),
        surface_elevated: Color::rgb(255, 255, 255),
        idle: Color::rgb(244, 244, 245),
        hovered: Color::rgb(228, 228, 228),
        pressed: Color::rgb(212, 212, 212),
        active: Color::rgb(228, 228, 228),
        active_hover: Color::rgb(212, 212, 212),
        border_faint: Color::rgb(0, 0, 0),
        border: Color::rgb(0, 0, 0),
        border_strong: Color::rgb(0, 0, 0),
        focus_border: Color::rgb(24, 24, 27),
        text_primary: Color::rgb(9, 9, 11),
        text_secondary: Color::rgb(113, 113, 122),
        text_muted: Color::rgb(161, 161, 170),
        success: Color::rgb(22, 163, 74),
        warning: Color::rgb(217, 119, 6),
        error: Color::rgb(225, 29, 72),
        shadow_ambient: Color::rgb(0, 0, 0),
        shadow_key: Color::rgb(0, 0, 0),
        selection: Color::rgb(0, 0, 0),
    };

    pub const DARK: Theme = Theme {
        name: "shadcn-dark",
        is_dark: true,
        bg_canvas: Color::rgb(0, 0, 0),
        surface: Color::rgb(9, 9, 11),
        surface_subtle: Color::rgb(18, 18, 21),
        surface_elevated: Color::rgb(24, 24, 27),
        idle: Color::rgb(18, 18, 21),
        hovered: Color::rgb(32, 32, 36),
        pressed: Color::rgb(39, 39, 42),
        active: Color::rgb(250, 250, 250),
        active_hover: Color::rgb(228, 228, 231),
        border_faint: Color::rgba(255, 255, 255, 0.08),
        border: Color::rgba(255, 255, 255, 0.14),
        border_strong: Color::rgba(255, 255, 255, 0.25),
        focus_border: Color::rgba(250, 250, 250, 0.85),
        text_primary: Color::rgb(250, 250, 250),
        text_secondary: Color::rgb(161, 161, 170),
        text_muted: Color::rgb(113, 113, 122),
        success: Color::rgb(34, 197, 94),
        warning: Color::rgb(245, 158, 11),
        error: Color::rgb(244, 63, 94),
        shadow_ambient: Color::rgba(0, 0, 0, 0.28),
        shadow_key: Color::rgba(0, 0, 0, 0.40),
        selection: Color::rgba(99, 102, 241, 0.30),
    };

    pub const CATPPUCCIN_MOCHA: Theme = Theme {
        name: "catppuccin-mocha",
        is_dark: true,
        bg_canvas: Color::rgb(30, 30, 46),
        surface: Color::rgb(24, 24, 37),
        surface_subtle: Color::rgb(49, 50, 68),
        surface_elevated: Color::rgb(69, 71, 90),
        idle: Color::rgb(49, 50, 68),
        hovered: Color::rgb(69, 71, 90),
        pressed: Color::rgb(88, 91, 112),
        active: Color::rgb(203, 166, 247),
        active_hover: Color::rgb(180, 190, 254),
        border_faint: Color::rgba(205, 214, 244, 0.06),
        border: Color::rgba(205, 214, 244, 0.10),
        border_strong: Color::rgba(205, 214, 244, 0.20),
        focus_border: Color::rgb(203, 166, 247),
        text_primary: Color::rgb(205, 214, 244),
        text_secondary: Color::rgb(166, 173, 200),
        text_muted: Color::rgb(108, 112, 134),
        success: Color::rgb(166, 227, 161),
        warning: Color::rgb(249, 226, 175),
        error: Color::rgb(243, 139, 168),
        shadow_ambient: Color::rgba(17, 17, 27, 0.35),
        shadow_key: Color::rgba(17, 17, 27, 0.55),
        selection: Color::rgba(203, 166, 247, 0.25),
    };

    pub const CATPPUCCIN_LATTE: Theme = Theme {
        name: "catppuccin-latte",
        is_dark: false,
        bg_canvas: Color::rgb(239, 241, 245),
        surface: Color::rgb(255, 255, 255),
        surface_subtle: Color::rgb(230, 233, 239),
        surface_elevated: Color::rgb(255, 255, 255),
        idle: Color::rgb(230, 233, 239),
        hovered: Color::rgb(204, 208, 218),
        pressed: Color::rgb(188, 192, 204),
        active: Color::rgb(136, 57, 239),
        active_hover: Color::rgb(114, 135, 253),
        border_faint: Color::rgba(76, 79, 105, 0.05),
        border: Color::rgba(76, 79, 105, 0.10),
        border_strong: Color::rgba(76, 79, 105, 0.20),
        focus_border: Color::rgba(136, 57, 239, 0.90),
        text_primary: Color::rgb(76, 79, 105),
        text_secondary: Color::rgb(108, 111, 133),
        text_muted: Color::rgb(156, 160, 176),
        success: Color::rgb(64, 160, 43),
        warning: Color::rgb(223, 142, 29),
        error: Color::rgb(210, 15, 57),
        shadow_ambient: Color::rgba(76, 79, 105, 0.04),
        shadow_key: Color::rgba(76, 79, 105, 0.08),
        selection: Color::rgba(136, 57, 239, 0.15),
    };

    pub const TOKYO_NIGHT: Theme = Theme {
        name: "tokyo-night",
        is_dark: true,
        bg_canvas: Color::rgb(26, 27, 38),
        surface: Color::rgb(36, 40, 59),
        surface_subtle: Color::rgb(31, 35, 53),
        surface_elevated: Color::rgb(41, 46, 66),
        idle: Color::rgb(36, 40, 59),
        hovered: Color::rgb(47, 53, 77),
        pressed: Color::rgb(59, 66, 97),
        active: Color::rgb(122, 162, 247),
        active_hover: Color::rgb(187, 154, 247),
        border_faint: Color::rgba(192, 202, 245, 0.06),
        border: Color::rgba(192, 202, 245, 0.10),
        border_strong: Color::rgba(192, 202, 245, 0.20),
        focus_border: Color::rgb(122, 162, 247),
        text_primary: Color::rgb(192, 202, 245),
        text_secondary: Color::rgb(169, 177, 214),
        text_muted: Color::rgb(86, 95, 137),
        success: Color::rgb(158, 206, 106),
        warning: Color::rgb(224, 175, 104),
        error: Color::rgb(247, 118, 142),
        shadow_ambient: Color::rgba(15, 15, 23, 0.35),
        shadow_key: Color::rgba(15, 15, 23, 0.55),
        selection: Color::rgba(122, 162, 247, 0.25),
    };

    pub const GRUVBOX_DARK: Theme = Theme {
        name: "gruvbox-dark",
        is_dark: true,
        bg_canvas: Color::rgb(40, 40, 40),
        surface: Color::rgb(50, 48, 47),
        surface_subtle: Color::rgb(60, 56, 54),
        surface_elevated: Color::rgb(80, 73, 69),
        idle: Color::rgb(60, 56, 54),
        hovered: Color::rgb(80, 73, 69),
        pressed: Color::rgb(102, 92, 84),
        active: Color::rgb(254, 128, 25),
        active_hover: Color::rgb(250, 189, 47),
        border_faint: Color::rgba(251, 241, 199, 0.06),
        border: Color::rgba(251, 241, 199, 0.10),
        border_strong: Color::rgba(251, 241, 199, 0.20),
        focus_border: Color::rgb(254, 128, 25),
        text_primary: Color::rgb(251, 241, 199),
        text_secondary: Color::rgb(235, 219, 178),
        text_muted: Color::rgb(146, 131, 116),
        success: Color::rgb(184, 187, 38),
        warning: Color::rgb(250, 189, 47),
        error: Color::rgb(251, 73, 52),
        shadow_ambient: Color::rgba(29, 32, 33, 0.35),
        shadow_key: Color::rgba(29, 32, 33, 0.55),
        selection: Color::rgba(254, 128, 25, 0.25),
    };

    pub const GRUVBOX_LIGHT: Theme = Theme {
        name: "gruvbox-light",
        is_dark: false,
        bg_canvas: Color::rgb(251, 241, 199),
        surface: Color::rgb(249, 245, 215),
        surface_subtle: Color::rgb(235, 219, 178),
        surface_elevated: Color::rgb(249, 245, 215),
        idle: Color::rgb(235, 219, 178),
        hovered: Color::rgb(213, 196, 161),
        pressed: Color::rgb(189, 174, 147),
        active: Color::rgb(175, 58, 3),
        active_hover: Color::rgb(214, 93, 14),
        border_faint: Color::rgba(40, 40, 40, 0.06),
        border: Color::rgba(40, 40, 40, 0.12),
        border_strong: Color::rgba(40, 40, 40, 0.22),
        focus_border: Color::rgba(175, 58, 3, 0.90),
        text_primary: Color::rgb(40, 40, 40),
        text_secondary: Color::rgb(60, 56, 54),
        text_muted: Color::rgb(124, 111, 100),
        success: Color::rgb(121, 116, 14),
        warning: Color::rgb(181, 118, 20),
        error: Color::rgb(157, 0, 6),
        shadow_ambient: Color::rgba(40, 40, 40, 0.04),
        shadow_key: Color::rgba(40, 40, 40, 0.08),
        selection: Color::rgba(175, 58, 3, 0.15),
    };

    pub const NORD: Theme = Theme {
        name: "nord",
        is_dark: true,
        bg_canvas: Color::rgb(46, 52, 64),
        surface: Color::rgb(59, 66, 82),
        surface_subtle: Color::rgb(67, 76, 94),
        surface_elevated: Color::rgb(76, 86, 106),
        idle: Color::rgb(59, 66, 82),
        hovered: Color::rgb(67, 76, 94),
        pressed: Color::rgb(76, 86, 106),
        active: Color::rgb(136, 192, 208),
        active_hover: Color::rgb(129, 161, 193),
        border_faint: Color::rgba(216, 222, 233, 0.12),
        border: Color::rgba(216, 222, 233, 0.22),
        border_strong: Color::rgba(216, 222, 233, 0.38),
        focus_border: Color::rgb(136, 192, 208),
        text_primary: Color::rgb(236, 239, 244),
        text_secondary: Color::rgb(216, 222, 233),
        text_muted: Color::rgb(165, 178, 202),
        success: Color::rgb(163, 190, 140),
        warning: Color::rgb(235, 203, 139),
        error: Color::rgb(191, 97, 106),
        shadow_ambient: Color::rgba(36, 41, 51, 0.35),
        shadow_key: Color::rgba(36, 41, 51, 0.55),
        selection: Color::rgba(136, 192, 208, 0.25),
    };

    pub const ROSE_PINE: Theme = Theme {
        name: "rose-pine",
        is_dark: true,
        bg_canvas: Color::rgb(25, 23, 36),
        surface: Color::rgb(31, 29, 46),
        surface_subtle: Color::rgb(38, 35, 58),
        surface_elevated: Color::rgb(42, 40, 62),
        idle: Color::rgb(38, 35, 58),
        hovered: Color::rgb(49, 47, 72),
        pressed: Color::rgb(57, 53, 82),
        active: Color::rgb(235, 111, 146),
        active_hover: Color::rgb(196, 167, 231),
        border_faint: Color::rgba(224, 222, 244, 0.06),
        border: Color::rgba(224, 222, 244, 0.10),
        border_strong: Color::rgba(224, 222, 244, 0.20),
        focus_border: Color::rgb(235, 111, 146),
        text_primary: Color::rgb(224, 222, 244),
        text_secondary: Color::rgb(144, 140, 170),
        text_muted: Color::rgb(110, 106, 134),
        success: Color::rgb(156, 207, 216),
        warning: Color::rgb(246, 193, 119),
        error: Color::rgb(235, 111, 146),
        shadow_ambient: Color::rgba(20, 18, 30, 0.35),
        shadow_key: Color::rgba(20, 18, 30, 0.55),
        selection: Color::rgba(235, 111, 146, 0.25),
    };

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

    pub fn button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(self.surface_subtle),
            hover_fill: Fill::Solid(self.hovered),
            pressed_fill: Fill::Solid(self.pressed),
            text_color: self.text_primary,
            border_width: 1.0,
            border_color: self.border,
            corner_radius: 6.0,
            padding: [14.0, 8.0],
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }

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
            padding: [14.0, 8.0],
            shadow: Some(ShadowStyle {
                color: self.active.with_alpha(0.20),
                blur_radius: 4.0,
                offset: [0.0, 1.5],
            }),
            sharp: false,
        }
    }

    pub fn outline_button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(Color::TRANSPARENT),
            hover_fill: Fill::Solid(self.surface_subtle),
            pressed_fill: Fill::Solid(self.hovered),
            text_color: self.text_primary,
            border_width: 1.0,
            border_color: self.border_strong,
            corner_radius: 6.0,
            padding: [14.0, 8.0],
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }

    pub fn ghost_button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(Color::TRANSPARENT),
            hover_fill: Fill::Solid(self.surface_subtle),
            pressed_fill: Fill::Solid(self.hovered),
            text_color: self.text_secondary,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 6.0,
            padding: [14.0, 8.0],
            shadow: None,
            sharp: false,
        }
    }

    pub fn danger_button_style(&self) -> ButtonStyle {
        ButtonStyle {
            fill: Fill::Solid(self.error),
            hover_fill: Fill::Solid(self.error.lighten(0.08)),
            pressed_fill: Fill::Solid(self.error.darken(0.12)),
            text_color: Color::WHITE,
            border_width: 1.0,
            border_color: self.error.darken(0.15),
            corner_radius: 6.0,
            padding: [14.0, 8.0],
            shadow: Some(ShadowStyle {
                color: self.error.with_alpha(0.25),
                blur_radius: 4.0,
                offset: [0.0, 1.5],
            }),
            sharp: false,
        }
    }

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

    /// A color that stays legible drawn on top of `active`. Most themes'
    /// `active` is dark/saturated enough for white to read fine, but
    /// shadcn-dark's `active` is near-white — fall back to black there so
    /// e.g. a switch thumb or a selected radio dot doesn't disappear.
    pub fn on_active(&self) -> Color {
        if self.is_dark && self.active.r > 0.8 && self.active.g > 0.8 {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    pub fn checkbox_style(&self) -> CheckboxStyle {
        CheckboxStyle {
            fill: Fill::Solid(if self.is_dark {
                self.surface
            } else {
                Color::WHITE
            }),
            hover_fill: Fill::Solid(self.surface_subtle),
            checked_fill: Fill::Solid(self.active),
            check_color: self.on_active(),
            border_width: 1.0,
            border_color: self.border_strong,
            corner_radius: 4.0,
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }

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

    pub fn input_style(&self) -> TextInputStyle {
        TextInputStyle {
            fill: Fill::Solid(self.surface),
            text_color: self.text_primary,
            placeholder_color: self.text_muted,
            border_width: 1.0,
            border_color: self.border_strong,
            focus_border_color: self.focus_border,
            corner_radius: 6.0,
            padding: [10.0, 10.0],
            selection_color: self.selection,
            cursor_color: self.active,
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }

    pub fn text_area_style(&self) -> TextAreaStyle {
        TextAreaStyle {
            fill: Fill::Solid(self.surface),
            text_color: self.text_primary,
            border_width: 1.0,
            border_color: self.border_strong,
            focus_border_color: self.focus_border,
            corner_radius: 6.0,
            padding: [10.0, 10.0],
            selection_color: self.selection,
            cursor_color: self.active,
            thumb_fill: Fill::Solid(self.text_muted.with_alpha(0.35)),
            thumb_dragging_fill: Fill::Solid(self.text_muted.with_alpha(0.65)),
            shadow: Some(self.shadow_sm()[0]),
            sharp: false,
        }
    }
}
