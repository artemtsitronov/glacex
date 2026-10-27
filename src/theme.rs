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

    // secondary
    pub idle: Color,
    pub hovered: Color,
    pub pressed: Color,

    // primary
    pub active: Color,
    pub active_hover: Color,
    pub active_pressed: Color,

    pub accent: Color,
    pub accent_hover: Color,
    pub accent_pressed: Color,

    pub disabled_bg: Color,
    pub disabled_border: Color,
    pub disabled_text: Color,

    pub border_faint: Color,
    pub border: Color,
    pub border_strong: Color,
    pub border_strong_hover: Color,
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

    pub border_width: f32,
    pub focus_ring_width: f32,
    pub focus_ring_offset: f32,

    pub font_size_md: f32,
    pub font_size_sm: f32,
    pub font_size_xs: f32,

    pub control_min_width: f32,
    pub field_pad_x: f32,
    pub icon_size: f32,
    pub icon_stroke: f32,
    pub icon_gap: f32,

    pub checkbox_size: f32,
    pub radio_size: f32,
    pub radio_dot: f32,

    pub switch_width: f32,
    pub switch_height: f32,
    pub switch_thumb_inset: f32,

    pub slider_track: f32,
    pub slider_thumb: f32,
    pub progress_height: f32,

    pub badge_height: f32,
    pub badge_pad_x: f32,

    pub tab_height: f32,
    pub tab_underline: f32,

    pub accordion_row_height: f32,
    pub accordion_pad_x: f32,

    pub card_pad: f32,

    pub scrollbar_thumb: f32,
    pub scrollbar_inset: f32,

    pub textarea_min_height: f32,
    pub textarea_line_height: f32,

    pub select_option_height: f32,
}

impl Metrics {
    // sharp and dense: 32px controls, 4px corners, 2px for small parts
    // (checkbox, badge, scrollbar thumb), 1px borders, 1px focus ring
    pub const DEFAULT: Metrics = Metrics {
        control_height_sm: 24.0,
        control_height_md: 32.0,
        control_height_lg: 40.0,
        control_pad_x: 14.0,
        control_pad_y: 8.0,
        radius_xs: 2.0,
        radius_sm: 4.0,
        radius_md: 4.0,
        radius_lg: 6.0,
        radius_full: 9999.0,
        space_1: 4.0,
        space_2: 8.0,
        space_3: 12.0,
        space_4: 16.0,
        space_6: 24.0,
        space_8: 32.0,

        border_width: 1.0,
        focus_ring_width: 1.0,
        focus_ring_offset: 2.0,

        font_size_md: 13.0,
        font_size_sm: 12.0,
        font_size_xs: 11.0,

        control_min_width: 80.0,
        field_pad_x: 10.0,
        icon_size: 14.0,
        icon_stroke: 1.5,
        icon_gap: 8.0,

        checkbox_size: 16.0,
        radio_size: 16.0,
        radio_dot: 6.0,

        switch_width: 36.0,
        switch_height: 20.0,
        switch_thumb_inset: 3.0,

        slider_track: 4.0,
        slider_thumb: 14.0,
        progress_height: 4.0,

        badge_height: 20.0,
        badge_pad_x: 6.0,

        tab_height: 32.0,
        tab_underline: 2.0,

        accordion_row_height: 40.0,
        accordion_pad_x: 12.0,

        card_pad: 16.0,

        scrollbar_thumb: 6.0,
        scrollbar_inset: 3.0,

        textarea_min_height: 80.0,
        textarea_line_height: 20.0,

        select_option_height: 28.0,
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
        Self::CATPPUCCIN_MOCHA
    }
}

impl Theme {
    pub const NORD: Theme = Theme {
        name: "nord",
        is_dark: true,
        bg_canvas: Color::rgb(36, 41, 51),
        surface: Color::rgb(46, 52, 64),
        surface_subtle: Color::rgb(52, 59, 72),
        surface_elevated: Color::rgb(59, 66, 82),
        idle: Color::rgb(67, 76, 94),
        hovered: Color::rgb(76, 86, 106),
        pressed: Color::rgb(86, 97, 119),
        active: Color::rgb(136, 192, 208),
        active_hover: Color::rgb(150, 205, 219),
        active_pressed: Color::rgb(118, 168, 184),
        accent: Color::rgb(180, 142, 173),
        accent_hover: Color::rgb(193, 160, 186),
        accent_pressed: Color::rgb(160, 125, 153),
        disabled_bg: Color::rgb(52, 59, 72),
        disabled_border: Color::rgb(59, 66, 82),
        disabled_text: Color::rgb(98, 108, 128),
        border_faint: Color::rgb(52, 59, 72),
        border: Color::rgb(67, 76, 94),
        border_strong: Color::rgb(97, 110, 136),
        border_strong_hover: Color::rgb(129, 161, 193),
        focus_border: Color::rgb(236, 239, 244),
        text_primary: Color::rgb(236, 239, 244),
        text_secondary: Color::rgb(190, 198, 212),
        text_muted: Color::rgb(144, 155, 174),
        success: Color::rgb(163, 190, 140),
        warning: Color::rgb(235, 203, 139),
        error: Color::rgb(191, 97, 106),
        shadow_ambient: Color::rgba(0, 0, 0, 0.30),
        shadow_key: Color::rgba(0, 0, 0, 0.50),
        selection: Color::rgba(136, 192, 208, 0.28),
        metrics: Metrics::DEFAULT,
    };

    pub const DRACULA: Theme = Theme {
        name: "dracula",
        is_dark: true,
        bg_canvas: Color::rgb(30, 31, 41),
        surface: Color::rgb(40, 42, 54),
        surface_subtle: Color::rgb(46, 48, 63),
        surface_elevated: Color::rgb(52, 55, 70),
        idle: Color::rgb(58, 61, 80),
        hovered: Color::rgb(68, 71, 90),
        pressed: Color::rgb(80, 84, 106),
        active: Color::rgb(189, 147, 249),
        active_hover: Color::rgb(204, 170, 251),
        active_pressed: Color::rgb(160, 120, 225),
        accent: Color::rgb(255, 121, 198),
        accent_hover: Color::rgb(255, 146, 208),
        accent_pressed: Color::rgb(225, 100, 175),
        disabled_bg: Color::rgb(46, 48, 63),
        disabled_border: Color::rgb(52, 55, 70),
        disabled_text: Color::rgb(98, 102, 130),
        border_faint: Color::rgb(44, 46, 60),
        border: Color::rgb(62, 65, 85),
        border_strong: Color::rgb(98, 114, 164),
        border_strong_hover: Color::rgb(139, 153, 201),
        focus_border: Color::rgb(248, 248, 242),
        text_primary: Color::rgb(248, 248, 242),
        text_secondary: Color::rgb(190, 192, 210),
        text_muted: Color::rgb(130, 138, 175),
        success: Color::rgb(80, 250, 123),
        warning: Color::rgb(241, 250, 140),
        error: Color::rgb(255, 85, 85),
        shadow_ambient: Color::rgba(0, 0, 0, 0.35),
        shadow_key: Color::rgba(0, 0, 0, 0.55),
        selection: Color::rgba(189, 147, 249, 0.28),
        metrics: Metrics::DEFAULT,
    };

    pub const SOLARIZED_DARK: Theme = Theme {
        name: "solarized-dark",
        is_dark: true,
        bg_canvas: Color::rgb(0, 32, 41),
        surface: Color::rgb(0, 43, 54),
        surface_subtle: Color::rgb(7, 54, 66),
        surface_elevated: Color::rgb(12, 62, 76),
        idle: Color::rgb(16, 68, 82),
        hovered: Color::rgb(24, 80, 95),
        pressed: Color::rgb(34, 92, 108),
        active: Color::rgb(53, 152, 222),
        active_hover: Color::rgb(82, 174, 240),
        active_pressed: Color::rgb(38, 130, 198),
        accent: Color::rgb(230, 80, 155),
        accent_hover: Color::rgb(240, 105, 175),
        accent_pressed: Color::rgb(205, 60, 135),
        disabled_bg: Color::rgb(7, 54, 66),
        disabled_border: Color::rgb(12, 62, 76),
        disabled_text: Color::rgb(88, 110, 117),
        border_faint: Color::rgb(4, 48, 60),
        border: Color::rgb(16, 70, 84),
        border_strong: Color::rgb(88, 110, 117),
        border_strong_hover: Color::rgb(131, 148, 150),
        focus_border: Color::rgb(238, 232, 213),
        text_primary: Color::rgb(238, 232, 213),
        text_secondary: Color::rgb(147, 161, 161),
        text_muted: Color::rgb(112, 132, 138),
        success: Color::rgb(152, 173, 20),
        warning: Color::rgb(203, 160, 20),
        error: Color::rgb(235, 80, 75),
        shadow_ambient: Color::rgba(0, 0, 0, 0.35),
        shadow_key: Color::rgba(0, 0, 0, 0.55),
        selection: Color::rgba(53, 152, 222, 0.28),
        metrics: Metrics::DEFAULT,
    };

    pub const GRUVBOX_DARK: Theme = Theme {
        name: "gruvbox-dark",
        is_dark: true,
        bg_canvas: Color::rgb(29, 32, 33),
        surface: Color::rgb(40, 40, 40),
        surface_subtle: Color::rgb(50, 48, 47),
        surface_elevated: Color::rgb(60, 56, 54),
        idle: Color::rgb(66, 62, 59),
        hovered: Color::rgb(80, 73, 69),
        pressed: Color::rgb(102, 92, 84),
        active: Color::rgb(250, 189, 47),
        active_hover: Color::rgb(255, 205, 85),
        active_pressed: Color::rgb(215, 153, 33),
        accent: Color::rgb(211, 134, 155),
        accent_hover: Color::rgb(225, 155, 175),
        accent_pressed: Color::rgb(177, 98, 134),
        disabled_bg: Color::rgb(50, 48, 47),
        disabled_border: Color::rgb(60, 56, 54),
        disabled_text: Color::rgb(124, 111, 100),
        border_faint: Color::rgb(45, 43, 42),
        border: Color::rgb(62, 58, 56),
        border_strong: Color::rgb(124, 111, 100),
        border_strong_hover: Color::rgb(168, 153, 132),
        focus_border: Color::rgb(235, 219, 178),
        text_primary: Color::rgb(235, 219, 178),
        text_secondary: Color::rgb(189, 174, 147),
        text_muted: Color::rgb(146, 131, 116),
        success: Color::rgb(184, 187, 38),
        warning: Color::rgb(254, 128, 25),
        error: Color::rgb(251, 73, 52),
        shadow_ambient: Color::rgba(0, 0, 0, 0.35),
        shadow_key: Color::rgba(0, 0, 0, 0.55),
        selection: Color::rgba(250, 189, 47, 0.25),
        metrics: Metrics::DEFAULT,
    };

    pub const TOKYO_NIGHT: Theme = Theme {
        name: "tokyo-night",
        is_dark: true,
        bg_canvas: Color::rgb(22, 22, 30),
        surface: Color::rgb(26, 27, 38),
        surface_subtle: Color::rgb(31, 35, 53),
        surface_elevated: Color::rgb(36, 40, 59),
        idle: Color::rgb(41, 46, 66),
        hovered: Color::rgb(52, 59, 88),
        pressed: Color::rgb(65, 72, 104),
        active: Color::rgb(122, 162, 247),
        active_hover: Color::rgb(145, 180, 255),
        active_pressed: Color::rgb(98, 138, 222),
        accent: Color::rgb(187, 154, 247),
        accent_hover: Color::rgb(202, 174, 250),
        accent_pressed: Color::rgb(160, 125, 225),
        disabled_bg: Color::rgb(31, 35, 53),
        disabled_border: Color::rgb(41, 46, 66),
        disabled_text: Color::rgb(86, 95, 137),
        border_faint: Color::rgb(30, 32, 46),
        border: Color::rgb(41, 46, 66),
        border_strong: Color::rgb(86, 95, 137),
        border_strong_hover: Color::rgb(115, 125, 170),
        focus_border: Color::rgb(192, 202, 245),
        text_primary: Color::rgb(192, 202, 245),
        text_secondary: Color::rgb(154, 165, 206),
        text_muted: Color::rgb(110, 120, 165),
        success: Color::rgb(158, 206, 106),
        warning: Color::rgb(224, 175, 104),
        error: Color::rgb(247, 118, 142),
        shadow_ambient: Color::rgba(0, 0, 0, 0.35),
        shadow_key: Color::rgba(0, 0, 0, 0.55),
        selection: Color::rgba(122, 162, 247, 0.28),
        metrics: Metrics::DEFAULT,
    };

    pub const CATPPUCCIN_MOCHA: Theme = Theme {
        name: "catppuccin-mocha",
        is_dark: true,
        bg_canvas: Color::rgb(17, 17, 27),
        surface: Color::rgb(30, 30, 46),
        surface_subtle: Color::rgb(36, 36, 54),
        surface_elevated: Color::rgb(49, 50, 68),
        idle: Color::rgb(54, 56, 76),
        hovered: Color::rgb(69, 71, 90),
        pressed: Color::rgb(88, 91, 112),
        active: Color::rgb(137, 180, 250),
        active_hover: Color::rgb(160, 198, 255),
        active_pressed: Color::rgb(110, 155, 225),
        accent: Color::rgb(245, 194, 231),
        accent_hover: Color::rgb(248, 212, 238),
        accent_pressed: Color::rgb(215, 165, 205),
        disabled_bg: Color::rgb(36, 36, 54),
        disabled_border: Color::rgb(49, 50, 68),
        disabled_text: Color::rgb(108, 112, 134),
        border_faint: Color::rgb(36, 37, 52),
        border: Color::rgb(49, 50, 68),
        border_strong: Color::rgb(108, 112, 134),
        border_strong_hover: Color::rgb(147, 153, 178),
        focus_border: Color::rgb(205, 214, 244),
        text_primary: Color::rgb(205, 214, 244),
        text_secondary: Color::rgb(166, 173, 200),
        text_muted: Color::rgb(127, 132, 156),
        success: Color::rgb(166, 227, 161),
        warning: Color::rgb(249, 226, 175),
        error: Color::rgb(243, 139, 168),
        shadow_ambient: Color::rgba(0, 0, 0, 0.35),
        shadow_key: Color::rgba(0, 0, 0, 0.55),
        selection: Color::rgba(137, 180, 250, 0.28),
        metrics: Metrics::DEFAULT,
    };

    pub const ROSE_PINE: Theme = Theme {
        name: "rose-pine",
        is_dark: true,
        bg_canvas: Color::rgb(25, 23, 36),
        surface: Color::rgb(31, 29, 46),
        surface_subtle: Color::rgb(38, 35, 58),
        surface_elevated: Color::rgb(45, 42, 66),
        idle: Color::rgb(49, 46, 70),
        hovered: Color::rgb(64, 61, 82),
        pressed: Color::rgb(82, 79, 103),
        active: Color::rgb(196, 167, 231),
        active_hover: Color::rgb(210, 186, 240),
        active_pressed: Color::rgb(170, 140, 210),
        accent: Color::rgb(235, 111, 146),
        accent_hover: Color::rgb(240, 140, 168),
        accent_pressed: Color::rgb(205, 90, 125),
        disabled_bg: Color::rgb(38, 35, 58),
        disabled_border: Color::rgb(49, 46, 70),
        disabled_text: Color::rgb(110, 106, 134),
        border_faint: Color::rgb(35, 32, 52),
        border: Color::rgb(52, 49, 74),
        border_strong: Color::rgb(110, 106, 134),
        border_strong_hover: Color::rgb(144, 140, 170),
        focus_border: Color::rgb(224, 222, 244),
        text_primary: Color::rgb(224, 222, 244),
        text_secondary: Color::rgb(144, 140, 170),
        text_muted: Color::rgb(124, 120, 148),
        success: Color::rgb(156, 207, 216),
        warning: Color::rgb(246, 193, 119),
        error: Color::rgb(225, 90, 110),
        shadow_ambient: Color::rgba(0, 0, 0, 0.35),
        shadow_key: Color::rgba(0, 0, 0, 0.55),
        selection: Color::rgba(196, 167, 231, 0.28),
        metrics: Metrics::DEFAULT,
    };

    pub const MIDNIGHT: Theme = Theme {
        name: "midnight",
        is_dark: true,
        bg_canvas: Color::rgb(0, 0, 0),
        surface: Color::rgb(8, 8, 10),
        surface_subtle: Color::rgb(14, 14, 17),
        surface_elevated: Color::rgb(20, 20, 24),
        idle: Color::rgb(24, 24, 29),
        hovered: Color::rgb(33, 33, 40),
        pressed: Color::rgb(44, 44, 53),
        active: Color::rgb(64, 214, 176),
        active_hover: Color::rgb(100, 230, 195),
        active_pressed: Color::rgb(45, 180, 148),
        accent: Color::rgb(255, 140, 66),
        accent_hover: Color::rgb(255, 165, 105),
        accent_pressed: Color::rgb(225, 115, 45),
        disabled_bg: Color::rgb(14, 14, 17),
        disabled_border: Color::rgb(20, 20, 24),
        disabled_text: Color::rgb(70, 70, 80),
        border_faint: Color::rgb(12, 12, 15),
        border: Color::rgb(28, 28, 34),
        border_strong: Color::rgb(72, 72, 86),
        border_strong_hover: Color::rgb(110, 110, 130),
        focus_border: Color::rgb(240, 240, 245),
        text_primary: Color::rgb(240, 240, 245),
        text_secondary: Color::rgb(160, 160, 172),
        text_muted: Color::rgb(112, 112, 126),
        success: Color::rgb(80, 210, 120),
        warning: Color::rgb(240, 190, 70),
        error: Color::rgb(245, 95, 95),
        shadow_ambient: Color::rgba(0, 0, 0, 0.60),
        shadow_key: Color::rgba(0, 0, 0, 0.80),
        selection: Color::rgba(64, 214, 176, 0.25),
        metrics: Metrics::DEFAULT,
    };

    pub const SOLARIZED_LIGHT: Theme = Theme {
        name: "solarized-light",
        is_dark: false,
        bg_canvas: Color::rgb(253, 246, 227),
        surface: Color::rgb(255, 251, 238),
        surface_subtle: Color::rgb(238, 232, 213),
        surface_elevated: Color::rgb(255, 252, 242),
        idle: Color::rgb(234, 227, 205),
        hovered: Color::rgb(224, 217, 193),
        pressed: Color::rgb(212, 205, 180),
        active: Color::rgb(30, 115, 176),
        active_hover: Color::rgb(22, 98, 150),
        active_pressed: Color::rgb(15, 82, 128),
        accent: Color::rgb(211, 54, 130),
        accent_hover: Color::rgb(186, 40, 112),
        accent_pressed: Color::rgb(160, 30, 96),
        disabled_bg: Color::rgb(238, 232, 213),
        disabled_border: Color::rgb(225, 218, 196),
        disabled_text: Color::rgb(147, 161, 161),
        border_faint: Color::rgb(238, 232, 213),
        border: Color::rgb(221, 214, 190),
        border_strong: Color::rgb(147, 161, 161),
        border_strong_hover: Color::rgb(88, 110, 117),
        focus_border: Color::rgb(0, 43, 54),
        text_primary: Color::rgb(0, 43, 54),
        text_secondary: Color::rgb(88, 110, 117),
        text_muted: Color::rgb(101, 123, 131),
        success: Color::rgb(110, 130, 0),
        warning: Color::rgb(181, 137, 0),
        error: Color::rgb(220, 50, 47),
        shadow_ambient: Color::rgba(0, 43, 54, 0.05),
        shadow_key: Color::rgba(0, 43, 54, 0.10),
        selection: Color::rgba(30, 115, 176, 0.25),
        metrics: Metrics::DEFAULT,
    };

    pub const GRUVBOX_LIGHT: Theme = Theme {
        name: "gruvbox-light",
        is_dark: false,
        bg_canvas: Color::rgb(242, 229, 188),
        surface: Color::rgb(251, 241, 199),
        surface_subtle: Color::rgb(235, 219, 178),
        surface_elevated: Color::rgb(253, 246, 215),
        idle: Color::rgb(235, 219, 178),
        hovered: Color::rgb(213, 196, 161),
        pressed: Color::rgb(189, 174, 147),
        active: Color::rgb(7, 102, 120),
        active_hover: Color::rgb(5, 84, 100),
        active_pressed: Color::rgb(3, 70, 84),
        accent: Color::rgb(143, 63, 113),
        accent_hover: Color::rgb(120, 50, 95),
        accent_pressed: Color::rgb(100, 40, 80),
        disabled_bg: Color::rgb(235, 219, 178),
        disabled_border: Color::rgb(213, 196, 161),
        disabled_text: Color::rgb(168, 153, 132),
        border_faint: Color::rgb(235, 223, 185),
        border: Color::rgb(213, 196, 161),
        border_strong: Color::rgb(146, 131, 116),
        border_strong_hover: Color::rgb(102, 92, 84),
        focus_border: Color::rgb(60, 56, 54),
        text_primary: Color::rgb(60, 56, 54),
        text_secondary: Color::rgb(80, 73, 69),
        text_muted: Color::rgb(102, 92, 84),
        success: Color::rgb(121, 116, 14),
        warning: Color::rgb(181, 118, 20),
        error: Color::rgb(157, 0, 6),
        shadow_ambient: Color::rgba(60, 56, 54, 0.05),
        shadow_key: Color::rgba(60, 56, 54, 0.10),
        selection: Color::rgba(7, 102, 120, 0.25),
        metrics: Metrics::DEFAULT,
    };

    pub const CATPPUCCIN_LATTE: Theme = Theme {
        name: "catppuccin-latte",
        is_dark: false,
        bg_canvas: Color::rgb(239, 241, 245),
        surface: Color::rgb(252, 252, 254),
        surface_subtle: Color::rgb(230, 233, 239),
        surface_elevated: Color::rgb(252, 252, 254),
        idle: Color::rgb(220, 224, 232),
        hovered: Color::rgb(204, 208, 218),
        pressed: Color::rgb(188, 192, 204),
        active: Color::rgb(30, 102, 245),
        active_hover: Color::rgb(20, 84, 215),
        active_pressed: Color::rgb(14, 68, 180),
        accent: Color::rgb(203, 80, 172),
        accent_hover: Color::rgb(180, 62, 150),
        accent_pressed: Color::rgb(155, 48, 128),
        disabled_bg: Color::rgb(230, 233, 239),
        disabled_border: Color::rgb(204, 208, 218),
        disabled_text: Color::rgb(156, 160, 176),
        border_faint: Color::rgb(223, 227, 234),
        border: Color::rgb(204, 208, 218),
        border_strong: Color::rgb(140, 143, 161),
        border_strong_hover: Color::rgb(108, 111, 133),
        focus_border: Color::rgb(76, 79, 105),
        text_primary: Color::rgb(76, 79, 105),
        text_secondary: Color::rgb(92, 95, 119),
        text_muted: Color::rgb(108, 111, 133),
        success: Color::rgb(64, 160, 43),
        warning: Color::rgb(223, 142, 29),
        error: Color::rgb(210, 15, 57),
        shadow_ambient: Color::rgba(76, 79, 105, 0.05),
        shadow_key: Color::rgba(76, 79, 105, 0.10),
        selection: Color::rgba(30, 102, 245, 0.22),
        metrics: Metrics::DEFAULT,
    };

    pub fn all() -> &'static [Theme] {
        &[
            Self::NORD,
            Self::DRACULA,
            Self::SOLARIZED_DARK,
            Self::GRUVBOX_DARK,
            Self::TOKYO_NIGHT,
            Self::CATPPUCCIN_MOCHA,
            Self::ROSE_PINE,
            Self::MIDNIGHT,
            Self::SOLARIZED_LIGHT,
            Self::GRUVBOX_LIGHT,
            Self::CATPPUCCIN_LATTE,
        ]
    }

    pub fn from_name(name: &str) -> Option<Theme> {
        let slug = name.trim().to_lowercase().replace(' ', "-");
        match slug.as_str() {
            "nord" => Some(Self::DRACULA),
            "dracula" => Some(Self::DRACULA),
            "solarized_dark" => Some(Self::SOLARIZED_DARK),
            "gruvbox_dark" => Some(Self::GRUVBOX_DARK),
            "tokyo_night" => Some(Self::TOKYO_NIGHT),
            "catpuccin_mocha" => Some(Self::CATPPUCCIN_MOCHA),
            "rose_pine" => Some(Self::ROSE_PINE),
            "midnight" => Some(Self::MIDNIGHT),
            "solarized_light" => Some(Self::SOLARIZED_LIGHT),
            "gruvbox_light" => Some(Self::GRUVBOX_LIGHT),
            "catpuccin_latte" => Some(Self::CATPPUCCIN_LATTE),
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

    pub fn on_active(&self) -> Color {
        if self.is_dark {
            Color::rgb(5, 10, 10)
        } else {
            Color::rgb(245, 249, 250)
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
            _ => self.text_primary,
        }
    }
}
