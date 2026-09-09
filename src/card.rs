use crate::color::Color;
use crate::fill::Fill;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, AnyWidget, Measurable, Widget, hash_id};
use accesskit::{NodeId, Role};

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
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 16.0,
            padding: [22.0, 22.0],
            shadow: Some(ShadowStyle {
                color: Theme::SURFACE_SHADOW,
                blur_radius: 24.0,
                offset: [0.0, 6.0],
            }),
        }
    }
}

impl CardStyle {
    pub fn subtle() -> Self {
        CardStyle {
            fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 14.0,
            padding: [18.0, 18.0],
            shadow: None,
        }
    }

    pub fn elevated() -> Self {
        CardStyle {
            fill: Fill::Solid(Theme::SURFACE_ELEVATED),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: 20.0,
            padding: [24.0, 24.0],
            shadow: Some(ShadowStyle {
                color: Theme::SHADOW_KEY,
                blur_radius: 32.0,
                offset: [0.0, 10.0],
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
    id: Option<String>,
    child: Box<dyn AnyWidget + 'a>,
    variant: CardVariant,
    style: Option<CardStyle>,
    custom_padding: Option<[f32; 2]>,
    width: Option<f32>,
    height: Option<f32>,
}

impl<'a> Card<'a> {
    pub fn new(child: &'a mut impl Measurable) -> Self {
        Card {
            id: None,
            child: Box::new(child),
            variant: CardVariant::Default,
            style: None,
            custom_padding: None,
            width: None,
            height: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
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

    pub fn subtle(mut self) -> Self {
        self.variant = CardVariant::Subtle;
        self
    }

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
            self.width.unwrap_or(inner_size[0] + style.padding[0] * 2.0),
            self.height
                .unwrap_or(inner_size[1] + style.padding[1] * 2.0),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let style = self.resolved_style(ui.theme());

        if self.id.is_some() {
            ui.register_accessible(
                self,
                [
                    position[0],
                    position[1],
                    position[0] + size[0],
                    position[1] + size[1],
                ],
            );
        }

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

impl<'a> Accessible for Card<'a> {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(self.id.as_deref().unwrap_or("card")))
    }
    fn accessibility_role(&self) -> Role {
        Role::GenericContainer
    }
}
