use crate::color::Color;
use crate::geometry::contains;
use crate::text_edit::TextEditState;
use crate::ui::Ui;
use crate::widget::{FocusId, Measurable, Widget};

pub struct TextInput {
    id: String,
    focus_id: FocusId,
    width: f32,
    corner_radius: f32,
}

impl TextInput {
    pub fn new(id: impl Into<String>, width: f32) -> Self {
        let id = id.into();
        TextInput {
            focus_id: FocusId::new(&id),
            id,
            width,
            corner_radius: 8.0,
        }
    }

    pub fn focused(&self, ui: &Ui) -> bool {
        ui.is_focused(self.focus_id)
    }
}

impl Widget for TextInput {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

impl Measurable for TextInput {
    fn measure(&mut self, ui: &mut Ui) -> [f32; 2] {
        [self.width, ui.line_height() + 16.0]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        ui.register_focusable(self.focus_id);

        let padding = 10.0;

        let mouse_pos = ui.mouse_position();
        let hovered = !ui.is_input_blocked(mouse_pos)
            && contains(position, size, self.corner_radius, mouse_pos);
        let focused = self.focused(ui);

        let text_position = [
            position[0] + padding,
            position[1] + (size[1] - ui.line_height()) / 2.0,
        ];

        let mut state = ui.take_widget_state::<TextEditState>(&self.id);
        state.hovered = hovered;

        if ui.mouse_pressed_this_frame() && hovered {
            ui.request_focus(self.focus_id);
            state.dragging = true;
            state.mark_activity();
            let click_x = ui.mouse_position()[0] - text_position[0] + state.scroll_offset();
            let index = state.cursor_index_for_x(ui, click_x);
            match ui.click_count() {
                1 => {
                    state.set_cursor(index);
                    state.set_selection_anchor(Some(index));
                }
                2 => {
                    let start = state.word_start(index);
                    let end = state.word_end(index);
                    state.set_selection_anchor(Some(start));
                    state.set_cursor(end);
                }
                _ => {
                    let char_count = state.text().chars().count();
                    state.set_selection_anchor(Some(0));
                    state.set_cursor(char_count);
                }
            }
        }

        if !ui.mouse_pressed() {
            state.dragging = false;
        }

        if ui.mouse_pressed() && state.dragging && ui.click_count() == 1 {
            state.mark_activity();
            let drag_x = ui.mouse_position()[0] - text_position[0] + state.scroll_offset();
            let index = state.cursor_index_for_x(ui, drag_x);
            state.set_cursor(index);
        }

        if focused {
            state.handle_input(ui);
        }

        let text_width = size[0] - padding * 2.0;
        let cursor_x = ui.measure_text(state.prefix());
        state.scroll_into_view(cursor_x, text_width);

        let border_color = if focused {
            [0.3, 0.4, 0.85, 1.0]
        } else {
            [0.0, 0.0, 0.0, 0.1]
        };

        ui.draw_rect(
            position,
            size,
            Color::rgba(0, 0, 0, 0.35),
            self.corner_radius,
            0.0,
            [0.0; 4],
            10.0,
            false,
        );

        ui.draw_rect(
            position,
            size,
            Color::rgb(30, 30, 34),
            self.corner_radius,
            1.0,
            border_color,
            0.0,
            false,
        );

        if let Some((start, end)) = state.selection_range() {
            let prefix_start = ui.measure_text(&state.text()[..state.byte_index_for(start)]);
            let prefix_end = ui.measure_text(&state.text()[..state.byte_index_for(end)]);

            let highlight_position = [
                (text_position[0] + prefix_start - state.scroll_offset()).round(),
                text_position[1],
            ];
            let highlight_size = [prefix_end - prefix_start, ui.line_height()];

            ui.draw_rect(
                highlight_position,
                highlight_size,
                Color::rgba(76, 95, 213, 0.35),
                0.0,
                0.0,
                [0.0; 4],
                0.0,
                false,
            );
        }

        let clip_rect = [
            position[0] + padding,
            position[1],
            position[0] + size[0] - padding,
            position[1] + size[1],
        ];
        ui.draw_text(
            state.text(),
            [text_position[0] - state.scroll_offset(), text_position[1]],
            clip_rect,
        );

        if focused {
            let idle_seconds = state.last_activity().elapsed().as_secs_f32();
            let cursor_visible = idle_seconds < 0.3 || (idle_seconds % 1.0) < 0.5;
            if cursor_visible {
                let cursor_position = [
                    (text_position[0] + cursor_x - state.scroll_offset()).round(),
                    text_position[1],
                ];
                ui.draw_rect(
                    cursor_position,
                    [2.0, ui.line_height()],
                    Color::WHITE,
                    0.0,
                    0.0,
                    [0.0; 4],
                    0.0,
                    true,
                );
            }
        }

        ui.put_widget_state(&self.id, state);
    }
}
