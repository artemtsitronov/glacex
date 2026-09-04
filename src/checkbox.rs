use crate::color::Color;
use crate::fill::Fill;
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, StatefulWidget, Widget};
use winit::window::CursorIcon;

#[derive(Default)]
pub struct CheckboxState {
    pub checked: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct CheckboxResponse {
    pub checked: bool,
    pub clicked: bool,
    pub hovered: bool,
}

#[derive(Debug, Clone)]
pub struct CheckboxStyle {
    pub fill: Fill,
    pub hover_fill: Fill,
    pub checked_fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
    pub shadow: Option<ShadowStyle>,
    pub sharp: bool,
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        CheckboxStyle {
            fill: Fill::Solid(Theme::IDLE),
            hover_fill: Fill::Solid(Theme::HOVERED),
            checked_fill: Fill::Solid(Theme::ACTIVE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: 5.0,
            shadow: Some(ShadowStyle::default()),
            sharp: false,
        }
    }
}

pub struct Checkbox {
    id: String,
    interaction: Interaction,
    style: Option<CheckboxStyle>,
    default_checked: bool,
}

impl Checkbox {
    pub fn new(id: impl Into<String>) -> Self {
        Checkbox {
            id: id.into(),
            interaction: Interaction::default(),
            style: None,
            default_checked: false,
        }
    }

    pub fn style(mut self, style: CheckboxStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<CheckboxStyle>) {
        self.style = style;
    }

    pub fn default_checked(mut self, checked: bool) -> Self {
        self.default_checked = checked;
        self
    }

    pub fn clicked(&self) -> bool {
        self.interaction.clicked
    }
    pub fn hovered(&self) -> bool {
        self.interaction.hovered
    }
}

impl Widget for Checkbox {
    type Output = CheckboxResponse;

    fn ui(&mut self, ui: &mut Ui) -> CheckboxResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Checkbox {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [18.0, 18.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> CheckboxResponse {
        let style = self.style.clone().unwrap_or_default();

        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

        if interaction.hovered {
            ui.set_cursor_icon(CursorIcon::Pointer);
        }

        let state = ui.widget_state_or(&self.id, self.initial_state());

        if interaction.clicked {
            state.checked = !state.checked;
        }
        let checked = state.checked; // copy out before state's borrow needs to end for the draw_rect calls below

        let fill = if checked {
            style.checked_fill
        } else if interaction.hovered {
            style.hover_fill
        } else {
            style.fill
        };

        if let Some(shadow) = &style.shadow {
            draw_shadow(shadow, position, size, style.corner_radius, ui);
        }
        ui.draw_rect(
            position,
            size,
            fill,
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            style.sharp,
        );

        CheckboxResponse {
            checked,
            clicked: interaction.clicked,
            hovered: interaction.hovered,
        }
    }
}

impl StatefulWidget for Checkbox {
    type State = CheckboxState;

    fn state_id(&self) -> &str {
        &self.id
    }

    fn initial_state(&self) -> CheckboxState {
        CheckboxState {
            checked: self.default_checked,
        }
    }
}
