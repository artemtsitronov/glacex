use crate::animation::Motion;
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::Path;
use crate::interaction::{animate_hover, stored_hover};
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, AnyWidget, Measurable, Widget, hash_id, is_anonymous};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Clone)]
pub struct CardStyle {
    pub fill: Stateful<Fill>,
    pub border_width: Stateful<f32>,
    pub border_color: Stateful<Color>,
    pub padding: Stateful<[f32; 2]>,
    pub min_size: Stateful<[f32; 2]>,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub sharp: bool,
    pub clip_content: bool,
    pub path: Stateful<Path>,
    pub hover_motion: f32,
    pub cursor: Stateful<Option<CursorIcon>>,
}

impl CardStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self::build(
            Fill::Solid(theme.surface),
            theme.border,
            [20.0, 20.0],
            Some(theme.shadow_sm()[0]),
            10.0,
        )
    }

    pub fn subtle(theme: &Theme) -> Self {
        Self::build(
            Fill::Solid(theme.surface_subtle),
            theme.border_faint,
            [16.0, 16.0],
            None,
            8.0,
        )
    }

    pub fn elevated(theme: &Theme) -> Self {
        Self::build(
            Fill::Solid(theme.surface_elevated),
            theme.border_strong,
            [20.0, 20.0],
            Some(theme.shadow_md()[0]),
            12.0,
        )
    }

    pub fn interactive(theme: &Theme) -> Self {
        let base = Self::from_theme(theme);
        CardStyle {
            fill: base
                .fill
                .clone()
                .hovered(Fill::Solid(theme.surface_elevated)),
            border_color: base.border_color.clone().hovered(theme.border_strong),
            shadow: base.shadow.clone().hovered(Some(theme.shadow_md()[0])),
            cursor: CursorIcon::Pointer.into(),
            ..base
        }
    }

    fn build(
        fill: Fill,
        border_color: Color,
        padding: [f32; 2],
        shadow: Option<ShadowStyle>,
        radius: f32,
    ) -> Self {
        CardStyle {
            fill: Stateful::new(fill),
            border_width: Stateful::new(1.0),
            border_color: Stateful::new(border_color),
            padding: Stateful::new(padding),
            min_size: Stateful::new([0.0, 0.0]),
            shadow: Stateful::new(shadow),
            sharp: false,
            clip_content: false,
            path: Stateful::new(Path::rect([radius; 4])),
            hover_motion: Motion::SNAPPY,
            cursor: Stateful::new(None),
        }
    }
}

impl Default for CardStyle {
    fn default() -> Self {
        CardStyle::from_theme(&Theme::default())
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
    custom_padding: Option<Stateful<[f32; 2]>>,
    overrides: StyleOverrides,
    width: Option<f32>,
    height: Option<f32>,
}

impl<'a> Card<'a> {
    pub fn new(child: &'a mut impl Measurable) -> Self {
        Card {
            id: generate_uuid(),
            child: Box::new(child),
            variant: CardVariant::Default,
            style: None,
            custom_padding: None,
            overrides: StyleOverrides::default(),
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

    pub fn padding(mut self, padding: impl Into<Stateful<[f32; 2]>>) -> Self {
        self.custom_padding = Some(padding.into());
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
                CardVariant::Default => CardStyle::from_theme(theme),
                CardVariant::Subtle => CardStyle::subtle(theme),
                CardVariant::Elevated => CardStyle::elevated(theme),
            }
        };
        if let Some(p) = &self.custom_padding {
            base.padding = p.clone();
        }
        self.overrides.apply(
            &mut base.fill,
            &mut base.border_color,
            &mut base.border_width,
            &mut base.shadow,
        );
        base
    }
}

style_overrides!(Card<'_>);

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
        let weights = StateWeights::IDLE.hovered(stored_hover(ui, &self.id));
        let padding = style.padding.resolve(weights);
        let min_size = style.min_size.resolve(weights);
        let inner_size = self.child.measure(ui);
        [
            self.width
                .unwrap_or((inner_size[0] + padding[0] * 2.0).max(min_size[0])),
            self.height
                .unwrap_or((inner_size[1] + padding[1] * 2.0).max(min_size[1])),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let style = self.resolved_style(ui.theme());

        // anonymous cards share a fallback id, only register named ones
        if !is_anonymous(&self.id) {
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

        let previous = StateWeights::IDLE.hovered(stored_hover(ui, &self.id));
        let hit_path = style.path.resolve(previous);
        let mouse_pos = ui.mouse_position();
        let hovered = !ui.is_input_blocked(mouse_pos)
            && ui.point_in_current_clip(mouse_pos)
            && hit_path(position, size).contains(mouse_pos);
        let weights =
            StateWeights::IDLE.hovered(animate_hover(ui, &self.id, hovered, style.hover_motion));

        if hovered && let Some(cursor) = style.cursor.resolve(StateWeights::IDLE.hovered(1.0)) {
            ui.set_cursor_icon(cursor);
        }

        let path = style.path.resolve(weights);
        if let Some(shadow) = &style.shadow.resolve(weights) {
            draw_shadow(shadow, position, size, &path, ui);
        }

        ui.draw_shape(
            path(position, size),
            style.fill.resolve(weights),
            style.border_width.resolve(weights),
            style.border_color.resolve(weights),
            0.0,
            style.sharp,
            0.0,
        );

        let padding = style.padding.resolve(weights);
        let child_position = [position[0] + padding[0], position[1] + padding[1]];
        let child_size = [
            (size[0] - padding[0] * 2.0).max(0.0),
            (size[1] - padding[1] * 2.0).max(0.0),
        ];

        if style.clip_content {
            ui.push_clip([
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ]);
        }
        self.child.arrange(child_position, child_size, ui);
        if style.clip_content {
            ui.pop_clip();
        }
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
