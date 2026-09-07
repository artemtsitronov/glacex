use crate::color::Color;
use crate::painter::FontWeight;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, Widget, hash_id};
use accesskit::{NodeId, Role};

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

pub struct Label {
    id: String,
    text: String,
    variant: LabelVariant,
    font_size: f32,
    line_height: f32,
    weight: FontWeight,
    is_mono: bool,
    width: Option<f32>,
    height: Option<f32>,
}

impl Label {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Label {
            id: id.into(),
            text: text.into(),
            variant: LabelVariant::Primary,
            font_size: 14.0,
            line_height: 20.0,
            weight: FontWeight::Regular,
            is_mono: false,
            width: None,
            height: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = Some(size[0]);
        self.height = Some(size[1]);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.variant = LabelVariant::Custom(color);
        self
    }

    pub fn secondary(mut self) -> Self {
        self.variant = LabelVariant::Secondary;
        self
    }

    pub fn muted(mut self) -> Self {
        self.variant = LabelVariant::Muted;
        self
    }

    pub fn accent(mut self) -> Self {
        self.variant = LabelVariant::Accent;
        self
    }

    pub fn success(mut self) -> Self {
        self.variant = LabelVariant::Success;
        self
    }

    pub fn warning(mut self) -> Self {
        self.variant = LabelVariant::Warning;
        self
    }

    pub fn error(mut self) -> Self {
        self.variant = LabelVariant::Error;
        self
    }

    pub fn size_preset(mut self, size: f32) -> Self {
        self.font_size = size;
        self.line_height = (size * 1.35).round();
        self
    }

    pub fn size_with_line_height(mut self, size: f32, line_height: f32) -> Self {
        self.font_size = size;
        self.line_height = line_height;
        self
    }

    pub fn caption(mut self) -> Self {
        self.font_size = 12.0;
        self.line_height = 16.0;
        self.variant = LabelVariant::Muted;
        self
    }

    pub fn subheading(mut self) -> Self {
        self.font_size = 16.0;
        self.line_height = 22.0;
        self.weight = FontWeight::SemiBold;
        self
    }

    pub fn heading(mut self) -> Self {
        self.font_size = 18.0;
        self.line_height = 24.0;
        self.weight = FontWeight::SemiBold;
        self
    }

    pub fn title(mut self) -> Self {
        self.font_size = 22.0;
        self.line_height = 28.0;
        self.weight = FontWeight::Bold;
        self
    }

    pub fn metric(mut self) -> Self {
        self.font_size = 28.0;
        self.line_height = 34.0;
        self.weight = FontWeight::Bold;
        self
    }

    pub fn medium(mut self) -> Self {
        self.weight = FontWeight::Medium;
        self
    }

    pub fn semibold(mut self) -> Self {
        self.weight = FontWeight::SemiBold;
        self
    }

    pub fn bold(mut self) -> Self {
        self.weight = FontWeight::Bold;
        self
    }

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
        [
            self.width.unwrap_or(text_width),
            self.height.unwrap_or(self.line_height),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let clip_rect = [
            position[0],
            position[1],
            position[0] + size[0],
            position[1] + size[1],
        ];

        ui.register_accessible(self, clip_rect);

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

impl Accessible for Label {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::Label
    }
    fn accessibility_label(&self) -> Option<String> {
        Some(self.text.clone())
    }
}
