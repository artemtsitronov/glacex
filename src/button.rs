use crate::Fill;
use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::geometry::center_text_in;
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::tooltip::Tooltip;
use crate::ui::Ui;
use crate::widget::{Measurable, StatefulWidget, Widget};
use std::default::Default;
use winit::window::CursorIcon;

pub type ButtonResponse = Interaction;

#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub fill: Fill,
    pub hover_fill: Fill,
    pub pressed_fill: Fill,
    pub text_color: Color,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
    pub shadow: Option<ShadowStyle>,
    pub sharp: bool,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle {
            fill: Fill::Solid(Theme::IDLE),
            hover_fill: Fill::Solid(Theme::HOVERED),
            pressed_fill: Fill::Solid(Theme::PRESSED),
            text_color: Theme::TEXT_PRIMARY,
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: Theme::RADIUS_MD,
            shadow: Some(ShadowStyle::default()),
            sharp: false,
        }
    }
}

impl ButtonStyle {
    /// Primary accent button (Linear / Vercel CTA style).
    pub fn primary() -> Self {
        ButtonStyle {
            fill: Fill::Solid(Theme::ACTIVE),
            hover_fill: Fill::Solid(Theme::ACTIVE_HOVER),
            pressed_fill: Fill::Solid(Theme::ACTIVE.darken(0.12)),
            text_color: Color::WHITE,
            border_width: 1.0,
            border_color: Color::WHITE.with_alpha(0.18),
            corner_radius: Theme::RADIUS_MD,
            shadow: Some(ShadowStyle {
                color: Theme::ACTIVE.with_alpha(0.35),
                blur_radius: 8.0,
                offset: [0.0, 2.0],
            }),
            sharp: false,
        }
    }

    /// Outline button with transparent surface and prominent border.
    pub fn outline() -> Self {
        ButtonStyle {
            fill: Fill::Solid(Color::TRANSPARENT),
            hover_fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            pressed_fill: Fill::Solid(Theme::HOVERED),
            text_color: Theme::TEXT_PRIMARY,
            border_width: 1.0,
            border_color: Theme::BORDER_STRONG,
            corner_radius: Theme::RADIUS_MD,
            shadow: None,
            sharp: false,
        }
    }

    /// Ghost / flat button without background or border until hovered.
    pub fn ghost() -> Self {
        ButtonStyle {
            fill: Fill::Solid(Color::TRANSPARENT),
            hover_fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            pressed_fill: Fill::Solid(Theme::HOVERED),
            text_color: Theme::TEXT_SECONDARY,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            corner_radius: Theme::RADIUS_MD,
            shadow: None,
            sharp: false,
        }
    }

    /// Destructive / danger button for high-consequence actions.
    pub fn danger() -> Self {
        ButtonStyle {
            fill: Fill::Solid(Theme::ERROR.with_alpha(0.14)),
            hover_fill: Fill::Solid(Theme::ERROR.with_alpha(0.24)),
            pressed_fill: Fill::Solid(Theme::ERROR.with_alpha(0.36)),
            text_color: Theme::ERROR,
            border_width: 1.0,
            border_color: Theme::ERROR.with_alpha(0.35),
            corner_radius: Theme::RADIUS_MD,
            shadow: Some(ShadowStyle {
                color: Theme::ERROR.with_alpha(0.25),
                blur_radius: 6.0,
                offset: [0.0, 1.0],
            }),
            sharp: false,
        }
    }
}

/// Per-button animation state -- persists between frames.
pub struct ButtonState {
    /// 0.0 = resting, 1.0 = fully hovered.
    pub hover_t: f32,
    /// 0.0 = resting, 1.0 = fully pressed.
    pub press_t: f32,
    pub hover_started: Option<std::time::Instant>,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState {
            hover_t: 0.0,
            press_t: 0.0,
            hover_started: None,
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
    tooltip_text: Option<String>,
}

impl Button {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Button {
            id: id.into(),
            label: label.into(),
            interaction: Interaction::default(),
            variant: ButtonVariant::Default,
            style: None,
            tooltip_text: None,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<ButtonStyle>) {
        self.style = style;
    }

    /// Applies the primary accent CTA style.
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    /// Applies the outline style.
    pub fn outline(mut self) -> Self {
        self.variant = ButtonVariant::Outline;
        self
    }

    /// Applies the ghost / flat style.
    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        self
    }

    /// Applies the danger / destructive style.
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
}

impl Widget for Button {
    type Output = ButtonResponse;

    fn ui(&mut self, ui: &mut Ui) -> ButtonResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Button {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let padding = [14.0, 8.0];
        let text_width = ui.measure_text_styled(
            &self.label,
            14.0,
            20.0,
            crate::painter::FontWeight::Medium,
            false,
        );
        [text_width + padding[0] * 2.0, 36.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> ButtonResponse {
        let theme = *ui.theme();
        let style = self.style.clone().unwrap_or_else(|| match self.variant {
            ButtonVariant::Default => theme.button_style(),
            ButtonVariant::Primary => theme.primary_button_style(),
            ButtonVariant::Outline => theme.outline_button_style(),
            ButtonVariant::Ghost => theme.ghost_button_style(),
            ButtonVariant::Danger => theme.danger_button_style(),
        });
        let dt = ui.dt();

        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

        // Animate hover and press smoothly using Framer / Linear motion constants
        let state_id = format!(
            "__btn_anim_{}_{}_{}",
            self.label, position[0] as i32, position[1] as i32
        );
        let state = ui.widget_state::<ButtonState>(&state_id);

        let hover_target = if interaction.hovered { 1.0f32 } else { 0.0 };
        let press_target = if interaction.pressed { 1.0f32 } else { 0.0 };
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, Motion::SNAPPY);
        state.press_t = animate_towards(state.press_t, press_target, dt, Motion::INSTANT);
        let hover_t = state.hover_t;
        let press_t = state.press_t;

        // Blend fill colors based on animation progress
        let color = if let (Fill::Solid(base), Fill::Solid(hov), Fill::Solid(prs)) =
            (&style.fill, &style.hover_fill, &style.pressed_fill)
        {
            let blended = base.lerp(*hov, hover_t);
            Fill::Solid(blended.lerp(*prs, press_t))
        } else if interaction.pressed {
            style.pressed_fill
        } else if interaction.hovered {
            style.hover_fill
        } else {
            style.fill
        };

        // Smooth subtle border brightening on hover
        let border_color = if hover_t > 0.01 {
            style.border_color.lerp(theme.border_strong, hover_t)
        } else {
            style.border_color
        };

        // Micro-scale effect: subtle 1px press depth for tactile feel
        let y_offset = press_t * 1.0;
        let draw_position = [position[0], position[1] + y_offset];

        if let Some(shadow) = &style.shadow {
            // Shadow recedes slightly on press for tactile physical feedback
            let mut s = *shadow;
            s.offset[1] -= press_t * 1.0;
            s.blur_radius = (s.blur_radius - press_t * 2.0).max(1.0);
            draw_shadow(&s, draw_position, size, style.corner_radius, ui);
        }

        ui.draw_rect(
            draw_position,
            size,
            color,
            style.corner_radius,
            style.border_width,
            border_color,
            0.0,
            style.sharp,
            0.0,
        );

        let text_width = ui.measure_text_styled(
            &self.label,
            14.0,
            20.0,
            crate::painter::FontWeight::Medium,
            false,
        );
        let text_position = center_text_in(draw_position, size, text_width, 20.0);
        let clip_rect = [
            draw_position[0],
            draw_position[1],
            draw_position[0] + size[0],
            draw_position[1] + size[1],
        ];
        ui.draw_text_styled(
            &self.label,
            text_position,
            clip_rect,
            style.text_color,
            14.0,
            20.0,
            crate::painter::FontWeight::Medium,
            false,
        );

        if interaction.hovered {
            ui.set_cursor_icon(CursorIcon::Pointer);
            if state.hover_started.is_none() {
                state.hover_started = Some(std::time::Instant::now());
            }
        } else {
            state.hover_started = None;
        }

        if let Some(tooltip_text) = &self.tooltip_text {
            Tooltip::show(interaction.hovered, state.hover_started, tooltip_text, ui);
        }

        ui.put_widget_state(&self.id, state);

        interaction
    }
}

impl StatefulWidget for Button {
    type State = ButtonState;

    fn state_id(&self) -> &str {
        &self.label
    }

    fn initial_state(&self) -> ButtonState {
        ButtonState::default()
    }
}
