use crate::geometry::center_text_in;
use crate::interaction::Interaction;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

pub type ButtonResponse = Interaction;

pub struct Button {
    label: String,
    corner_radius: f32,
    interaction: Interaction,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Button {
            label: label.into(),
            corner_radius: 12.0,
            interaction: Interaction::default(),
        }
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
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
        let padding = [16.0, 12.0];
        let text_width = ui.measure_text(&self.label);
        [
            text_width + padding[0] * 2.0,
            ui.line_height() + padding[1] * 2.0,
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> ButtonResponse {
        let interaction = Interaction::update(position, size, self.corner_radius, ui);
        self.interaction = interaction;

        let color = Theme::state_color(interaction.pressed, interaction.hovered);

        ui.draw_rect(
            position,
            size,
            Theme::SHADOW,
            self.corner_radius,
            0.0,
            [0.0; 4],
            10.0,
            false,
        );
        ui.draw_rect(
            position,
            size,
            color,
            self.corner_radius,
            1.0,
            Theme::BORDER,
            0.0,
            false,
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
