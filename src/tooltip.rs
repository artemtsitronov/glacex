use crate::fill::Fill;
use crate::theme::Theme;
use crate::ui::Ui;

pub struct Tooltip;

impl Tooltip {
    pub fn show(hovered: bool, hover_started: Option<std::time::Instant>, text: &str, ui: &mut Ui) {
        let Some(started) = hover_started else { return };
        if !hovered {
            return;
        }

        let delay = 0.5;
        if started.elapsed().as_secs_f32() < delay {
            return;
        }

        let mouse_pos = ui.mouse_position();
        let text = text.to_string();
        let position = [mouse_pos[0] + 12.0, mouse_pos[1] + 12.0]; // small offset from cursor

        ui.queue_overlay(move |ui| {
            let text_width = ui.measure_text(&text);
            let padding = 6.0;
            let size = [text_width + padding * 2.0, ui.line_height() + padding * 2.0];

            ui.draw_rect(
                position,
                size,
                Fill::Solid(Theme::SURFACE),
                4.0,
                1.0,
                Theme::BORDER,
                0.0,
                false,
                0.0,
            );

            let text_position = [position[0] + padding, position[1] + padding];
            let clip_rect = [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ];
            ui.draw_text(&text, text_position, clip_rect);
        });
    }
}
