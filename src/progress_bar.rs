use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, StatefulWidget, Widget};

#[derive(Default)]
pub struct ProgressBarState {
    pub animated_progress: f32,
    pub initialized: bool,
}

#[derive(Debug, Clone)]
pub struct ProgressBarStyle {
    pub track_fill: Fill,
    pub progress_fill: Fill,
    pub border_width: f32,
    pub border_color: Color,
    pub corner_radius: f32,
}

impl Default for ProgressBarStyle {
    fn default() -> Self {
        ProgressBarStyle {
            track_fill: Fill::Solid(Theme::SURFACE_SUBTLE),
            progress_fill: Fill::Solid(Theme::ACTIVE),
            border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: 4.0,
        }
    }
}

pub struct ProgressBar {
    id: Option<String>,
    progress: f32,
    width: f32,
    height: f32,
    style: Option<ProgressBarStyle>,
}

impl ProgressBar {
    pub fn new(progress: f32, width: f32) -> Self {
        ProgressBar {
            id: None,
            progress: progress.clamp(0.0, 1.0),
            width,
            height: 8.0,
            style: None,
        }
    }

    /// Assigns a stable state ID for smooth animated progress transitions.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn style(mut self, style: ProgressBarStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<ProgressBarStyle>) {
        self.style = style;
    }

    /// Colors the progress bar with the semantic success color (Emerald).
    pub fn success(mut self) -> Self {
        let mut s = self.style.take().unwrap_or_default();
        s.progress_fill = Fill::Solid(Theme::SUCCESS);
        self.style = Some(s);
        self
    }

    /// Colors the progress bar with the semantic warning color (Amber).
    pub fn warning(mut self) -> Self {
        let mut s = self.style.take().unwrap_or_default();
        s.progress_fill = Fill::Solid(Theme::WARNING);
        self.style = Some(s);
        self
    }

    /// Colors the progress bar with the semantic error color (Rose).
    pub fn error(mut self) -> Self {
        let mut s = self.style.take().unwrap_or_default();
        s.progress_fill = Fill::Solid(Theme::ERROR);
        self.style = Some(s);
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
        let style = self.style.clone().unwrap_or_default();
        let dt = ui.dt();

        // Draw track
        ui.draw_rect(
            position,
            size,
            style.track_fill.clone(),
            style.corner_radius,
            style.border_width,
            style.border_color,
            0.0,
            false,
            0.0,
        );

        // Smooth progress interpolation if ID is provided, or fallback to auto position-based ID
        let id = self
            .id
            .clone()
            .unwrap_or_else(|| format!("__prog_{}_{}", position[0] as i32, position[1] as i32));
        let state = ui.widget_state::<ProgressBarState>(&id);
        if !state.initialized {
            state.animated_progress = self.progress;
            state.initialized = true;
        }
        state.animated_progress =
            animate_towards(state.animated_progress, self.progress, dt, Motion::FLUID);
        let current_progress = state.animated_progress;

        // Draw filled progress bar with fluid animated width
        let filled_width = (size[0] * current_progress).max(0.0);
        if filled_width > 0.0 {
            ui.draw_rect(
                position,
                [filled_width, size[1]],
                style.progress_fill.clone(),
                style.corner_radius,
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
        self.id.as_deref().unwrap_or("__default_progressbar")
    }

    fn initial_state(&self) -> ProgressBarState {
        ProgressBarState {
            animated_progress: self.progress,
            initialized: true,
        }
    }
}
