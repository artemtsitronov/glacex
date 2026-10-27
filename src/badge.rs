use crate::alignment::Alignment;
use crate::animation::Motion;
use crate::border::Border;
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::Path;
use crate::interaction::{Interaction, animate_hover, stored_hover};
use crate::misc::align_text_in;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
use crate::text::{FontWeight, TextStyle};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, Widget, hash_id};
use accesskit::{NodeId, Role};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Success,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct BadgeStyle {
    pub fill: Stateful<Fill>,
    pub border: Stateful<Border>,
    pub padding: Stateful<[f32; 2]>,
    pub height: Stateful<f32>,
    pub min_width: Stateful<f32>,
    pub text_style: Stateful<TextStyle>,
    pub text_align: Alignment,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub hover_motion: f32,
}

impl BadgeStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self::build(
            theme,
            Fill::Solid(theme.accent),
            Color::TRANSPARENT,
            theme.on_active(),
        )
    }

    pub fn secondary(theme: &Theme) -> Self {
        Self::build(
            theme,
            Fill::Solid(theme.surface_subtle),
            theme.border_faint,
            theme.text_secondary,
        )
    }

    pub fn outline(theme: &Theme) -> Self {
        Self::build(
            theme,
            Fill::Solid(Color::TRANSPARENT),
            theme.border,
            theme.text_primary,
        )
    }

    pub fn success(theme: &Theme) -> Self {
        Self::tinted(theme, theme.success)
    }

    pub fn warning(theme: &Theme) -> Self {
        Self::tinted(theme, theme.warning)
    }

    pub fn error(theme: &Theme) -> Self {
        Self::tinted(theme, theme.error)
    }

    pub fn tinted(theme: &Theme, color: Color) -> Self {
        Self::build(
            theme,
            Fill::Solid(color.with_alpha(0.12)),
            color.with_alpha(0.28),
            color,
        )
    }

    fn build(theme: &Theme, fill: Fill, border_color: Color, text_color: Color) -> Self {
        BadgeStyle {
            fill: Stateful::new(fill),
            border: Stateful::new(Border::new(theme.metrics.border_width, border_color)),
            padding: Stateful::new([theme.metrics.badge_pad_x, 2.0]),
            height: Stateful::new(theme.metrics.badge_height),
            min_width: Stateful::new(0.0),
            text_style: Stateful::new(
                TextStyle::new()
                    .size(theme.metrics.font_size_sm, 16.0)
                    .weight(FontWeight::Medium)
                    .color(text_color),
            ),
            text_align: Alignment::Center,
            shadow: Stateful::new(None),
            sharp: false,
            path: Stateful::new(Path::rect([theme.metrics.radius_full; 4])),
            hover_motion: Motion::SNAPPY,
        }
    }
}

impl Default for BadgeStyle {
    fn default() -> Self {
        BadgeStyle::from_theme(&Theme::default())
    }
}

pub struct Badge {
    id: String,
    text: String,
    variant: BadgeVariant,
    style: Option<BadgeStyle>,
    text_style: Option<TextStyle>,
    width: Option<f32>,
    height: Option<f32>,
    custom_padding: Option<Stateful<[f32; 2]>>,
    overrides: StyleOverrides,
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Badge {
            id: generate_uuid(),
            text: text.into(),
            variant: BadgeVariant::Default,
            style: None,
            text_style: None,
            width: None,
            height: None,
            custom_padding: None,
            overrides: StyleOverrides::default(),
        }
    }

    pub fn padding(mut self, padding: impl Into<Stateful<[f32; 2]>>) -> Self {
        self.custom_padding = Some(padding.into());
        self
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

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.variant = BadgeVariant::Secondary;
        self
    }

    pub fn outline(mut self) -> Self {
        self.variant = BadgeVariant::Outline;
        self
    }

    pub fn success(mut self) -> Self {
        self.variant = BadgeVariant::Success;
        self
    }

    pub fn warning(mut self) -> Self {
        self.variant = BadgeVariant::Warning;
        self
    }

    pub fn error(mut self) -> Self {
        self.variant = BadgeVariant::Error;
        self
    }

    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<BadgeStyle>) {
        self.style = style;
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = Some(text_style);
        self
    }

    pub fn set_text_style(&mut self, text_style: Option<TextStyle>) {
        self.text_style = text_style;
    }

    fn resolved_style(&self, theme: &Theme) -> BadgeStyle {
        let mut style = self.style.clone().unwrap_or_else(|| match self.variant {
            BadgeVariant::Default => BadgeStyle::from_theme(theme),
            BadgeVariant::Secondary => BadgeStyle::secondary(theme),
            BadgeVariant::Outline => BadgeStyle::outline(theme),
            BadgeVariant::Success => BadgeStyle::success(theme),
            BadgeVariant::Warning => BadgeStyle::warning(theme),
            BadgeVariant::Error => BadgeStyle::error(theme),
        });
        if let Some(p) = &self.custom_padding {
            style.padding = p.clone();
        }
        if let Some(custom) = self.text_style {
            style.text_style = style.text_style.map(|t| custom.inherit(*t));
        }
        self.overrides
            .apply(&mut style.fill, &mut style.border, &mut style.shadow);
        style
    }
}

style_overrides!(Badge);

impl Widget for Badge {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for Badge {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let weights = StateWeights::IDLE.hovered(stored_hover(ui, &self.id));
        let text_style = style.text_style.resolve(weights);
        let padding = style.padding.resolve(weights);
        let text_width = ui.measure_text(&self.text, text_style);
        [
            self.width
                .unwrap_or((text_width + padding[0] * 2.0).max(style.min_width.resolve(weights))),
            self.height.unwrap_or(
                style
                    .height
                    .resolve(weights)
                    .max(text_style.line_height + padding[1] * 2.0),
            ),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);

        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        let previous = StateWeights::IDLE.hovered(stored_hover(ui, &self.id));
        let hovered =
            Interaction::update(&(style.path.resolve(previous))(position, size), ui).hovered;
        let weights =
            StateWeights::IDLE.hovered(animate_hover(ui, &self.id, hovered, style.hover_motion));
        let path = style.path.resolve(weights);

        if let Some(shadow) = &style.shadow.resolve(weights) {
            draw_shadow(shadow, position, size, &path, ui);
        }

        ui.draw_shape(
            path(position, size),
            style.fill.resolve(weights),
            style.border.resolve(weights),
            0.0,
            style.sharp,
            0.0,
        );

        let text_style = style.text_style.resolve(weights);
        let text_width = ui.measure_text(&self.text, text_style);
        let text_pos = align_text_in(
            position,
            size,
            text_width,
            text_style.line_height,
            style.padding.resolve(weights)[0],
            style.text_align,
        );
        let clip_rect = [
            position[0],
            position[1],
            position[0] + size[0],
            position[1] + size[1],
        ];
        ui.draw_text(&self.text, text_style, text_pos, clip_rect);
    }
}

impl Accessible for Badge {
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
