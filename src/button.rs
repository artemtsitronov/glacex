use crate::alignment::Alignment;
use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::geometry::Path;
use crate::interaction::Interaction;
use crate::misc::{align_text_in, scale_rect};
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
use crate::text::{FontWeight, TextStyle};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, StatefulWidget, Widget, hash_id, is_anonymous};
use crate::{Fill, generate_uuid};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

pub type ButtonResponse = Interaction;

#[derive(Clone)]
pub struct ButtonStyle {
    pub fill: Stateful<Fill>,
    pub border_width: Stateful<f32>,
    pub border_color: Stateful<Color>,
    pub padding: Stateful<[f32; 2]>,
    pub min_size: Stateful<[f32; 2]>,
    pub text_style: Stateful<TextStyle>,
    pub text_align: Alignment,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub scale: Stateful<f32>,
    pub offset: Stateful<[f32; 2]>,
    pub hover_motion: f32,
    pub press_motion: f32,
    pub cursor: Stateful<CursorIcon>,
}

fn pressed_variant(shadow: Option<ShadowStyle>, press_offset: [f32; 2]) -> Option<ShadowStyle> {
    shadow.map(|s| {
        s.offset([s.offset[0] - press_offset[0], s.offset[1] - press_offset[1]])
            .blur((s.blur_radius - 2.0).max(1.0))
    })
}

impl ButtonStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self::build(
            theme,
            Stateful::new(Fill::Solid(theme.surface_subtle))
                .hovered(Fill::Solid(theme.hovered))
                .pressed(Fill::Solid(theme.pressed)),
            1.0,
            theme.border,
            theme.text_primary,
            Some(theme.shadow_sm()[0]),
        )
    }

    pub fn primary(theme: &Theme) -> Self {
        let text_color = if theme.is_dark && theme.active.r > 0.7 && theme.active.g > 0.7 {
            Color::BLACK
        } else {
            Color::WHITE
        };
        Self::build(
            theme,
            Stateful::new(Fill::Solid(theme.active))
                .hovered(Fill::Solid(theme.active_hover))
                .pressed(Fill::Solid(theme.active.darken(0.12))),
            1.0,
            Color::WHITE.with_alpha(0.15),
            text_color,
            Some(ShadowStyle::new(
                theme.active.with_alpha(0.20),
                4.0,
                [0.0, 1.5],
            )),
        )
    }

    pub fn outline(theme: &Theme) -> Self {
        Self::build(
            theme,
            Stateful::new(Fill::Solid(Color::TRANSPARENT))
                .hovered(Fill::Solid(theme.surface_subtle))
                .pressed(Fill::Solid(theme.hovered)),
            1.0,
            theme.border_strong,
            theme.text_primary,
            Some(theme.shadow_sm()[0]),
        )
    }

    pub fn ghost(theme: &Theme) -> Self {
        Self::build(
            theme,
            Stateful::new(Fill::Solid(Color::TRANSPARENT))
                .hovered(Fill::Solid(theme.surface_subtle))
                .pressed(Fill::Solid(theme.hovered)),
            0.0,
            Color::TRANSPARENT,
            theme.text_secondary,
            None,
        )
    }

    pub fn danger(theme: &Theme) -> Self {
        Self::build(
            theme,
            Stateful::new(Fill::Solid(theme.error))
                .hovered(Fill::Solid(theme.error.lighten(0.08)))
                .pressed(Fill::Solid(theme.error.darken(0.12))),
            1.0,
            theme.error.darken(0.15),
            Color::WHITE,
            Some(ShadowStyle::new(
                theme.error.with_alpha(0.25),
                4.0,
                [0.0, 1.5],
            )),
        )
    }

    fn build(
        theme: &Theme,
        fill: Stateful<Fill>,
        border_width: f32,
        border_color: Color,
        text_color: Color,
        shadow: Option<ShadowStyle>,
    ) -> Self {
        let press_offset = [0.0, 2.0];
        ButtonStyle {
            fill,
            border_width: Stateful::new(border_width),
            border_color: Stateful::new(border_color)
                .hovered(theme.border_strong)
                .pressed(theme.border_strong),
            padding: Stateful::new([14.0, 8.0]),
            min_size: Stateful::new([0.0, 0.0]),
            text_style: Stateful::new(
                TextStyle::new()
                    .size(14.0, 20.0)
                    .weight(FontWeight::Medium)
                    .color(text_color),
            ),
            text_align: Alignment::Center,
            shadow: Stateful::new(shadow).pressed(pressed_variant(shadow, press_offset)),
            sharp: false,
            path: Stateful::new(Path::rect([6.0; 4])),
            scale: Stateful::new(1.0),
            offset: Stateful::new([0.0, 0.0]).pressed(press_offset),
            hover_motion: Motion::SNAPPY,
            press_motion: Motion::INSTANT,
            cursor: Stateful::new(CursorIcon::Pointer),
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle::from_theme(&Theme::default())
    }
}

pub struct ButtonState {
    pub hover_t: f32,
    pub press_t: f32,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState {
            hover_t: 0.0,
            press_t: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Outline,
    Ghost,
    Danger,
}

pub struct Button {
    id: String,
    label: String,
    interaction: Interaction,
    variant: ButtonVariant,
    style: Option<ButtonStyle>,
    text_style: Option<TextStyle>,
    tooltip: Option<String>,
    width: Option<f32>,
    height: Option<f32>,
    custom_padding: Option<Stateful<[f32; 2]>>,
    overrides: StyleOverrides,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Button {
            id: generate_uuid(),
            label: label.into(),
            interaction: Interaction::default(),
            variant: ButtonVariant::Default,
            style: None,
            text_style: None,
            tooltip: None,
            width: None,
            height: None,
            custom_padding: None,
            overrides: StyleOverrides::default(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Stateful<[f32; 2]>>) -> Self {
        self.custom_padding = Some(padding.into());
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

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<ButtonStyle>) {
        self.style = style;
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = Some(text_style);
        self
    }

    pub fn set_text_style(&mut self, text_style: Option<TextStyle>) {
        self.text_style = text_style;
    }

    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    pub fn outline(mut self) -> Self {
        self.variant = ButtonVariant::Outline;
        self
    }

    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        self
    }

    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn hovered(&self) -> bool {
        self.interaction.hovered
    }

    pub fn pressed(&self) -> bool {
        self.interaction.pressed
    }

    pub fn clicked(&self) -> bool {
        self.interaction.clicked
    }

    fn resolved_style(&self, theme: &Theme) -> ButtonStyle {
        let mut style = self.style.clone().unwrap_or_else(|| match self.variant {
            ButtonVariant::Default => ButtonStyle::from_theme(theme),
            ButtonVariant::Primary => ButtonStyle::primary(theme),
            ButtonVariant::Outline => ButtonStyle::outline(theme),
            ButtonVariant::Ghost => ButtonStyle::ghost(theme),
            ButtonVariant::Danger => ButtonStyle::danger(theme),
        });
        if let Some(p) = &self.custom_padding {
            style.padding = p.clone();
        }
        if let Some(custom) = self.text_style {
            style.text_style = style.text_style.map(|t| custom.inherit(*t));
        }
        self.overrides.apply(
            &mut style.fill,
            &mut style.border_color,
            &mut style.border_width,
            &mut style.shadow,
        );
        style
    }
}

style_overrides!(Button);

impl Widget for Button {
    type Output = ButtonResponse;

    fn ui(&mut self, ui: &mut Ui) -> ButtonResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Button {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let state = ui.widget_state::<ButtonState>(&self.id);
        let weights = StateWeights::IDLE
            .hovered(state.hover_t)
            .pressed(state.press_t);
        let text_style = style.text_style.resolve(weights);
        let padding = style.padding.resolve(weights);
        let min_size = style.min_size.resolve(weights);
        let text_width = ui.measure_text(&self.label, text_style);
        [
            self.width
                .unwrap_or((text_width + padding[0] * 2.0).max(min_size[0])),
            self.height
                .unwrap_or((padding[1] * 2.0 + text_style.line_height).max(min_size[1])),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> ButtonResponse {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();

        let previous = ui.widget_state::<ButtonState>(&self.id);
        let hit_path = style.path.resolve(
            StateWeights::IDLE
                .hovered(previous.hover_t)
                .pressed(previous.press_t),
        );
        let interaction = Interaction::update(&hit_path(position, size), ui);
        self.interaction = interaction;

        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        let state = ui.widget_state::<ButtonState>(&self.id);

        let hover_target = if interaction.hovered { 1.0f32 } else { 0.0 };
        let press_target = if interaction.pressed { 1.0f32 } else { 0.0 };
        if is_anonymous(&self.id) {
            state.hover_t = hover_target;
            state.press_t = press_target;
        } else {
            state.hover_t = animate_towards(state.hover_t, hover_target, dt, style.hover_motion);
            state.press_t = animate_towards(state.press_t, press_target, dt, style.press_motion);
        }
        let weights = StateWeights::IDLE
            .hovered(state.hover_t)
            .pressed(state.press_t);

        let path = style.path.resolve(weights);
        let offset = style.offset.resolve(weights);
        let offset_position = [position[0] + offset[0], position[1] + offset[1]];
        let (draw_position, draw_size) =
            scale_rect(offset_position, size, style.scale.resolve(weights));

        if let Some(shadow) = &style.shadow.resolve(weights) {
            draw_shadow(shadow, draw_position, draw_size, &path, ui);
        }

        ui.draw_shape(
            path(draw_position, draw_size),
            style.fill.resolve(weights),
            style.border_width.resolve(weights),
            style.border_color.resolve(weights),
            0.0,
            style.sharp,
            0.0,
        );

        let text_style = style
            .text_style
            .resolve(weights)
            .or_color(theme.text_primary);
        let padding = style.padding.resolve(weights);
        let text_width = ui.measure_text(&self.label, text_style);
        let text_position = align_text_in(
            draw_position,
            draw_size,
            text_width,
            text_style.line_height,
            padding[0],
            style.text_align,
        );
        let clip_rect = [
            draw_position[0],
            draw_position[1],
            draw_position[0] + draw_size[0],
            draw_position[1] + draw_size[1],
        ];
        ui.draw_text(&self.label, text_style, text_position, clip_rect);

        if interaction.hovered {
            ui.set_cursor_icon(
                style.cursor.resolve(
                    StateWeights::IDLE
                        .hovered(1.0)
                        .pressed(StateWeights::when(interaction.pressed)),
                ),
            );
            if let Some(tooltip) = &self.tooltip {
                ui.show_tooltip(tooltip.clone());
            }
        }

        interaction
    }
}

impl StatefulWidget for Button {
    type State = ButtonState;

    fn state_id(&self) -> &str {
        &self.id
    }

    fn initial_state(&self) -> ButtonState {
        ButtonState::default()
    }
}

impl Accessible for Button {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::Button
    }
    fn accessibility_label(&self) -> Option<String> {
        Some(self.label.clone())
    }
}
