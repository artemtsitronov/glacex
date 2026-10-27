use crate::alignment::Alignment;
use crate::animation::Motion;
use crate::border::Border;
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::Path;
use crate::interaction::{Interaction, animate_hover, stored_hover};
use crate::stateful::{StateWeights, Stateful};
use crate::text::{FontFamily, FontWeight, TextStyle};
use crate::theme::Theme;
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

#[derive(Clone)]
pub struct LabelStyle {
    pub padding: Stateful<[f32; 2]>,
    pub align_x: Alignment,
    pub align_y: Alignment,
    pub fill: Stateful<Option<Fill>>,
    pub text_color: Stateful<Option<Color>>,
    pub border: Stateful<Border>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub hover_motion: f32,
}

impl LabelStyle {
    pub fn from_theme(_theme: &Theme) -> Self {
        LabelStyle {
            padding: Stateful::new([0.0, 0.0]),
            align_x: Alignment::Start,
            align_y: Alignment::Start,
            fill: Stateful::new(None),
            text_color: Stateful::new(None),
            border: Stateful::new(Border::NONE),
            sharp: false,
            path: Stateful::new(Path::rect([0.0; 4])),
            hover_motion: Motion::SNAPPY,
        }
    }

    pub fn highlighted(theme: &Theme) -> Self {
        LabelStyle {
            padding: Stateful::new([6.0, 2.0]),
            fill: Stateful::new(Some(Fill::Solid(theme.surface_subtle))),
            border: Stateful::new(Border::new(theme.metrics.border_width, theme.border_faint)),
            path: Stateful::new(Path::rect([theme.metrics.radius_sm; 4])),
            ..Self::from_theme(theme)
        }
    }
}

impl Default for LabelStyle {
    fn default() -> Self {
        LabelStyle::from_theme(&Theme::default())
    }
}

pub struct Label {
    id: String,
    text: String,
    variant: LabelVariant,
    text_style: TextStyle,
    style: Option<LabelStyle>,
    width: Option<f32>,
    height: Option<f32>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label {
            id: generate_uuid(),
            text: text.into(),
            variant: LabelVariant::Primary,
            text_style: TextStyle::new(),
            style: None,
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

    pub fn style(mut self, style: LabelStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<LabelStyle>) {
        self.style = style;
    }

    pub fn align(self, align_x: Alignment) -> Self {
        self.restyle(|style| style.align_x = align_x)
    }

    pub fn fill(self, fill: impl Into<Stateful<Option<Fill>>>) -> Self {
        let fill = fill.into();
        self.restyle(|style| style.fill = fill)
    }

    pub fn text_color(self, color: impl Into<Stateful<Option<Color>>>) -> Self {
        let color = color.into();
        self.restyle(|style| style.text_color = color)
    }

    pub fn border(self, border: impl Into<Stateful<Border>>) -> Self {
        let border = border.into();
        self.restyle(|style| style.border = border)
    }

    #[deprecated(since = "0.2.3", note = "use `.border(..)` instead")]
    pub fn border_color(self, color: impl Into<Stateful<Color>>) -> Self {
        let color = color.into();
        self.restyle(|style| style.border = style.border.with_color(&color))
    }

    #[deprecated(since = "0.2.3", note = "use `.border(..)` instead")]
    pub fn border_width(self, width: impl Into<Stateful<f32>>) -> Self {
        let width = width.into();
        self.restyle(|style| style.border = style.border.with_width(&width))
    }

    pub fn padding(self, padding: impl Into<Stateful<[f32; 2]>>) -> Self {
        let padding = padding.into();
        self.restyle(|style| style.padding = padding)
    }

    fn restyle(mut self, f: impl FnOnce(&mut LabelStyle)) -> Self {
        let mut style = self.style.take().unwrap_or_default();
        f(&mut style);
        self.style = Some(style);
        self
    }

    fn resolved_style(&self, theme: &Theme) -> LabelStyle {
        self.style
            .clone()
            .unwrap_or_else(|| LabelStyle::from_theme(theme))
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
        let style = self.resolved_style(ui.theme());
        let padding = style
            .padding
            .resolve(StateWeights::IDLE.hovered(stored_hover(ui, &self.id)));
        let text_width = ui.measure_text(&self.text, self.text_style);
        [
            self.width.unwrap_or(text_width + padding[0] * 2.0),
            self.height
                .unwrap_or(self.text_style.line_height + padding[1] * 2.0),
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
        let style = self.resolved_style(&theme);

        let previous = StateWeights::IDLE.hovered(stored_hover(ui, &self.id));
        let hovered =
            Interaction::update(&(style.path.resolve(previous))(position, size), ui).hovered;
        let weights =
            StateWeights::IDLE.hovered(animate_hover(ui, &self.id, hovered, style.hover_motion));

        if let Some(fill) = style.fill.resolve(weights) {
            ui.draw_shape(
                (style.path.resolve(weights))(position, size),
                fill,
                style.border.resolve(weights),
                0.0,
                style.sharp,
                0.0,
            );
        }

        let variant_color = match self.variant {
            LabelVariant::Primary => theme.text_primary,
            LabelVariant::Secondary => theme.text_secondary,
            LabelVariant::Muted => theme.text_muted,
            LabelVariant::Accent => theme.accent,
            LabelVariant::Success => theme.success,
            LabelVariant::Warning => theme.warning,
            LabelVariant::Error => theme.error,
            LabelVariant::Custom(c) => c,
        };
        let text_style = match style.text_color.resolve(weights) {
            Some(color) => self.text_style.color(color),
            None => self.text_style.or_color(variant_color),
        };

        let padding = style.padding.resolve(weights);
        let inner_size = [
            (size[0] - padding[0] * 2.0).max(0.0),
            (size[1] - padding[1] * 2.0).max(0.0),
        ];
        let text_width = ui.measure_text(&self.text, text_style);
        let free = [
            inner_size[0] - text_width,
            inner_size[1] - text_style.line_height,
        ];
        let offset = |align: Alignment, free: f32| match align {
            Alignment::Start | Alignment::Stretch => 0.0,
            Alignment::Center => free / 2.0,
            Alignment::End => free,
        };
        let text_position = [
            position[0] + padding[0] + offset(style.align_x, free[0]),
            position[1] + padding[1] + offset(style.align_y, free[1]),
        ];
        ui.draw_text(&self.text, text_style, text_position, clip_rect);
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
