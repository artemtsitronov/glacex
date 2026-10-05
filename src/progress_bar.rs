use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::generate_uuid;
use crate::geometry::{MeasurablePath, Path};
use crate::interaction::Interaction;
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::stateful::{StateWeights, Stateful, StyleOverrides, style_overrides};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, Measurable, StatefulWidget, Widget, hash_id, is_anonymous};
use accesskit::{NodeId, Role};

#[derive(Default)]
pub struct ProgressBarState {
    pub animated_progress: f32,
    pub hover_t: f32,
    pub initialized: bool,
}

fn default_track_path() -> Path {
    Path::from_fn(|position, size| MeasurablePath::rect(position, size, [size[1] * 0.5; 4]))
}

#[derive(Clone)]
pub struct ProgressBarStyle {
    pub track_fill: Stateful<Fill>,
    pub progress_fill: Stateful<Fill>,
    pub border_width: Stateful<f32>,
    pub border_color: Stateful<Color>,
    pub progress_border_width: Stateful<f32>,
    pub progress_border_color: Stateful<Color>,
    pub progress_inset: Stateful<f32>,
    pub min_progress_width: Stateful<f32>,
    pub size: Stateful<[f32; 2]>,
    pub shadow: Stateful<Option<ShadowStyle>>,
    pub sharp: bool,
    pub path: Stateful<Path>,
    pub progress_path: Stateful<Path>,
    pub progress_motion: f32,
    pub hover_motion: f32,
}

impl ProgressBarStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        ProgressBarStyle {
            track_fill: Stateful::new(Fill::Solid(theme.surface_subtle)),
            progress_fill: Stateful::new(Fill::Solid(theme.active)),
            border_width: Stateful::new(1.0),
            border_color: Stateful::new(theme.border),
            progress_border_width: Stateful::new(0.0),
            progress_border_color: Stateful::new(Color::TRANSPARENT),
            progress_inset: Stateful::new(0.0),
            min_progress_width: Stateful::new(0.0),
            size: Stateful::new([ProgressBar::DEFAULT_WIDTH, ProgressBar::DEFAULT_HEIGHT]),
            shadow: Stateful::new(None),
            sharp: false,
            path: Stateful::new(default_track_path()),
            progress_path: Stateful::new(default_track_path()),
            progress_motion: Motion::FLUID,
            hover_motion: Motion::SNAPPY,
        }
    }

    pub fn success(theme: &Theme) -> Self {
        Self::colored(theme, theme.success)
    }

    pub fn warning(theme: &Theme) -> Self {
        Self::colored(theme, theme.warning)
    }

    pub fn error(theme: &Theme) -> Self {
        Self::colored(theme, theme.error)
    }

    pub fn colored(theme: &Theme, color: Color) -> Self {
        ProgressBarStyle {
            progress_fill: Stateful::new(Fill::Solid(color)),
            ..ProgressBarStyle::from_theme(theme)
        }
    }
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        ProgressBarStyle::from_theme(&Theme::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressBarVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
}

pub struct ProgressBar {
    id: String,
    progress: f32,
    width: Option<f32>,
    height: Option<f32>,
    variant: ProgressBarVariant,
    style: Option<ProgressBarStyle>,
    overrides: StyleOverrides,
}

impl ProgressBar {
    pub const DEFAULT_WIDTH: f32 = 200.0;
    pub const DEFAULT_HEIGHT: f32 = 6.0;

    pub fn new(progress: f32) -> Self {
        ProgressBar {
            id: generate_uuid(),
            progress: progress.clamp(0.0, 1.0),
            width: None,
            height: None,
            variant: ProgressBarVariant::Default,
            style: None,
            overrides: StyleOverrides::default(),
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

    pub fn style(mut self, style: ProgressBarStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<ProgressBarStyle>) {
        self.style = style;
    }

    fn resolved_style(&self, theme: &Theme) -> ProgressBarStyle {
        let mut style = self.style.clone().unwrap_or_else(|| match self.variant {
            ProgressBarVariant::Default => ProgressBarStyle::from_theme(theme),
            ProgressBarVariant::Success => ProgressBarStyle::success(theme),
            ProgressBarVariant::Warning => ProgressBarStyle::warning(theme),
            ProgressBarVariant::Error => ProgressBarStyle::error(theme),
        });
        self.overrides.apply(
            &mut style.track_fill,
            &mut style.border_color,
            &mut style.border_width,
            &mut style.shadow,
        );
        style
    }

    pub fn success(mut self) -> Self {
        self.variant = ProgressBarVariant::Success;
        self
    }

    pub fn warning(mut self) -> Self {
        self.variant = ProgressBarVariant::Warning;
        self
    }

    pub fn error(mut self) -> Self {
        self.variant = ProgressBarVariant::Error;
        self
    }
}

style_overrides!(ProgressBar);

impl Widget for ProgressBar {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for ProgressBar {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        let style = self.resolved_style(ui.theme());
        let hover_t = ui.widget_state::<ProgressBarState>(&self.id).hover_t;
        let size = style.size.resolve(StateWeights::IDLE.hovered(hover_t));
        [
            self.width.unwrap_or(size[0]),
            self.height.unwrap_or(size[1]),
        ]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let theme = *ui.theme();
        let style = self.resolved_style(&theme);
        let dt = ui.dt();

        // anonymous progress bars share a fallback id, only register named ones
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

        let previous = ui.widget_state::<ProgressBarState>(&self.id).hover_t;
        let hit_path = style.path.resolve(StateWeights::IDLE.hovered(previous));
        let hovered = Interaction::update(&hit_path(position, size), ui).hovered;

        let state = ui.widget_state::<ProgressBarState>(&self.id);
        if !state.initialized {
            state.animated_progress = self.progress;
            state.initialized = true;
        }
        state.animated_progress = animate_towards(
            state.animated_progress,
            self.progress,
            dt,
            style.progress_motion,
        );
        let hover_target = if hovered { 1.0 } else { 0.0 };
        state.hover_t = if is_anonymous(&self.id) {
            hover_target
        } else {
            animate_towards(state.hover_t, hover_target, dt, style.hover_motion)
        };
        let current_progress = state.animated_progress;
        let weights = StateWeights::IDLE.hovered(state.hover_t);

        let path = style.path.resolve(weights);
        if let Some(shadow) = &style.shadow.resolve(weights) {
            draw_shadow(shadow, position, size, &path, ui);
        }

        ui.draw_shape(
            path(position, size),
            style.track_fill.resolve(weights),
            style.border_width.resolve(weights),
            style.border_color.resolve(weights),
            0.0,
            style.sharp,
            0.0,
        );

        let inset = style.progress_inset.resolve(weights);
        let inner_position = [position[0] + inset, position[1] + inset];
        let inner_size = [
            (size[0] - inset * 2.0).max(0.0),
            (size[1] - inset * 2.0).max(0.0),
        ];
        let filled_width = if current_progress > 0.0 {
            (current_progress * inner_size[0])
                .max(style.min_progress_width.resolve(weights))
                .min(inner_size[0])
        } else {
            0.0
        };

        if filled_width > 0.0 {
            ui.draw_shape(
                (style.progress_path.resolve(weights))(
                    inner_position,
                    [filled_width, inner_size[1]],
                ),
                style.progress_fill.resolve(weights),
                style.progress_border_width.resolve(weights),
                style.progress_border_color.resolve(weights),
                0.0,
                style.sharp,
                0.0,
            );
        }
    }
}

impl StatefulWidget for ProgressBar {
    type State = ProgressBarState;

    fn state_id(&self) -> &str {
        &self.id
    }

    fn initial_state(&self) -> ProgressBarState {
        ProgressBarState {
            animated_progress: self.progress,
            hover_t: 0.0,
            initialized: true,
        }
    }
}

impl Accessible for ProgressBar {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::ProgressIndicator
    }
}
