use crate::Fill;
use crate::geometry::center_text_in;
use crate::interaction::Interaction;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

pub type ButtonResponse = Interaction;

#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub fill: Fill,
    pub hover_fill: Fill,
    pub pressed_fill: Fill,
    pub border_width: f32,
    pub border_color: [f32; 4],
    pub corner_radius: f32,
    pub sharp: bool,
}

impl ButtonStyle {
    pub fn default_style() -> Self {
        ButtonStyle {
            fill: Fill::Solid(Theme::IDLE),
            hover_fill: Fill::Solid(Theme::HOVERED),
            pressed_fill: Fill::Solid(Theme::ACTIVE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: 12.0,
            sharp: false,
        }
    }
}

pub struct Button {
    label: String,
    interaction: Interaction,
    style: Option<ButtonStyle>,
}

impl Button {
    pub fn new(label: impl Into<String>, style: Option<ButtonStyle>) -> Self {
        Button {
            label: label.into(),
            interaction: Interaction::default(),
            style,
        }
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
        let padding = [16.0, 12.0];
        let text_width = ui.measure_text(&self.label);
        [
            text_width + padding[0] * 2.0,
            ui.line_height() + padding[1] * 2.0,
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> ButtonResponse {
        let style = self
            .style
            .clone()
            .unwrap_or_else(ButtonStyle::default_style);

        let interaction = Interaction::update(position, size, style.corner_radius, ui);
        self.interaction = interaction;

        let color = if interaction.pressed {
            style.pressed_fill
        } else if interaction.hovered {
            style.hover_fill
        } else {
            style.fill
        };

        ui.draw_rect(
            //shadow
            position,
            size,
            Fill::Solid(Theme::SHADOW),
            style.corner_radius,
            0.0,
            [0.0; 4],
            10.0,
            false,
        );
        ui.draw_rect(
            position,
            size,
            color,
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            style.sharp,
        );

        let text_width = ui.measure_text(&self.label);
        let text_position = center_text_in(position, size, text_width, ui.line_height());
        let clip_rect = [
            position[0],
            position[1],
            position[0] + size[0],
            position[1] + size[1],
        ];
        ui.draw_text(&self.label, text_position, clip_rect);

        interaction
    }
}
