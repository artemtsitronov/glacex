use crate::animation::{Motion, animate_towards};
use crate::color::Color;
use crate::fill::Fill;
use crate::geometry::contains;
use crate::scrolling::{ScrollAxisState, ScrollConfig, compute_geometry, handle_drag};
use crate::shadow::{ShadowStyle, draw_shadow};
use crate::text_edit::TextEditState;
use crate::theme::Theme;
use crate::ui::Ui;
use crate::widget::{Accessible, FocusId, Measurable, StatefulWidget, Widget, hash_id};
use accesskit::{NodeId, Role};
use winit::keyboard::{Key, NamedKey};
use winit::window::CursorIcon;

#[derive(Default)]
struct TextAreaExtra {
    preferred_column: Option<usize>,
    scroll: ScrollAxisState,
    scroll_x: f32,
    text_dragging: bool,
}

#[derive(Debug, Clone)]
pub struct TextAreaStyle {
    pub fill: Fill,
    pub text_color: Color,
    pub border_width: f32,
    pub border_color: Color,
    pub focus_border_color: Color,
    pub corner_radius: f32,
    pub padding: [f32; 2],
    pub selection_color: Color,
    pub cursor_color: Color,
    pub thumb_fill: Fill,
    pub thumb_dragging_fill: Fill,
    pub shadow: Option<ShadowStyle>,
    pub sharp: bool,
}

impl Default for TextAreaStyle {
    fn default() -> Self {
        TextAreaStyle {
            fill: Fill::Solid(Theme::SURFACE),
            text_color: Theme::TEXT_PRIMARY,
            border_width: 1.0,
            border_color: Theme::BORDER,
            focus_border_color: Theme::FOCUS_BORDER,
            corner_radius: Theme::RADIUS_MD,
            padding: [10.0, 10.0],
            selection_color: Theme::SELECTION,
            cursor_color: Theme::ACTIVE,
            thumb_fill: Fill::Solid(Color::rgb(113, 113, 122).with_alpha(0.45)),
            thumb_dragging_fill: Fill::Solid(Color::rgb(82, 82, 91).with_alpha(0.75)),
            shadow: Some(ShadowStyle {
                color: Theme::SURFACE_SHADOW,
                blur_radius: 8.0,
                offset: [0.0, 1.0],
            }),
            sharp: false,
        }
    }
}

pub struct TextArea {
    id: String,
    extra_id: String,
    focus_id: FocusId,
    width: f32,
    height: f32,
    style: Option<TextAreaStyle>,
    default_text: String,
    custom_padding: Option<[f32; 2]>,
}

impl TextArea {
    pub const DEFAULT_WIDTH: f32 = 240.0;
    pub const DEFAULT_HEIGHT: f32 = 120.0;

    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        let extra_id = format!("{id}__extra");
        TextArea {
            focus_id: FocusId::new(&id),
            extra_id,
            id,
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            style: None,
            default_text: String::new(),
            custom_padding: None,
        }
    }

    pub fn padding(mut self, padding: [f32; 2]) -> Self {
        self.custom_padding = Some(padding);
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

    pub fn style(mut self, style: TextAreaStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn set_style(&mut self, style: Option<TextAreaStyle>) {
        self.style = style;
    }

    pub fn default_text(mut self, text: impl Into<String>) -> Self {
        self.default_text = text.into();
        self
    }

    pub fn focused(&self, ui: &Ui) -> bool {
        ui.is_focused(self.focus_id)
    }

    fn resolved_style(&self, theme: &Theme) -> TextAreaStyle {
        let mut style = self
            .style
            .clone()
            .unwrap_or_else(|| theme.text_area_style());
        if let Some(p) = self.custom_padding {
            style.padding = p;
        }
        style
    }
}

impl Widget for TextArea {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui);
    }
}

fn line_start_char_index(text: &str, line_index: usize) -> usize {
    if line_index == 0 {
        return 0;
    }
    text.chars()
        .enumerate()
        .filter(|(_, c)| *c == '\n')
        .nth(line_index - 1)
        .map(|(i, _)| i + 1)
        .unwrap_or(text.chars().count())
}

fn char_index_at_point(
    state: &TextEditState,
    ui: &mut Ui,
    line_height: f32,
    relative_pos: [f32; 2],
) -> usize {
    let text = state.text();
    let line_count = state.line_count();

    let raw_line = (relative_pos[1] / line_height).floor();
    let line_index = if raw_line < 0.0 {
        0
    } else {
        (raw_line as usize).min(line_count.saturating_sub(1))
    };

    let line_start = line_start_char_index(text, line_index);
    let line_end = state.line_end(line_start);
    let line_start_byte = state.byte_index_for(line_start);
    let line_end_byte = state.byte_index_for(line_end);
    let line_text = &text[line_start_byte..line_end_byte];

    let mut best_index = line_start;
    let mut best_distance = f32::MAX;
    for (char_offset, (byte_offset, _)) in line_text.char_indices().enumerate().chain(
        std::iter::once((line_text.chars().count(), (line_text.len(), ' '))),
    ) {
        let prefix_width = ui.measure_text(&line_text[..byte_offset]);
        let distance = (prefix_width - relative_pos[0]).abs();
        if distance < best_distance {
            best_distance = distance;
            best_index = line_start + char_offset;
        }
    }
    best_index
}

impl Measurable for TextArea {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [self.width, self.height]
    }

    fn arrange(&mut self, position: [f32; 2], size: [f32; 2], ui: &mut Ui) {
        ui.register_focusable(self.focus_id);

        let theme = *ui.theme();
        let style = self.resolved_style(&theme);

        let padding_x = style.padding[0];
        let padding_y = style.padding[1];
        let config = ScrollConfig::default();

        ui.register_accessible(
            self,
            [
                position[0],
                position[1],
                position[0] + size[0],
                position[1] + size[1],
            ],
        );

        let mouse_pos = ui.mouse_position();
        let hovered = !ui.is_input_blocked(mouse_pos)
            && ui.point_in_current_clip(mouse_pos)
            && contains(position, size, style.corner_radius, mouse_pos);

        let focused = self.focused(ui);

        if hovered {
            ui.set_cursor_icon(CursorIcon::Text);
        }

        if ui.mouse_pressed_this_frame() && hovered {
            ui.request_focus(self.focus_id);
        }

        let mut state = ui.take_widget_state_or(&self.id, self.initial_state());
        let mut extra = ui.take_widget_state::<TextAreaExtra>(&self.extra_id);

        let enter = ui.key_pressed(Key::Named(NamedKey::Enter));
        let up = ui.key_pressed(Key::Named(NamedKey::ArrowUp));
        let down = ui.key_pressed(Key::Named(NamedKey::ArrowDown));

        let home = ui.key_pressed(Key::Named(NamedKey::Home));
        let end = ui.key_pressed(Key::Named(NamedKey::End));

        let home_target = if home {
            Some(state.line_start(state.cursor()))
        } else {
            None
        };
        let end_target = if end {
            Some(state.line_end(state.cursor()))
        } else {
            None
        };

        if focused {
            state.handle_input(ui);

            if let Some(target) = home_target {
                state.set_cursor(target);
            }
            if let Some(target) = end_target {
                state.set_cursor(target);
            }

            if enter {
                state.insert_newline();
                state.mark_activity();
            }

            let non_vertical_activity = enter
                || ui.key_pressed(Key::Named(NamedKey::ArrowLeft))
                || ui.key_pressed(Key::Named(NamedKey::ArrowRight))
                || !ui.typed_text().is_empty();
            if non_vertical_activity {
                extra.preferred_column = None;
            }

            if up {
                let column = extra
                    .preferred_column
                    .unwrap_or_else(|| state.column_of(state.cursor()));
                let new_cursor = state.move_cursor_up(Some(column));
                state.set_cursor(new_cursor);
                extra.preferred_column = Some(column);
                state.mark_activity();
            }

            if down {
                let column = extra
                    .preferred_column
                    .unwrap_or_else(|| state.column_of(state.cursor()));
                let new_cursor = state.move_cursor_down(Some(column));
                state.set_cursor(new_cursor);
                extra.preferred_column = Some(column);
                state.mark_activity();
            }
        }

        let line_height = ui.line_height();
        let content_height = state.line_count() as f32 * line_height;
        let visible_height = size[1] - padding_y * 2.0;
        let track_length = size[1] - config.padding * 2.0;

        let track_x = position[0] + size[0] - config.thickness - config.padding;
        let track_rect_position = [track_x, position[1]];
        let track_rect_size = [config.thickness + config.padding, size[1]];
        let track_hovered = contains(track_rect_position, track_rect_size, 0.0, mouse_pos);
        if track_hovered {
            extra.scroll.mark_activity();
        }

        let text_click = ui.mouse_pressed_this_frame() && hovered && !track_hovered;

        let text_origin_unscrolled = [
            position[0] + padding_x - extra.scroll_x,
            position[1] + padding_y - extra.scroll.offset,
        ];
        let relative_click = [
            mouse_pos[0] - text_origin_unscrolled[0],
            mouse_pos[1] - text_origin_unscrolled[1],
        ];

        if text_click {
            ui.request_focus(self.focus_id);
            state.mark_activity();
            let index = char_index_at_point(&state, ui, line_height, relative_click);

            match ui.click_count() {
                1 => {
                    extra.text_dragging = true;
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
                    let line_start = state.line_start(index);
                    let line_end = state.line_end(index);
                    state.set_selection_anchor(Some(line_start));
                    state.set_cursor(line_end);
                }
            }
        }

        if !ui.mouse_pressed() {
            extra.text_dragging = false;
        }

        if ui.mouse_pressed() && extra.text_dragging && ui.click_count() == 1 {
            state.mark_activity();
            let index = char_index_at_point(&state, ui, line_height, relative_click);
            state.set_cursor(index);
        }

        let cursor_moved = enter
            || up
            || down
            || ui.key_pressed(Key::Named(NamedKey::ArrowLeft))
            || ui.key_pressed(Key::Named(NamedKey::ArrowRight))
            || ui.key_pressed(Key::Named(NamedKey::Home))
            || ui.key_pressed(Key::Named(NamedKey::End))
            || !ui.typed_text().is_empty()
            || text_click
            || extra.text_dragging;

        if focused && cursor_moved && !extra.scroll.dragging {
            let cursor_line_start = state.line_start(state.cursor());
            let cursor_line_index = state.text()[..state.byte_index_for(cursor_line_start)]
                .matches('\n')
                .count();
            let cursor_line_y = cursor_line_index as f32 * line_height;

            if cursor_line_y - extra.scroll.offset + line_height > visible_height {
                extra.scroll.offset = cursor_line_y + line_height - visible_height;
            }
            if cursor_line_y - extra.scroll.offset < 0.0 {
                extra.scroll.offset = cursor_line_y;
            }
        }

        if hovered && ui.scroll_delta_y() != 0.0 {
            extra.scroll.offset -= ui.scroll_delta_y();
            extra.scroll.mark_activity();
        }

        let geometry = compute_geometry(
            visible_height,
            content_height,
            track_length,
            extra.scroll.offset,
            &config,
        );

        let thumb_position = [
            track_x,
            position[1] + config.padding + geometry.thumb_position_along_track,
        ];
        let thumb_size = [config.thickness, geometry.thumb_size];
        let thumb_hovered = contains(thumb_position, thumb_size, 0.0, mouse_pos);

        if let Some(new_offset) = handle_drag(
            &mut extra.scroll,
            position[1] + config.padding,
            mouse_pos[1],
            thumb_position[1],
            ui.mouse_pressed_this_frame(),
            ui.mouse_pressed(),
            thumb_hovered,
            &geometry,
        ) {
            extra.scroll.offset = new_offset;
            extra.scroll.mark_activity();
        }

        extra.scroll.offset = extra.scroll.offset.clamp(0.0, geometry.max_scroll);

        let geometry_final = compute_geometry(
            visible_height,
            content_height,
            track_length,
            extra.scroll.offset,
            &config,
        );
        let thumb_position = [
            track_x,
            position[1] + config.padding + geometry_final.thumb_position_along_track,
        ];

        let show_scrollbar = geometry.max_scroll > 0.0
            && (track_hovered || extra.scroll.dragging || extra.scroll.recently_active(&config));

        if show_scrollbar {
            ui.push_input_block([
                track_rect_position[0],
                track_rect_position[1],
                track_rect_position[0] + track_rect_size[0],
                track_rect_position[1] + track_rect_size[1],
            ]);
        }

        let line_start = state.line_start(state.cursor());
        let line_start_byte = state.byte_index_for(line_start);
        let cursor_byte = state.byte_index_for(state.cursor());
        let prefix = &state.text()[line_start_byte..cursor_byte];
        let cursor_x = ui.measure_text(prefix);

        let visible_width = size[0] - padding_x * 2.0;
        if focused && cursor_moved {
            if cursor_x - extra.scroll_x > visible_width {
                extra.scroll_x = cursor_x - visible_width;
            }
            if cursor_x - extra.scroll_x < 0.0 {
                extra.scroll_x = cursor_x;
            }
        }
        extra.scroll_x = extra.scroll_x.max(0.0);

        let dt = ui.dt();
        let focus_target = if focused { 1.0f32 } else { 0.0 };
        let hover_target = if hovered { 1.0f32 } else { 0.0 };
        state.focus_t = animate_towards(state.focus_t, focus_target, dt, Motion::GENTLE);
        state.hover_t = animate_towards(state.hover_t, hover_target, dt, Motion::SNAPPY);
        let focus_t = state.focus_t;
        let hover_t = state.hover_t;

        let base_or_hover = style.border_color.lerp(theme.border_strong, hover_t);
        let border_color = base_or_hover.lerp(style.focus_border_color, focus_t);
        let border_width = style.border_width + focus_t * 0.5;

        if let Some(shadow) = &style.shadow {
            let mut s = *shadow;
            if focus_t > 0.01 {
                s.color = s
                    .color
                    .lerp(style.focus_border_color.with_alpha(0.25), focus_t);
                s.blur_radius += focus_t * 6.0;
            }
            draw_shadow(&s, position, size, style.corner_radius, ui);
        }

        ui.draw_rect(
            position,
            size,
            style.fill,
            style.corner_radius,
            border_width,
            border_color,
            0.0,
            style.sharp,
            0.0,
        );

        let clip_rect = [
            position[0] + padding_x,
            position[1] + padding_y,
            position[0] + size[0] - padding_x,
            position[1] + size[1] - padding_y,
        ];
        ui.push_clip(clip_rect);

        let text_origin = [
            position[0] + padding_x - extra.scroll_x,
            position[1] + padding_y - extra.scroll.offset,
        ];

        if let Some((sel_start, sel_end)) = state.selection_range() {
            let mut char_offset = 0usize;
            for (i, line) in state.text().split('\n').enumerate() {
                let line_char_len = line.chars().count();
                let this_line_start = char_offset;
                let this_line_end = char_offset + line_char_len;

                let overlap_start = sel_start.max(this_line_start);
                let overlap_end = sel_end.min(this_line_end);

                if overlap_start < overlap_end {
                    let prefix_start_byte =
                        state.byte_index_for(overlap_start) - state.byte_index_for(this_line_start);
                    let prefix_end_byte =
                        state.byte_index_for(overlap_end) - state.byte_index_for(this_line_start);
                    let x_start = ui.measure_text(&line[..prefix_start_byte]);
                    let x_end = ui.measure_text(&line[..prefix_end_byte]);

                    let highlight_position = [
                        (text_origin[0] + x_start).round(),
                        text_origin[1] + i as f32 * line_height,
                    ];
                    let highlight_size = [(x_end - x_start).max(2.0), line_height];

                    ui.draw_rect(
                        highlight_position,
                        highlight_size,
                        Fill::Solid(style.selection_color),
                        0.0,
                        0.0,
                        Color::TRANSPARENT,
                        0.0,
                        false,
                        0.0,
                    );
                }

                char_offset = this_line_end + 1;
            }
        }

        for (i, line) in state.text().split('\n').enumerate() {
            if !line.is_empty() {
                let line_position = [text_origin[0], text_origin[1] + i as f32 * line_height];
                ui.draw_text_colored(line, line_position, clip_rect, style.text_color);
            }
        }

        if focused {
            let idle_seconds = state.last_activity().elapsed().as_secs_f32();
            let cursor_visible = idle_seconds < 0.3 || (idle_seconds % 1.0) < 0.5;
            if cursor_visible {
                let line_index = state.text()[..line_start_byte].matches('\n').count();
                let cursor_position = [
                    (text_origin[0] + cursor_x).round(),
                    text_origin[1] + line_index as f32 * line_height,
                ];
                ui.draw_rect(
                    cursor_position,
                    [2.0, line_height],
                    Fill::Solid(style.cursor_color),
                    0.0,
                    0.0,
                    Color::TRANSPARENT,
                    0.0,
                    true,
                    0.0,
                );
            }
        }
        ui.pop_clip();

        if show_scrollbar {
            ui.pop_input_block();
        }

        if show_scrollbar {
            let thumb_fill = if extra.scroll.dragging {
                style.thumb_dragging_fill
            } else {
                style.thumb_fill
            };
            ui.draw_rect(
                thumb_position,
                [config.thickness, geometry_final.thumb_size],
                thumb_fill,
                config.thickness / 2.0,
                0.0,
                Color::TRANSPARENT,
                0.0,
                false,
                0.0,
            );
        }

        ui.put_widget_state(&self.id, state);
        ui.put_widget_state(&self.extra_id, extra);
    }
}

impl StatefulWidget for TextArea {
    type State = TextEditState;

    fn state_id(&self) -> &str {
        &self.id
    }

    fn initial_state(&self) -> TextEditState {
        let mut state = TextEditState::default();
        state.set_text(&self.default_text);
        state
    }
}

impl Accessible for TextArea {
    fn accessibility_id(&self) -> NodeId {
        NodeId(hash_id(&self.id))
    }
    fn accessibility_role(&self) -> Role {
        Role::MultilineTextInput
    }
}
