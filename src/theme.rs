use crate::Token;
use crate::color::Color;
use crate::shadow::ShadowStyle;

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

    pub metrics: Metrics,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metrics {
    pub control_height_sm: f32,
    pub control_height_md: f32,
    pub control_height_lg: f32,

    pub control_pad_x: f32,
    pub control_pad_y: f32,

    pub radius_xs: f32,
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    pub radius_full: f32,

    pub space_1: f32,
    pub space_2: f32,
    pub space_3: f32,
    pub space_4: f32,
    pub space_6: f32,
    pub space_8: f32,
}

impl Metrics {
    pub const DEFAULT: Metrics = Metrics {
        control_height_sm: 28.0,
        control_height_md: 36.0,
        control_height_lg: 44.0,
        control_pad_x: 14.0,
        control_pad_y: 9.0,
        radius_xs: 3.0,
        radius_sm: 5.0,
        radius_md: 7.0,
        radius_lg: 12.0,
        radius_full: 9999.0,
        space_1: 4.0,
        space_2: 8.0,
        space_3: 12.0,
        space_4: 16.0,
        space_6: 24.0,
        space_8: 32.0,
    };
}

impl Default for Metrics {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Default for Theme {
    #[inline]
    fn default() -> Self {
        Self::LIGHT
    }
}

impl Theme {
    pub const LIGHT: Theme = Theme {
        name: "light",
        is_dark: false,
        bg_canvas: Color::rgb(246, 247, 250),
        surface: Color::rgb(255, 255, 255),
        surface_subtle: Color::rgb(235, 237, 242),
        surface_elevated: Color::rgb(255, 255, 255),
        idle: Color::rgb(235, 237, 242),
        hovered: Color::rgb(222, 225, 233),
        pressed: Color::rgb(206, 210, 222),
        active: Color::rgb(65, 68, 196),
        active_hover: Color::rgb(89, 92, 214),
        border_faint: Color::rgba(30, 33, 54, 0.05),
        border: Color::rgba(30, 33, 54, 0.10),
        border_strong: Color::rgba(30, 33, 54, 0.18),
        focus_border: Color::rgba(65, 68, 196, 0.85),
        text_primary: Color::rgb(23, 25, 38),
        text_secondary: Color::rgb(83, 88, 107),
        text_muted: Color::rgb(137, 142, 158),
        success: Color::rgb(34, 139, 101),
        warning: Color::rgb(180, 120, 20),
        error: Color::rgb(196, 48, 66),
        shadow_ambient: Color::rgba(30, 33, 54, 0.04),
        shadow_key: Color::rgba(30, 33, 54, 0.08),
        selection: Color::rgba(65, 68, 196, 0.16),
        metrics: Metrics::DEFAULT,
    };

    pub const DARK: Theme = Theme {
        name: "dark",
        is_dark: true,
        bg_canvas: Color::rgb(10, 11, 16),
        surface: Color::rgb(17, 19, 26),
        surface_subtle: Color::rgb(24, 27, 36),
        surface_elevated: Color::rgb(32, 36, 48),
        idle: Color::rgb(24, 27, 36),
        hovered: Color::rgb(34, 38, 50),
        pressed: Color::rgb(44, 49, 64),
        active: Color::rgb(13, 148, 136),
        active_hover: Color::rgb(45, 212, 191),
        border_faint: Color::rgba(203, 213, 225, 0.06),
        border: Color::rgba(203, 213, 225, 0.12),
        border_strong: Color::rgba(203, 213, 225, 0.22),
        focus_border: Color::rgb(45, 212, 191),
        text_primary: Color::rgb(230, 234, 240),
        text_secondary: Color::rgb(163, 171, 184),
        text_muted: Color::rgb(105, 113, 128),
        success: Color::rgb(74, 222, 128),
        warning: Color::rgb(250, 204, 21),
        error: Color::rgb(248, 113, 113),
        shadow_ambient: Color::rgba(0, 0, 0, 0.35),
        shadow_key: Color::rgba(0, 0, 0, 0.55),
        selection: Color::rgba(45, 212, 191, 0.22),
        metrics: Metrics::DEFAULT,
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
        metrics: Metrics::DEFAULT,
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
        metrics: Metrics::DEFAULT,
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
        metrics: Metrics::DEFAULT,
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
        metrics: Metrics::DEFAULT,
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
        metrics: Metrics::DEFAULT,
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
        metrics: Metrics::DEFAULT,
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
        metrics: Metrics::DEFAULT,
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
            "light" | "white" => Some(Self::LIGHT),
            "dark" | "linear" => Some(Self::DARK),
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

    pub fn with_metrics(mut self, metrics: Metrics) -> Self {
        self.metrics = metrics;
        self
    }

    pub fn shadow_sm(&self) -> [ShadowStyle; 2] {
        [
            ShadowStyle::new(self.shadow_ambient, 3.0, [0.0, 1.0]),
            ShadowStyle::new(self.shadow_key, 6.0, [0.0, 2.0]),
        ]
    }

    pub fn shadow_md(&self) -> [ShadowStyle; 2] {
        [
            ShadowStyle::new(self.shadow_ambient, 12.0, [0.0, 3.0]),
            ShadowStyle::new(self.shadow_key, 4.0, [0.0, 2.0]),
        ]
    }

    pub fn shadow_lg(&self) -> [ShadowStyle; 2] {
        [
            ShadowStyle::new(self.shadow_ambient, 24.0, [0.0, 6.0]),
            ShadowStyle::new(self.shadow_key, 8.0, [0.0, 3.0]),
        ]
    }

    // some dark themes have a near white active, white text would vanish on it
    pub fn on_active(&self) -> Color {
        if self.is_dark && self.active.r > 0.8 && self.active.g > 0.8 {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    pub fn token_color(&self, token: Token) -> Color {
        match token {
            Token::Keyword => self.active,
            Token::Function => self.active_hover,
            Token::String => self.success,
            Token::Number => self.warning,
            Token::Comment => self.text_muted,
            Token::Operator => self.text_secondary,
            Token::Type => self.warning,
            // Token::Plain => self.text_primary,
            _ => self.text_primary,
        }
    }
}
