use crate::interaction::Interaction;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

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

pub struct Checkbox {
    id: String,
    corner_radius: f32,
    interaction: Interaction,
}

impl Checkbox {
    pub fn new(id: impl Into<String>) -> Self {
        Checkbox {
            id: id.into(),
            corner_radius: 12.0,
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

impl Widget for Checkbox {
    type Output = CheckboxResponse;

    fn ui(&mut self, ui: &mut Ui) -> CheckboxResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for Checkbox {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [30.0, 30.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> CheckboxResponse {
        let interaction = Interaction::update(position, size, self.corner_radius, ui);
        self.interaction = interaction;

        let state = ui.widget_state::<CheckboxState>(&self.id);
        if interaction.clicked {
            state.checked = !state.checked;
        }
        let checked = state.checked; // copy out before state's borrow needs to end for the draw_rect calls below

        let color = Theme::state_color(checked, interaction.hovered);

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

        CheckboxResponse {
            checked,
            clicked: interaction.clicked,
            hovered: interaction.hovered,
        }
    }
}
