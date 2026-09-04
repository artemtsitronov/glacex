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
            corner_radius: 12.0,
            padding: [16.0, 16.0],
            shadow: Some(ShadowStyle {
                color: Theme::SURFACE_SHADOW,
                blur_radius: 14.0,
                offset: [0.0, 4.0],
            }),
        }
    }
}

pub struct Card<'a> {
    child: Box<dyn AnyWidget + 'a>,
    style: CardStyle,
}

impl<'a> Card<'a> {
    pub fn new(child: &'a mut impl Measurable) -> Self {
        Card {
            child: Box::new(child),
            style: CardStyle::default(),
        }
    }

    pub fn style(mut self, style: CardStyle) -> Self {
        self.style = style;
        self
    }

    pub fn padding(mut self, x: f32, y: f32) -> Self {
        self.style.padding = [x, y];
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.style.corner_radius = radius;
        self
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
        let inner_size = self.child.measure(ui);
        [
            inner_size[0] + self.style.padding[0] * 2.0,
            inner_size[1] + self.style.padding[1] * 2.0,
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        if let Some(shadow) = &self.style.shadow {
            draw_shadow(shadow, position, size, self.style.corner_radius, ui);
        }

        ui.draw_rect(
            position,
            size,
            self.style.fill.clone(),
            self.style.corner_radius,
            self.style.border_width,
            self.style.border_color,
            0.0,
            false,
        );

        let child_position = [
            position[0] + self.style.padding[0],
            position[1] + self.style.padding[1],
        ];
        let child_size = [
            (size[0] - self.style.padding[0] * 2.0).max(0.0),
            (size[1] - self.style.padding[1] * 2.0).max(0.0),
        ];

        self.child.arrange(child_position, child_size, ui);
    }
}
