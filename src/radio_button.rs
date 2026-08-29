use crate::interaction::Interaction;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

#[derive(Debug, Clone, Copy)]
pub struct RadioButtonResponse {
    pub selected: bool,
    pub clicked: bool,
    pub hovered: bool,
}

pub struct RadioButton {
    group_id: String,
    option_id: String,
    corner_radius: f32,
    interaction: Interaction,
}

impl RadioButton {
    pub fn new(group_id: impl Into<String>, option_id: impl Into<String>) -> Self {
        RadioButton {
            group_id: group_id.into(),
            option_id: option_id.into(),
            corner_radius: 10.0,
            interaction: Interaction::default(),
        }
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn clicked(&self) -> bool {
        self.interaction.clicked
    }
    pub fn hovered(&self) -> bool {
        self.interaction.hovered
    }
}

impl Widget for RadioButton {
    type Output = RadioButtonResponse;

    fn ui(&mut self, ui: &mut Ui) -> RadioButtonResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for RadioButton {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [20.0, 20.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> RadioButtonResponse {
        let interaction = Interaction::update(position, size, self.corner_radius, ui);
        self.interaction = interaction;

        if interaction.clicked {
            ui.select(&self.group_id, &self.option_id);
        }
        let selected = ui.is_selected(&self.group_id, &self.option_id); // copy out before state's borrow needs to end for the draw_rect calls below

        let color = Theme::state_color(selected, interaction.hovered);

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

        RadioButtonResponse {
            selected,
            clicked: interaction.clicked,
            hovered: interaction.hovered,
        }
    }
}
