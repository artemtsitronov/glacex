use crate::ui::Ui;
use std::time::Instant;
use winit::keyboard::{Key, KeyCode, NamedKey, PhysicalKey};

pub struct TextEditState {
    text: String,
    cursor: usize,
    scroll_offset: f32,
    selection_anchor: Option<usize>,
    last_activity: Instant,
    pub dragging: bool,
    pub hovered: bool,
}

impl Default for TextEditState {
    fn default() -> Self {
        TextEditState {
            text: String::new(),
            cursor: 0,
            scroll_offset: 0.0,
            selection_anchor: None,
            last_activity: Instant::now(),
            dragging: false,
            hovered: false,
        }
    }
}

impl TextEditState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.cursor = self.text.chars().count();
        self.selection_anchor = None;
        self.scroll_offset = 0.0;
        self.mark_activity();
    }

    pub fn clear(&mut self) {
        self.set_text("");
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn scroll_offset(&self) -> f32 {
        self.scroll_offset
    }

    pub fn last_activity(&self) -> Instant {
        self.last_activity
    }

    pub fn mark_activity(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn selection_range(&self) -> Option<(usize, usize)> {
        let anchor = self.selection_anchor?;
        if anchor == self.cursor {
            return None;
        }
        let mut start = anchor;
        let mut end = self.cursor;
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        Some((start, end))
    }

    pub fn set_selection_anchor(&mut self, anchor: Option<usize>) {
        self.selection_anchor = anchor;
    }

    pub fn set_scroll_offset(&mut self, offset: f32) {
        self.scroll_offset = offset;
    }

    pub fn byte_index(&self) -> usize {
        self.byte_index_for(self.cursor())
    }

    pub fn byte_index_for(&self, char_index: usize) -> usize {
        self.text
            .char_indices()
            .nth(char_index)
            .map(|(i, _)| i)
            .unwrap_or(self.text.len())
    }

    pub fn set_cursor(&mut self, cursor_index: usize) {
        self.cursor = cursor_index;
    }

    pub fn prefix(&self) -> &str {
        &self.text[..self.byte_index()]
    }

    fn delete_range(&mut self, start: usize, end: usize) {
        let start_byte = self.byte_index_for(start);
        let end_byte = self.byte_index_for(end);
        self.text.replace_range(start_byte..end_byte, "");
        self.cursor = start;
        self.selection_anchor = None;
    }

    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    pub fn word_start(&self, index: usize) -> usize {
        let byte_idx = self.byte_index_for(index);
        let mut char_idx = index;
        let mut it = self.text[..byte_idx].char_indices().rev().peekable();
        while let Some(&(_, c)) = it.peek() {
            if Self::is_word_char(c) {
                break;
            }
            it.next();
            char_idx -= 1;
        }
        while let Some(&(_, c)) = it.peek() {
            if !Self::is_word_char(c) {
                break;
            }
            it.next();
            char_idx -= 1;
        }
        char_idx
    }

    pub fn word_end(&self, index: usize) -> usize {
        let byte_idx = self.byte_index_for(index);
        let mut char_idx = index;
        let mut it = self.text[byte_idx..].chars().peekable();
        while let Some(&c) = it.peek() {
            if Self::is_word_char(c) {
                break;
            }
            it.next();
            char_idx += 1;
        }
        while let Some(&c) = it.peek() {
            if !Self::is_word_char(c) {
                break;
            }
            it.next();
            char_idx += 1;
        }
        char_idx
    }

    /// Char index of the start of the line `index` is on — scans backward
    /// to the nearest '\n' (or the start of the text).
    pub fn line_start(&self, index: usize) -> usize {
        let byte_idx = self.byte_index_for(index);
        let mut char_idx = index;
        let mut it = self.text[..byte_idx].char_indices().rev().peekable();
        while let Some(&(_, c)) = it.peek() {
            if c == '\n' {
                break;
            }
            it.next();
            char_idx -= 1;
        }
        char_idx
    }

    /// Char index of the end of the line `index` is on — scans forward to
    /// the nearest '\n' (or the end of the text). The returned index points
    /// AT the '\n' itself (or text.len()), not past it — i.e. it's where
    /// the cursor sits when you press End on this line.
    pub fn line_end(&self, index: usize) -> usize {
        let byte_idx = self.byte_index_for(index);
        let mut char_idx = index;
        let mut it = self.text[byte_idx..].chars().peekable();
        while let Some(&c) = it.peek() {
            if c == '\n' {
                break;
            }
            it.next();
            char_idx += 1;
        }
        char_idx
    }

    /// How many characters into its own line `index` is.
    pub fn column_of(&self, index: usize) -> usize {
        index - self.line_start(index)
    }

    /// Total number of lines (a string with no '\n' at all still counts as
    /// one line).
    pub fn line_count(&self) -> usize {
        self.text.matches('\n').count() + 1
    }

    /// Moves the cursor up one line, landing at `preferred_column` (or the
    /// cursor's current column if None) — clamped to the previous line's
    /// actual length. Returns the resulting cursor index; does not mutate
    /// self.cursor itself, since the caller needs the value before deciding
    /// whether to update preferred_column too.
    pub fn move_cursor_up(&self, preferred_column: Option<usize>) -> usize {
        let current_line_start = self.line_start(self.cursor);
        if current_line_start == 0 {
            return self.cursor; // already on the first line, nowhere to go
        }
        let prev_line_end = current_line_start - 1; // the '\n' just before this line
        let prev_line_start = self.line_start(prev_line_end);
        let prev_line_length = prev_line_end - prev_line_start;

        let column = preferred_column.unwrap_or_else(|| self.column_of(self.cursor));
        prev_line_start + column.min(prev_line_length)
    }

    /// Moves the cursor down one line, same column-preservation logic as
    /// move_cursor_up.
    pub fn move_cursor_down(&self, preferred_column: Option<usize>) -> usize {
        let current_line_end = self.line_end(self.cursor);
        let char_count = self.text.chars().count();
        if current_line_end >= char_count {
            return self.cursor; // already on the last line
        }
        let next_line_start = current_line_end + 1; // skip past the '\n'
        let next_line_end = self.line_end(next_line_start);
        let next_line_length = next_line_end - next_line_start;

        let column = preferred_column.unwrap_or_else(|| self.column_of(self.cursor));
        next_line_start + column.min(next_line_length)
    }

    /// Inserts a real newline at the cursor — deliberately bypasses the
    /// is_control() filter that keeps '\n' out of single-line TextInput;
    /// only TextArea should call this.
    pub fn insert_newline(&mut self) {
        let idx = self.byte_index();
        self.text.insert(idx, '\n');
        self.cursor += 1;
    }

    pub fn handle_input(&mut self, ui: &mut Ui) {
        let backspace = ui.key_pressed(Key::Named(NamedKey::Backspace));
        let delete = ui.key_pressed(Key::Named(NamedKey::Delete));
        let left = ui.key_pressed(Key::Named(NamedKey::ArrowLeft));
        let right = ui.key_pressed(Key::Named(NamedKey::ArrowRight));
        let home = ui.key_pressed(Key::Named(NamedKey::Home));
        let end = ui.key_pressed(Key::Named(NamedKey::End));
        let ctrl_a = ui.physical_key_pressed(PhysicalKey::Code(KeyCode::KeyA));
        let ctrl_c = ui.physical_key_pressed(PhysicalKey::Code(KeyCode::KeyC));
        let ctrl_v = ui.physical_key_pressed(PhysicalKey::Code(KeyCode::KeyV));

        let typed: String = ui
            .typed_text()
            .chars()
            .filter(|c| !c.is_control())
            .collect();

        let editing = (!typed.is_empty() && !ui.ctrl_held()) || left || right;

        let any_activity = !typed.is_empty()
            || backspace
            || delete
            || left
            || right
            || home
            || end
            || (ui.ctrl_held() && ctrl_a)
            || (ui.ctrl_held() && ctrl_v);

        if any_activity {
            self.mark_activity();
        }

        if editing && !ui.shift_held() {
            self.selection_anchor = None;
        }

        if !ui.ctrl_held() && !typed.is_empty() {
            if let Some((start, end)) = self.selection_range() {
                self.delete_range(start, end);
            }
            for ch in typed.chars() {
                let idx = self.byte_index();
                self.text.insert(idx, ch);
                self.cursor += 1;
            }
        }

        if backspace {
            if let Some((start, end)) = self.selection_range() {
                self.delete_range(start, end);
            } else if self.cursor > 0 {
                self.cursor -= 1;
                let idx = self.byte_index();
                self.text.remove(idx);
            }
        }

        if delete {
            if let Some((start, end)) = self.selection_range() {
                self.delete_range(start, end);
            } else {
                let char_count = self.text.chars().count();
                if self.cursor < char_count {
                    let idx = self.byte_index();
                    self.text.remove(idx);
                }
            }
        }

        if left {
            if ui.shift_held() && self.selection_anchor.is_none() {
                self.selection_anchor = Some(self.cursor);
            }
            if ui.ctrl_held() {
                self.cursor = self.word_start(self.cursor.saturating_sub(1));
            } else if self.cursor > 0 {
                self.cursor -= 1;
            }
        }

        if right {
            if ui.shift_held() && self.selection_anchor.is_none() {
                self.selection_anchor = Some(self.cursor);
            }
            if ui.ctrl_held() {
                self.cursor = self.word_end(self.cursor);
            } else {
                let char_count = self.text.chars().count();
                if self.cursor < char_count {
                    self.cursor += 1;
                }
            }
        }

        if home {
            if ui.shift_held() && self.selection_anchor.is_none() {
                self.selection_anchor = Some(self.cursor);
            } else if !ui.shift_held() {
                self.selection_anchor = None;
            }
            self.cursor = 0;
        }

        if end {
            if ui.shift_held() && self.selection_anchor.is_none() {
                self.selection_anchor = Some(self.cursor);
            } else if !ui.shift_held() {
                self.selection_anchor = None;
            }
            self.cursor = self.text.chars().count();
        }

        if ui.ctrl_held() && ctrl_a {
            self.selection_anchor = Some(0);
            self.cursor = self.text.chars().count();
        }

        if ui.ctrl_held() && ctrl_c {
            if let Some((start, end)) = self.selection_range() {
                let start_byte = self.byte_index_for(start);
                let end_byte = self.byte_index_for(end);
                let selected = &self.text[start_byte..end_byte];
                ui.copy_to_clipboard(selected);
            }
        }

        if ui.ctrl_held() && ctrl_v {
            if let Some(pasted) = ui.paste_from_clipboard() {
                let pasted: String = pasted.chars().filter(|c| !c.is_control()).collect();
                if let Some((start, end)) = self.selection_range() {
                    self.delete_range(start, end);
                }
                let idx = self.byte_index();
                self.text.insert_str(idx, &pasted);
                self.cursor += pasted.chars().count();
                self.selection_anchor = None;
            }
        }
    }

    pub fn cursor_index_for_x(&self, ui: &mut Ui, click_x: f32) -> usize {
        let mut best_index = 0;
        let mut best_distance = f32::MAX;
        for (char_index, (byte_index, _)) in self.text.char_indices().enumerate().chain(
            std::iter::once((self.text.chars().count(), (self.text.len(), ' '))),
        ) {
            let prefix_width = ui.measure_text(&self.text[..byte_index]);
            let distance = (prefix_width - click_x).abs();
            if distance < best_distance {
                best_distance = distance;
                best_index = char_index;
            }
        }
        best_index
    }

    pub fn scroll_into_view(&mut self, cursor_x: f32, visible_width: f32) {
        if cursor_x - self.scroll_offset > visible_width {
            self.scroll_offset = cursor_x - visible_width;
        }
        if cursor_x - self.scroll_offset < 0.0 {
            self.scroll_offset = cursor_x;
        }
        self.scroll_offset = self.scroll_offset.max(0.0);
    }
}
