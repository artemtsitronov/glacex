use crate::color::Color;
use crate::fill::Fill;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Measurable, Widget};

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
    progress: f32,
    width: f32,
    height: f32,
    style: ProgressBarStyle,
}

impl ProgressBar {
    pub fn new(progress: f32, width: f32) -> Self {
        ProgressBar {
            progress: progress.clamp(0.0, 1.0),
            width,
            height: 8.0,
            style: ProgressBarStyle::default(),
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn style(mut self, style: ProgressBarStyle) -> Self {
        self.style = style;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.style.progress_fill = Fill::Solid(color);
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
        // Draw track
        ui.draw_rect(
            position,
            size,
            self.style.track_fill.clone(),
            self.style.corner_radius,
            self.style.border_width,
            self.style.border_color,
            0.0,
            false,
        );

        // Draw filled progress bar
        let filled_width = (size[0] * self.progress).max(0.0);
        if filled_width > 0.0 {
            ui.draw_rect(
                position,
                [filled_width, size[1]],
                self.style.progress_fill.clone(),
                self.style.corner_radius,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
            );
        }
    }
}
