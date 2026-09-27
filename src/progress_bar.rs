use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::{MeasurablePath, Path};
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, IntoId, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};

#[derive(Default)]
pub struct ProgressBarState {
    pub animated_progress: f32,
    pub initialized: bool,
}

fn default_track_path() -> Path {
    Path::from_fn(|position, size| MeasurablePath::rect(position, size, [size[1] * 0.5; 4]))
}

#[derive(Clone)]
pub struct ProgressBarStyle {
    pub track_fill: Fill,
    pub progress_fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub path: Path,
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        ProgressBarStyle {
            track_fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            progress_fill: Fill::Solid(Theme::ACTIVE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            path: default_track_path(),
        }
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
    width: f32,
    height: f32,
    variant: ProgressBarVariant,
    style: Option<ProgressBarStyle>,
}

impl ProgressBar {
    pub const DEFAULT_WIDTH: f32 = 200.0;
    pub const DEFAULT_HEIGHT: f32 = 6.0;

    pub fn new(progress: f32) -> Self {
        ProgressBar {
            id: uuid::Uuid::new_v4().to_string(),
            progress: progress.clamp(0.0, 1.0),
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            variant: ProgressBarVariant::Default,
            style: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.width = size[0];
        self.height = size[1];
        self
    }

    pub fn style(mut self, style: ProgressBarStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<ProgressBarStyle>) {
        self.style = style;
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

impl Widget for ProgressBar {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for ProgressBar {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.width, self.height]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        let theme = *ui.theme();
        let style = self.style.clone().unwrap_or_else(|| {
            let progress_fill = match self.variant {
                ProgressBarVariant::Default => theme.active,
                ProgressBarVariant::Success => theme.success,
                ProgressBarVariant::Warning => theme.warning,
                ProgressBarVariant::Error => theme.error,
            };
            ProgressBarStyle {
                track_fill: Fill::Solid(theme.surface_subtle),
                progress_fill: Fill::Solid(progress_fill),
                border_width: 1.0,
                border_color: theme.border,
                path: default_track_path(),
            }
        });
        let dt = ui.dt();

        // anonymous progress bars share a fallback id, only register named ones
        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        ui.draw_shape(
            (style.path)(position, size),
            style.track_fill.clone(),
            style.border_width,
            style.border_color,
            0.0,
            false,
            0.0,
        );

        let state = ui.widget_state::<ProgressBarState>(&self.id);
        if !state.initialized {
            state.animated_progress = self.progress;
            state.initialized = true;
        }
        state.animated_progress =
            animate_towards(state.animated_progress, self.progress, dt, Motion::FLUID);
        let current_progress = state.animated_progress;

        let filled_width = current_progress * size[0];

        if filled_width > 0.0 {
            ui.draw_shape(
                (style.path)(position, [filled_width, size[1]]),
                style.progress_fill.clone(),
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
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
