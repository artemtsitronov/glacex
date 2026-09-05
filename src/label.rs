use crate::color::Color;
use crate::painter::FontWeight;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LabelVariant {
    #[default]
    Primary,
    Secondary,
    Muted,
    Accent,
    Success,
    Warning,
    Error,
    Custom(Color),
}

/// Typography label with semantic hierarchy, weights, and Geist font rendering.
pub struct Label {
    text: String,
    variant: LabelVariant,
    font_size: f32,
    line_height: f32,
    weight: FontWeight,
    is_mono: bool,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label {
            text: text.into(),
            variant: LabelVariant::Primary,
            font_size: 14.0,
            line_height: 20.0,
            weight: FontWeight::Regular,
            is_mono: false,
        }
    }

    /// Sets an explicit text color.
    pub fn color(mut self, color: Color) -> Self {
        self.variant = LabelVariant::Custom(color);
        self
    }

    /// Convenience for secondary/supporting text hierarchy (`ui.theme().text_secondary`).
    pub fn secondary(mut self) -> Self {
        self.variant = LabelVariant::Secondary;
        self
    }

    /// Convenience for muted/subdued text hierarchy (`ui.theme().text_muted`).
    pub fn muted(mut self) -> Self {
        self.variant = LabelVariant::Muted;
        self
    }

    /// Convenience for primary accent-colored text (`ui.theme().active`).
    pub fn accent(mut self) -> Self {
        self.variant = LabelVariant::Accent;
        self
    }

    /// Semantic success color (`ui.theme().success`).
    pub fn success(mut self) -> Self {
        self.variant = LabelVariant::Success;
        self
    }

    /// Semantic warning color (`ui.theme().warning`).
    pub fn warning(mut self) -> Self {
        self.variant = LabelVariant::Warning;
        self
    }

    /// Semantic error color (`ui.theme().error`).
    pub fn error(mut self) -> Self {
        self.variant = LabelVariant::Error;
        self
    }

    /// Sets custom font size and proportional line height.
    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self.line_height = (size * 1.35).round();
        self
    }

    /// Sets custom font size and explicit line height.
    pub fn size_with_line_height(mut self, size: f32, line_height: f32) -> Self {
        self.font_size = size;
        self.line_height = line_height;
        self
    }

    /// Small muted caption (12px / 16px line height).
    pub fn caption(mut self) -> Self {
        self.font_size = 12.0;
        self.line_height = 16.0;
        self.variant = LabelVariant::Muted;
        self
    }

    /// Subheading style (16px / 22px line height, semibold).
    pub fn subheading(mut self) -> Self {
        self.font_size = 16.0;
        self.line_height = 22.0;
        self.weight = FontWeight::SemiBold;
        self
    }

    /// Section heading style (18px / 24px line height, semibold).
    pub fn heading(mut self) -> Self {
        self.font_size = 18.0;
        self.line_height = 24.0;
        self.weight = FontWeight::SemiBold;
        self
    }

    /// Prominent card or page title (22px / 28px line height, bold).
    pub fn title(mut self) -> Self {
        self.font_size = 22.0;
        self.line_height = 28.0;
        self.weight = FontWeight::Bold;
        self
    }

    /// Large hero metric (28px / 34px line height, bold) - e.g. "$1,250.00" in shadcn dashboards.
    pub fn metric(mut self) -> Self {
        self.font_size = 28.0;
        self.line_height = 34.0;
        self.weight = FontWeight::Bold;
        self
    }

    /// Sets Medium font weight (500).
    pub fn medium(mut self) -> Self {
        self.weight = FontWeight::Medium;
        self
    }

    /// Sets SemiBold font weight (600).
    pub fn semibold(mut self) -> Self {
        self.weight = FontWeight::SemiBold;
        self
    }

    /// Sets Bold font weight (700).
    pub fn bold(mut self) -> Self {
        self.weight = FontWeight::Bold;
        self
    }

    /// Renders using bundled Geist Mono.
    pub fn mono(mut self) -> Self {
        self.is_mono = true;
        self
    }
}

impl Widget for Label {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for Label {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let text_width = ui.measure_text_styled(
            &self.text,
            self.font_size,
            self.line_height,
            self.weight,
            self.is_mono,
        );
        [text_width, self.line_height]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let clip_rect = [
            position[0],
            position[1],
            position[0] + size[0],
            position[1] + size[1],
        ];
        let theme = *ui.theme();
        let color = match self.variant {
            LabelVariant::Primary => theme.text_primary,
            LabelVariant::Secondary => theme.text_secondary,
            LabelVariant::Muted => theme.text_muted,
            LabelVariant::Accent => theme.active,
            LabelVariant::Success => theme.success,
            LabelVariant::Warning => theme.warning,
            LabelVariant::Error => theme.error,
            LabelVariant::Custom(c) => c,
        };
        ui.draw_text_styled(
            &self.text,
            position,
            clip_rect,
            color,
            self.font_size,
            self.line_height,
            self.weight,
            self.is_mono,
        );
    }
}
