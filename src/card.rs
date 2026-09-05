use crate::color::Color;
use crate::fill::Fill;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{AnyWidget, Measurable, Widget};

#[derive(Debug, Clone)]
pub struct CardStyle {
    pub fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
    pub padding: [f32; 2],
    pub shadow: Option<ShadowStyle>,
}

impl Default for CardStyle {
    fn default() -> Self {
        CardStyle {
            fill: Fill::Solid(Theme::SURFACE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: Theme::RADIUS_LG,
            padding: [Theme::SPACE_4, Theme::SPACE_4],
            shadow: Some(ShadowStyle {
                color: Theme::SURFACE_SHADOW,
                blur_radius: 12.0,
                offset: [0.0, 3.0],
            }),
        }
    }
}

impl CardStyle {
    /// Inset subtle card surface (Linear sub-panel style).
    pub fn subtle() -> Self {
        CardStyle {
            fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            border_width: 1.0,
            border_color: Theme::BORDER_FAINT,
            corner_radius: Theme::RADIUS_MD,
            padding: [Theme::SPACE_3, Theme::SPACE_3],
            shadow: None,
        }
    }

    /// Elevated surface with prominent depth for floating cards/modals.
    pub fn elevated() -> Self {
        CardStyle {
            fill: Fill::Solid(Theme::SURFACE_ELEVATED),
            border_width: 1.0,
            border_color: Theme::BORDER_STRONG,
            corner_radius: Theme::RADIUS_LG,
            padding: [Theme::SPACE_4, Theme::SPACE_4],
            shadow: Some(ShadowStyle {
                color: Theme::SHADOW_KEY,
                blur_radius: 18.0,
                offset: [0.0, 6.0],
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Subtle,
    Elevated,
}

pub struct Card<'a> {
    child: Box<dyn AnyWidget + 'a>,
    variant: CardVariant,
    style: Option<CardStyle>,
    custom_padding: Option<[f32; 2]>,
}

impl<'a> Card<'a> {
    pub fn new(child: &'a mut impl Measurable) -> Self {
        Card {
            child: Box::new(child),
            variant: CardVariant::Default,
            style: None,
            custom_padding: None,
        }
    }

    pub fn style(mut self, style: CardStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<CardStyle>) {
        self.style = style;
    }

    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.custom_padding = Some(padding);
        self
    }

    /// Applies the subtle sub-panel style.
    pub fn subtle(mut self) -> Self {
        self.variant = CardVariant::Subtle;
        self
    }

    /// Applies the elevated floating card style.
    pub fn elevated(mut self) -> Self {
        self.variant = CardVariant::Elevated;
        self
    }

    fn resolved_style(&self, theme: &Theme) -> CardStyle {
        let mut base = if let Some(s) = &self.style {
            s.clone()
        } else {
            match self.variant {
                CardVariant::Default => theme.card_style(),
                CardVariant::Subtle => theme.card_subtle_style(),
                CardVariant::Elevated => theme.card_elevated_style(),
            }
        };
        if let Some(p) = self.custom_padding {
            base.padding = p;
        }
        base
    }
}

impl<'a> Widget for Card<'a> {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl<'a> Measurable for Card<'a> {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let style = self.resolved_style(ui.theme());
        let inner_size = self.child.measure(ui);
        [
            inner_size[0] + style.padding[0] * 2.0,
            inner_size[1] + style.padding[1] * 2.0,
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let style = self.resolved_style(ui.theme());
        if let Some(shadow) = &style.shadow {
            draw_shadow(shadow, position, size, style.corner_radius, ui);
        }

        ui.draw_rect(
            position,
            size,
            style.fill,
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            false,
            0.0,
        );

        let child_position = [
            position[0] + style.padding[0],
            position[1] + style.padding[1],
        ];
        let child_size = [
            (size[0] - style.padding[0] * 2.0).max(0.0),
            (size[1] - style.padding[1] * 2.0).max(0.0),
        ];

        self.child.arrange(child_position, child_size, ui);
    }
}
