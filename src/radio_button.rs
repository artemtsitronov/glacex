use crate::animation::{Motion, animate_towards};
use crate::border::Border;
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::Path;
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, Widget, hash_id, warn_if_anonymous};
use accesskit::{NodeId, Role};
use winit::window::CursorIcon;

#[derive(Debug, Clone, Copy)]
pub struct RadioButtonResponse {
    pub selected: bool,
    pub clicked: bool,
    pub hovered: bool,
}

#[derive(Clone)]
pub struct RadioButtonStyle {
    pub fill: Stateful<Fill>,
    pub border: Stateful<Border>,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub sharp: bool,
    // sits on selected fill so it should contrast with it
    pub dot_color: Stateful<Color>,
    pub dot_inset: Stateful<f32>,
    pub dot_path: Stateful<Path>,
    pub path: Stateful<Path>,
    pub size: Stateful<[f32; 2]>,
    pub dot_motion: f32,
    pub hover_motion: f32,
    pub cursor: Stateful<CursorIcon>,
}

impl RadioButtonStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        RadioButtonStyle {
            fill: Stateful::new(Fill::Solid(theme.idle))
                .hovered(Fill::Solid(theme.hovered))
                .selected(Fill::Solid(theme.active)),
            border: Stateful::new(theme.border)
                .hovered(theme.border_strong)
                .selected(theme.border.lerp(theme.active, 0.4))
                .map(|&color| Border::new(1.0, color)),
            shadow: Stateful::new(Some(theme.shadow_sm()[0])),
            sharp: false,
            dot_color: Stateful::new(theme.on_active()),
            dot_inset: Stateful::new(5.0),
            dot_path: Stateful::new(Path::ellipse(0.0)),
            path: Stateful::new(Path::ellipse(0.0)),
            size: Stateful::new([18.0, 18.0]),
            dot_motion: Motion::FLUID,
            hover_motion: Motion::SNAPPY,
            cursor: Stateful::new(CursorIcon::Pointer),
        }
    }

    pub fn outlined(theme: &Theme) -> Self {
        let base = Self::from_theme(theme);
        RadioButtonStyle {
            fill: base
                .fill
                .clone()
                .selected(Fill::Solid(theme.surface))
                .selected_hovered(Fill::Solid(theme.surface_subtle)),
            border: base
                .border
                .map(|border| border.with_width(1.5))
                .selected(Border::new(1.5, theme.active)),
            dot_color: Stateful::new(theme.active),
            ..base
        }
    }
}

impl Default for RadioButtonStyle {
    fn default() -> Self {
        RadioButtonStyle::from_theme(&Theme::default())
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
    overrides: StyleOverrides,
    interaction: Interaction,
    width: Option<f32>,
    height: Option<f32>,
}

impl RadioButton {
    pub fn new(group_id: impl Into<String>) -> Self {
        RadioButton {
            group_id: group_id.into(),
            option_id: generate_uuid(),
            style: None,
            overrides: StyleOverrides::default(),
            interaction: Interaction::default(),
            width: None,
            height: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.option_id = id.into();
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

    pub fn style(mut self, style: RadioButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<RadioButtonStyle>) {
        self.style = style;
    }

    fn resolved_style(&self, theme: &Theme) -> RadioButtonStyle {
        let mut style = self
            .style
            .clone()
            .unwrap_or_else(|| RadioButtonStyle::from_theme(theme));
        self.overrides
            .apply(&mut style.fill, &mut style.border, &mut style.shadow);
        style
    }

    fn anim_id(&self) -> String {
        format!("__radio_anim_{}_{}", self.group_id, self.option_id)
    }

    pub fn clicked(&self) -> bool {
        self.interaction.clicked
    }
    pub fn hovered(&self) -> bool {
        self.interaction.hovered
    }
}

style_overrides!(RadioButton);

impl Widget for RadioButton {
    type Output = RadioButtonResponse;

    fn ui(&mut self, ui: &mut Ui) -> RadioButtonResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

impl Measurable for RadioButton {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let style = self.resolved_style(ui.theme());
        let previous = ui.widget_state::<RadioButtonAnimState>(&self.anim_id());
        let size = style.size.resolve(
            StateWeights::IDLE
                .hovered(previous.hover_t)
                .selected(previous.dot_t),
        );
        [
            self.width.unwrap_or(size[0]),
            self.height.unwrap_or(size[1]),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) -> RadioButtonResponse {
        warn_if_anonymous("RadioButton", &self.option_id);
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();

        let anim_id = self.anim_id();
        let previous = ui.widget_state::<RadioButtonAnimState>(&anim_id);
        let hit_path = style.path.resolve(
            StateWeights::IDLE
                .hovered(previous.hover_t)
                .selected(previous.dot_t),
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

        if interaction.hovered {
            let selected = ui.is_selected(&self.group_id, &self.option_id);
            ui.set_cursor_icon(
                style.cursor.resolve(
                    StateWeights::IDLE
                        .hovered(1.0)
                        .pressed(StateWeights::when(interaction.pressed))
                        .selected(StateWeights::when(selected)),
                ),
            );
        }

        if interaction.clicked {
            ui.select(&self.group_id, &self.option_id);
        }
        let selected = ui.is_selected(&self.group_id, &self.option_id);

        let anim = ui.widget_state::<RadioButtonAnimState>(&anim_id);
        anim.dot_t = animate_towards(
            anim.dot_t,
            if selected { 1.0 } else { 0.0 },
            dt,
            style.dot_motion,
        );
        anim.hover_t = animate_towards(
            anim.hover_t,
            if interaction.hovered { 1.0 } else { 0.0 },
            dt,
            style.hover_motion,
        );
        let dot_t = anim.dot_t;
        let hover_t = anim.hover_t;

        let weights = StateWeights::IDLE.hovered(hover_t).selected(dot_t);
        let path = style.path.resolve(weights);

        if let Some(shadow) = &style.shadow.resolve(weights) {
            draw_shadow(shadow, position, size, &path, ui);
        }

        ui.draw_shape(
            path(position, size),
            style.fill.resolve(weights),
            style.border.resolve(weights),
            0.0,
            style.sharp,
            0.0,
        );

        if dot_t > 0.01 {
            let dot_inset = style.dot_inset.resolve(weights);
            let inset = dot_inset + (1.0 - dot_t) * dot_inset;
            let dot_size = (size[0].min(size[1]) - inset * 2.0).max(0.0);
            let dot_pos = [
                position[0] + (size[0] - dot_size) / 2.0,
                position[1] + (size[1] - dot_size) / 2.0,
            ];
            let dot_color = style.dot_color.resolve(weights);
            let dot_color = dot_color.with_alpha(dot_color.a * dot_t);
            ui.draw_shape(
                (style.dot_path.resolve(weights))(dot_pos, [dot_size, dot_size]),
                Fill::Solid(dot_color),
                Border::NONE,
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
