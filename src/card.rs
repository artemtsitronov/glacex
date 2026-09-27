use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::Path;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, AnyWidget, IntoId, Measurable, Widget, hash_id};
use accesskit::{NodeId, Role};

#[derive(Clone)]
pub struct CardStyle {
    pub fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub padding: [f32; 2],
    pub shadow: Option<ShadowStyle>,
    pub path: Path,
}

impl Default for CardStyle {
    fn default() -> Self {
        CardStyle {
            fill: Fill::Solid(Theme::SURFACE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            padding: [Theme::SPACE_4, Theme::SPACE_4],
            shadow: Some(ShadowStyle {
                color: Theme::SURFACE_SHADOW,
                blur_radius: 12.0,
                offset: [0.0, 3.0],
            }),
            path: Path::rect([Theme::RADIUS_LG; 4]),
        }
    }
}

impl CardStyle {
    pub fn subtle() -> Self {
        CardStyle {
            fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            border_width: 1.0,
            border_color: Theme::BORDER_FAINT,
            padding: [Theme::SPACE_3, Theme::SPACE_3],
            shadow: None,
            path: Path::rect([Theme::RADIUS_MD; 4]),
        }
    }

    pub fn elevated() -> Self {
        CardStyle {
            fill: Fill::Solid(Theme::SURFACE_ELEVATED),
            border_width: 1.0,
            border_color: Theme::BORDER_STRONG,
            padding: [Theme::SPACE_4, Theme::SPACE_4],
            shadow: Some(ShadowStyle {
                color: Theme::SHADOW_KEY,
                blur_radius: 18.0,
                offset: [0.0, 6.0],
            }),
            path: Path::rect([Theme::RADIUS_LG; 4]),
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
    id: String,
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
            id: uuid::Uuid::new_v4().to_string(),
            child: Box::new(child),
            variant: CardVariant::Default,
            style: None,
            custom_padding: None,
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

        // anonymous cards share a fallback id, only register named ones
        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        if let Some(shadow) = &style.shadow {
            draw_shadow(shadow, position, size, &style.path, ui);
        }

        ui.draw_shape(
            (style.path)(position, size),
            style.fill,
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
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::GenericContainer
    }
}
