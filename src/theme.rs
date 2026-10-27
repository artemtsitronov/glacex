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

    // secondary controls
    pub idle: Color,
    pub hovered: Color,
    pub pressed: Color,

    // primary controls
    pub active: Color,
    pub active_hover: Color,
    pub active_pressed: Color,

    // accent controls, used sparingly
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
    // the focus ring: 1px, offset 2px, in the text color
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
        Self::LIGHT
    }
}

impl Theme {
    pub const LIGHT: Theme = Theme {
        name: "light",
        is_dark: false,
        bg_canvas: Color::rgb(241, 246, 247),
        surface: Color::rgb(251, 254, 254),
        surface_subtle: Color::rgb(232, 240, 241),
        surface_elevated: Color::rgb(251, 254, 254),
        idle: Color::rgb(220, 232, 233),
        hovered: Color::rgb(207, 224, 224),
        pressed: Color::rgb(195, 215, 216),
        active: Color::rgb(0, 113, 119),
        active_hover: Color::rgb(3, 94, 99),
        active_pressed: Color::rgb(1, 79, 83),
        accent: Color::rgb(179, 2, 103),
        accent_hover: Color::rgb(151, 0, 86),
        accent_pressed: Color::rgb(128, 0, 72),
        disabled_bg: Color::rgb(226, 234, 235),
        disabled_border: Color::rgb(211, 223, 224),
        disabled_text: Color::rgb(138, 160, 162),
        border_faint: Color::rgb(219, 229, 230),
        border: Color::rgb(195, 209, 209),
        border_strong: Color::rgb(117, 143, 145),
        border_strong_hover: Color::rgb(52, 120, 125),
        focus_border: Color::rgb(10, 30, 32),
        text_primary: Color::rgb(10, 30, 32),
        text_secondary: Color::rgb(66, 90, 93),
        text_muted: Color::rgb(90, 114, 117),
        success: Color::rgb(40, 124, 66),
        warning: Color::rgb(173, 115, 0),
        error: Color::rgb(190, 34, 42),
        shadow_ambient: Color::rgba(10, 30, 32, 0.04),
        shadow_key: Color::rgba(10, 30, 32, 0.08),
        selection: Color::rgba(0, 113, 119, 0.25),
        metrics: Metrics::DEFAULT,
    };

    pub const DARK: Theme = Theme {
        name: "dark",
        is_dark: true,
        // the five Realtime Colors values, untouched:
        //   text #ddf1f3  background #050a0a  primary #6bc2c7
        //   secondary #446d6f  accent #ee77af
        // everything else is a mix of those five
        bg_canvas: Color::rgb(5, 10, 10),
        surface: Color::rgb(15, 20, 20),
        surface_subtle: Color::rgb(20, 26, 26),
        surface_elevated: Color::rgb(24, 31, 31),
        // text mixed into the background: 10% / 14% / 18%
        // (neutral on purpose, mixing in the teal secondary reads as green)
        idle: Color::rgb(27, 33, 33),
        hovered: Color::rgb(35, 42, 43),
        pressed: Color::rgb(44, 52, 52),
        active: Color::rgb(107, 194, 199),
        active_hover: Color::rgb(132, 204, 209),
        active_pressed: Color::rgb(91, 165, 169),
        accent: Color::rgb(238, 119, 175),
        accent_hover: Color::rgb(235, 143, 189),
        accent_pressed: Color::rgb(205, 104, 152),
        disabled_bg: Color::rgb(24, 31, 31),
        disabled_border: Color::rgb(20, 26, 26),
        disabled_text: Color::rgb(87, 98, 99),
        border_faint: Color::rgb(18, 24, 24),
        border: Color::rgb(31, 38, 38),
        border_strong: Color::rgb(68, 109, 111),
        // hover border is teal (secondary pulled toward primary);
        // caption text is neutral grey, so the two never read as one color
        border_strong_hover: Color::rgb(89, 156, 159),
        focus_border: Color::rgb(221, 241, 243),
        text_primary: Color::rgb(221, 241, 243),
        text_secondary: Color::rgb(161, 176, 178),
        text_muted: Color::rgb(122, 135, 136),
        success: Color::rgb(101, 198, 125),
        warning: Color::rgb(233, 180, 82),
        error: Color::rgb(246, 109, 103),
        shadow_ambient: Color::rgba(0, 0, 0, 0.35),
        shadow_key: Color::rgba(0, 0, 0, 0.55),
        selection: Color::rgba(107, 194, 199, 0.28),
        metrics: Metrics::DEFAULT,
    };

    pub fn light() -> Self {
        Self::LIGHT
    }

    pub fn dark() -> Self {
        Self::DARK
    }

    pub fn all() -> &'static [Theme] {
        &[Self::LIGHT, Self::DARK]
    }

    pub fn from_name(name: &str) -> Option<Theme> {
        let slug = name.trim().to_lowercase().replace(' ', "-");
        match slug.as_str() {
            "light" | "white" => Some(Self::LIGHT),
            "dark" | "linear" => Some(Self::DARK),
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

    // text on `active` and `accent` fills: near-white on the dark light-theme
    // teal and berry, near-black on the bright dark-theme ones
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
            // Token::Plain => self.text_primary,
            _ => self.text_primary,
        }
    }
}
