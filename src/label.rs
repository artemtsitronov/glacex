use crate::color::Color;
use crate::text::{FontFamily, FontWeight, TextStyle};
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
    text_style: TextStyle,
    width: Option<f32>,
    height: Option<f32>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label {
            id: uuid::Uuid::new_v4().to_string(),
            text: text.into(),
            variant: LabelVariant::Primary,
            text_style: TextStyle::new(),
            width: None,
            height: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
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
        self.text_style = self.text_style.size(size, (size * 1.35).round());
        self
    }

    pub fn size_with_line_height(mut self, size: f32, line_height: f32) -> Self {
        self.text_style = self.text_style.size(size, line_height);
        self
    }

    pub fn caption(mut self) -> Self {
        self.text_style = self.text_style.size(12.0, 16.0);
        self.variant = LabelVariant::Muted;
        self
    }

    pub fn subheading(self) -> Self {
        self.styled(16.0, 22.0, FontWeight::SemiBold)
    }

    pub fn heading(self) -> Self {
        self.styled(18.0, 24.0, FontWeight::SemiBold)
    }

    pub fn title(self) -> Self {
        self.styled(22.0, 28.0, FontWeight::Bold)
    }

    pub fn metric(self) -> Self {
        self.styled(28.0, 34.0, FontWeight::Bold)
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.text_style = self.text_style.weight(weight);
        self
    }

    pub fn medium(self) -> Self {
        self.weight(FontWeight::Medium)
    }

    pub fn semibold(self) -> Self {
        self.weight(FontWeight::SemiBold)
    }

    pub fn bold(self) -> Self {
        self.weight(FontWeight::Bold)
    }

    pub fn mono(mut self) -> Self {
        self.text_style = self.text_style.mono();
        self
    }

    pub fn family(mut self, family: FontFamily) -> Self {
        self.text_style = self.text_style.family(family);
        self
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = text_style;
        self
    }

    fn styled(mut self, font_size: f32, line_height: f32, weight: FontWeight) -> Self {
        self.text_style = self.text_style.size(font_size, line_height).weight(weight);
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
        let text_width = ui.measure_text(&self.text, self.text_style);
        [
            self.width.unwrap_or(text_width),
            self.height.unwrap_or(self.text_style.line_height),
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
        let text_style = self.text_style.or_color(color);
        ui.draw_text(&self.text, text_style, position, clip_rect);
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
