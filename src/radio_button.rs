use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::{MeasurablePath, Path};
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

use crate::animation::{Motion, animate_towards};

#[macro_export]
macro_rules! radio_button {    // quick macro to use uuid as id for the widget
    ( $( $group_id:expr ),+ ) => {
        use uuid::Uuid;

        let id = Uuid::new_v4().to_string();
        RadioButton::new($( $group_id ),+, id)
    };
}

#[derive(Debug, Clone, Copy)]
pub struct RadioButtonResponse {
    pub selected: bool,
    pub clicked: bool,
    pub hovered: bool,
}

#[derive(Clone)]
pub struct RadioButtonStyle {
    pub fill: Fill,
    pub hover_fill: Fill,
    pub selected_fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub shadow: Option<ShadowStyle>,
    pub sharp: bool,
    // sits on selected_fill so it should contrast with it
    pub dot_color: Color,
    pub path: Path,
}

impl Default for RadioButtonStyle {
    fn default() -> Self {
        RadioButtonStyle {
            fill: Fill::Solid(Theme::IDLE),
            hover_fill: Fill::Solid(Theme::HOVERED),
            selected_fill: Fill::Solid(Theme::ACTIVE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            shadow: Some(ShadowStyle::default()),
            sharp: false,
            dot_color: Color::WHITE,
            path: Path::ellipse(0.0),
        }
    }
}

pub struct RadioButtonAnimState {
    pub dot_t: f32,
    pub hover_t: f32,
}

impl Default for RadioButtonAnimState {
    fn default() -> Self {
        RadioButtonAnimState {
            dot_t: 0.0,
            hover_t: 0.0,
        }
    }
}

pub struct RadioButton {
    group_id: String,
    option_id: String,
    style: Option<RadioButtonStyle>,
    interaction: Interaction,
    width: Option<f32>,
    height: Option<f32>,
}

impl RadioButton {
    pub fn new(group_id: impl Into<String>, option_id: impl Into<String>) -> Self {
        RadioButton {
            group_id: group_id.into(),
            option_id: option_id.into(),
            style: None,
            interaction: Interaction::default(),
            width: None,
            height: None,
        }
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

    pub fn style(mut self, style: RadioButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<RadioButtonStyle>) {
        self.style = style;
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
        [self.width.unwrap_or(18.0), self.height.unwrap_or(18.0)]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> RadioButtonResponse {
        let theme = *ui.theme();
        let style = self.style.clone().unwrap_or(RadioButtonStyle {
            fill: Fill::Solid(theme.idle),
            hover_fill: Fill::Solid(theme.hovered),
            selected_fill: Fill::Solid(theme.active),
            border_width: 1.0,
            border_color: theme.border,
            shadow: Some(theme.shadow_sm()[0]),
            sharp: false,
            dot_color: theme.on_active(),
            path: Path::ellipse(0.0),
        });
        let dt = ui.dt();

        let shape = (style.path)(position, size);
        let interaction = Interaction::update(&shape, ui);
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

        if interaction.hovered {
            ui.set_cursor_icon(CursorIcon::Pointer);
        }

        if interaction.clicked {
            ui.select(&self.group_id, &self.option_id);
        }
        let selected = ui.is_selected(&self.group_id, &self.option_id);

        let anim_id = format!("__radio_anim_{}_{}", self.group_id, self.option_id);
        let anim = ui.widget_state::<RadioButtonAnimState>(&anim_id);
        anim.dot_t = animate_towards(
            anim.dot_t,
            if selected { 1.0 } else { 0.0 },
            dt,
            Motion::FLUID,
        );
        anim.hover_t = animate_towards(
            anim.hover_t,
            if interaction.hovered { 1.0 } else { 0.0 },
            dt,
            Motion::SNAPPY,
        );
        let dot_t = anim.dot_t;
        let hover_t = anim.hover_t;

        let fill = if let (Fill::Solid(idle), Fill::Solid(hov), Fill::Solid(sel)) =
            (&style.fill, &style.hover_fill, &style.selected_fill)
        {
            let idle_or_hover = idle.lerp(*hov, hover_t);
            Fill::Solid(idle_or_hover.lerp(*sel, dot_t))
        } else if selected {
            style.selected_fill
        } else if interaction.hovered {
            style.hover_fill
        } else {
            style.fill
        };

        let border_color = if dot_t > 0.01 {
            style.border_color.lerp(theme.active, dot_t * 0.4)
        } else if hover_t > 0.01 {
            style.border_color.lerp(theme.border_strong, hover_t)
        } else {
            style.border_color
        };

        if let Some(shadow) = &style.shadow {
            draw_shadow(shadow, position, size, &style.path, ui);
        }

        ui.draw_shape(
            (style.path)(position, size),
            fill,
            style.border_width,
            border_color,
            0.0,
            style.sharp,
            0.0,
        );

        if dot_t > 0.01 {
            let max_inset = 5.0;
            let inset = max_inset + (1.0 - dot_t) * max_inset;
            let dot_size = (size[0] - inset * 2.0).max(0.0);
            let dot_pos = [position[0] + inset, position[1] + inset];
            let dot_radius = dot_size / 2.0;
            let dot_color = style.dot_color.with_alpha(dot_t);
            ui.draw_shape(
                MeasurablePath::rect(dot_pos, [dot_size, dot_size], [dot_radius; 4]),
                Fill::Solid(dot_color),
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }

        RadioButtonResponse {
            selected,
            clicked: interaction.clicked,
            hovered: interaction.hovered,
        }
    }
}

impl Accessible for RadioButton {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&format!("{}:{}", self.group_id, self.option_id)))
    }
    fn accessibility_role(&self) -> Role {
        Role::RadioButton
    }
    fn accessibility_label(&self) -> Option<String> {
        Some(self.option_id.clone())
    }
}
