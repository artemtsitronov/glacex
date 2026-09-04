use crate::color::Color;
use crate::fill::Fill;
use crate::painter::Painter;
use crate::theme::Theme;
use crate::widget::{FocusId, Widget};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use winit::keyboard::{Key, PhysicalKey};
use winit::window::{CursorIcon, Window};

fn intersect_rects(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        a[0].max(b[0]),
        a[1].max(b[1]),
        a[2].min(b[2]),
        a[3].min(b[3]),
    ]
}

pub struct Ui {
    window: Arc<Window>,
    painter: Painter,
    persistent_state: HashMap<String, Box<dyn Any>>,
    mouse_position: [f32; 2],
    mouse_pressed: bool,
    mouse_released: bool,
    mouse_pressed_this_frame: bool,
    mouse_released_this_frame: bool,
    start_time: Instant,
    last_frame_time: Instant,
    dt: f32,
    typed_text: String,
    keys_pressed_this_frame: HashSet<Key>,
    physical_keys_pressed_this_frame: HashSet<PhysicalKey>,
    shift_held: bool,
    ctrl_held: bool,
    last_click_time: Option<Instant>,
    last_click_position: [f32; 2],
    clipboard: Option<arboard::Clipboard>,
    focused: Option<FocusId>,
    focus_requested_this_frame: bool,
    focus_order: Vec<FocusId>,
    clip_stack: Vec<[f32; 4]>,
    scroll_delta_x: f32,
    scroll_delta_y: f32,
    input_block_stack: Vec<[f32; 4]>,
    selected: HashMap<String, String>,
    click_count: u32,
    cursor_icon: CursorIcon,
    cursor_icon_set_this_frame: bool,
    mouse_right_pressed: bool,
    mouse_right_released: bool,
    mouse_right_pressed_this_frame: bool,
    mouse_right_released_this_frame: bool,
    mouse_middle_pressed: bool,
    mouse_middle_released: bool,
    mouse_middle_pressed_this_frame: bool,
    mouse_middle_released_this_frame: bool,
    pending_tooltip: Option<(String, [f32; 2])>,
}

impl Ui {
    pub async fn new(window: Arc<Window>) -> Ui {
        Ui {
            painter: Painter::new(window.clone()).await,
            window,
            persistent_state: HashMap::new(),
            mouse_position: [0.0, 0.0],
            mouse_pressed: false,
            mouse_released: false,
            mouse_pressed_this_frame: false,
            mouse_released_this_frame: false,
            start_time: Instant::now(),
            last_frame_time: Instant::now(),
            dt: 1.0 / 60.0,
            typed_text: String::new(),
            keys_pressed_this_frame: HashSet::new(),
            physical_keys_pressed_this_frame: HashSet::new(),
            shift_held: false,
            ctrl_held: false,
            last_click_time: None,
            last_click_position: [0.0, 0.0],
            clipboard: match arboard::Clipboard::new() {
                Ok(c) => Some(c),
                Err(error) => {
                    eprintln!("failed to initialize clipboard: {error}");
                    None
                }
            },
            focused: None,
            focus_requested_this_frame: false,
            focus_order: Vec::new(),
            clip_stack: Vec::new(),
            scroll_delta_x: 0.0,
            scroll_delta_y: 0.0,
            input_block_stack: Vec::new(),
            selected: HashMap::new(),
            click_count: 0,
            cursor_icon: CursorIcon::Default,
            cursor_icon_set_this_frame: false,
            mouse_right_pressed: false,
            mouse_right_released: false,
            mouse_right_pressed_this_frame: false,
            mouse_right_released_this_frame: false,
            mouse_middle_pressed: false,
            mouse_middle_released: false,
            mouse_middle_pressed_this_frame: false,
            mouse_middle_released_this_frame: false,
            pending_tooltip: None,
        }
    }

    pub fn select(&mut self, group_id: &str, option_id: &str) {
        self.selected
            .insert(group_id.to_string(), option_id.to_string());
    }

    pub fn is_selected(&self, group_id: &str, option_id: &str) -> bool {
        self.selected.get(group_id).map(|s| s.as_str()) == Some(option_id)
    }

    pub fn selected_option(&self, group_id: &str) -> Option<&str> {
        self.selected.get(group_id).map(|s| s.as_str())
    }

    pub fn push_input_block(&mut self, rect: [f32; 4]) {
        self.input_block_stack.push(rect);
    }

    pub fn pop_input_block(&mut self) {
        self.input_block_stack.pop();
    }

    pub fn is_input_blocked(&self, point: [f32; 2]) -> bool {
        self.input_block_stack.iter().any(|rect| {
            point[0] >= rect[0] && point[0] <= rect[2] && point[1] >= rect[1] && point[1] <= rect[3]
        })
    }

    pub fn point_in_current_clip(&self, point: [f32; 2]) -> bool {
        let clip = self.current_clip();
        point[0] >= clip[0] && point[0] <= clip[2] && point[1] >= clip[1] && point[1] <= clip[3]
    }

    pub fn widget_state<T: Default + 'static>(&mut self, id: &str) -> &mut T {
        self.persistent_state
            .entry(id.to_string())
            .or_insert_with(|| Box::new(T::default()))
            .downcast_mut::<T>()
            .expect("widget id reused with a different state type")
    }

    pub fn widget_state_or<T: 'static>(&mut self, id: &str, initial: T) -> &mut T {
        self.persistent_state
            .entry(id.to_string())
            .or_insert_with(|| Box::new(initial))
            .downcast_mut::<T>()
            .unwrap()
    }

    pub fn take_widget_state<T: Default + 'static>(&mut self, id: &str) -> T {
        self.persistent_state
            .remove(id)
            .and_then(|b| b.downcast::<T>().ok())
            .map(|b| *b)
            .unwrap_or_default()
    }

    pub fn take_widget_state_or<T: 'static>(&mut self, id: &str, initial: T) -> T {
        self.persistent_state
            .remove(id)
            .and_then(|b| b.downcast::<T>().ok())
            .map(|b| *b)
            .unwrap_or(initial)
    }

    pub fn put_widget_state<T: 'static>(&mut self, id: &str, state: T) {
        self.persistent_state
            .insert(id.to_string(), Box::new(state));
    }

    pub fn get_state<T: Default + Clone + 'static>(&mut self, id: &str) -> T {
        self.widget_state_or::<T>(id, T::default()).clone()
    }

    pub fn scroll_delta_x(&self) -> f32 {
        self.scroll_delta_x
    }
    pub fn scroll_delta_y(&self) -> f32 {
        self.scroll_delta_y
    }

    pub fn set_scroll_delta_x(&mut self, delta: f32) {
        self.scroll_delta_x = delta;
    }
    pub fn set_scroll_delta_y(&mut self, delta: f32) {
        self.scroll_delta_y = delta;
    }

    pub fn push_clip(&mut self, rect: [f32; 4]) {
        let current = self.current_clip();
        self.clip_stack.push(intersect_rects(current, rect));
    }

    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    fn current_clip(&self) -> [f32; 4] {
        self.clip_stack.last().copied().unwrap_or_else(|| {
            let window = self.painter.window_size();
            [0.0, 0.0, window[0], window[1]]
        })
    }

    pub fn request_focus(&mut self, id: FocusId) {
        self.focused = Some(id);
        self.focus_requested_this_frame = true;
    }

    pub fn is_focused(&self, id: FocusId) -> bool {
        self.focused == Some(id)
    }

    pub fn clear_focus(&mut self) {
        self.focused = None;
    }

    pub fn focused_id(&self) -> Option<FocusId> {
        self.focused
    }

    pub fn focus_requested_this_frame(&self) -> bool {
        self.focus_requested_this_frame
    }

    pub fn register_focusable(&mut self, id: FocusId) {
        self.focus_order.push(id);
    }

    pub fn advance_focus(&mut self, backward: bool) {
        if self.focus_order.is_empty() {
            return;
        }

        let current_index = self
            .focused
            .and_then(|id| self.focus_order.iter().position(|&fid| fid == id));

        let next_index = match current_index {
            Some(i) => {
                if backward {
                    (i + self.focus_order.len() - 1) % self.focus_order.len()
                } else {
                    (i + 1) % self.focus_order.len()
                }
            }
            None => 0,
        };

        self.focused = Some(self.focus_order[next_index]);
        self.focus_requested_this_frame = true; // Tab counts as a focus request too,
        // so the empty-click-clears-focus
        // check doesn't fight with it
    }

    pub fn copy_to_clipboard(&mut self, text: &str) {
        if let Some(clipboard) = self.clipboard.as_mut() {
            let _ = clipboard.set_text(text);
        } else {
            eprintln!("copy requested but clipboard is unavailable");
        }
    }

    pub fn paste_from_clipboard(&mut self) -> Option<String> {
        if self.clipboard.is_none() {
            eprintln!("paste requested but clipboard is unavailable");
        }
        match self.clipboard.as_mut()?.get_text() {
            Ok(text) => Some(text),
            Err(error) => {
                eprintln!("failed to paste from clipboard: {error}");
                None
            }
        }
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn begin_frame(&mut self) {
        let now = Instant::now();
        let frame_elapsed = now.duration_since(self.last_frame_time).as_secs_f32();
        self.dt = frame_elapsed.clamp(0.001, 0.1);
        self.last_frame_time = now;

        self.painter.begin_frame();
        self.focus_order.clear();
        self.clip_stack.clear();
        self.input_block_stack.clear();
    }

    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        self.mouse_position = [x as f32, y as f32];
    }

    pub fn mouse_pressed(&self) -> bool {
        self.mouse_pressed
    }

    pub fn mouse_released(&self) -> bool {
        self.mouse_released
    }

    pub fn set_mouse_pressed(&mut self, pressed: bool) {
        self.mouse_pressed_this_frame = pressed && !self.mouse_pressed;
        self.mouse_pressed = pressed;

        if self.mouse_pressed_this_frame {
            let now = Instant::now();
            let position = self.mouse_position;

            const DOUBLE_CLICK_TIME: f32 = 0.4; // seconds
            const DOUBLE_CLICK_DISTANCE: f32 = 5.0; // pixels

            let is_repeat_click = match self.last_click_time {
                Some(last_time) => {
                    let elapsed = now.duration_since(last_time).as_secs_f32();
                    let dx = position[0] - self.last_click_position[0];
                    let dy = position[1] - self.last_click_position[1];
                    let distance = (dx * dx + dy * dy).sqrt();
                    elapsed < DOUBLE_CLICK_TIME && distance < DOUBLE_CLICK_DISTANCE
                }
                None => false,
            };

            self.click_count = if is_repeat_click {
                self.click_count + 1
            } else {
                1
            };
            self.last_click_time = Some(now);
            self.last_click_position = position;
        }
    }

    pub fn set_mouse_released(&mut self, released: bool) {
        self.mouse_released_this_frame = released && !self.mouse_released;
        self.mouse_released = released;
    }

    pub fn mouse_right_pressed(&self) -> bool {
        self.mouse_right_pressed
    }

    pub fn mouse_right_released(&self) -> bool {
        self.mouse_right_released
    }

    pub fn mouse_right_pressed_this_frame(&self) -> bool {
        self.mouse_right_pressed_this_frame
    }

    pub fn mouse_right_released_this_frame(&self) -> bool {
        self.mouse_right_released_this_frame
    }

    pub fn set_mouse_right_pressed(&mut self, pressed: bool) {
        self.mouse_right_pressed_this_frame = pressed && !self.mouse_right_pressed;
        self.mouse_right_pressed = pressed;
    }

    pub fn set_mouse_right_released(&mut self, released: bool) {
        self.mouse_right_released_this_frame = released && !self.mouse_right_released;
        self.mouse_right_released = released;
    }

    pub fn mouse_middle_pressed(&self) -> bool {
        self.mouse_middle_pressed
    }

    pub fn mouse_middle_released(&self) -> bool {
        self.mouse_middle_released
    }

    pub fn mouse_middle_pressed_this_frame(&self) -> bool {
        self.mouse_middle_pressed_this_frame
    }

    pub fn mouse_middle_released_this_frame(&self) -> bool {
        self.mouse_middle_released_this_frame
    }

    pub fn set_mouse_middle_pressed(&mut self, pressed: bool) {
        self.mouse_middle_pressed_this_frame = pressed && !self.mouse_middle_pressed;
        self.mouse_middle_pressed = pressed;
    }

    pub fn set_mouse_middle_released(&mut self, released: bool) {
        self.mouse_middle_released_this_frame = released && !self.mouse_middle_released;
        self.mouse_middle_released = released;
    }

    pub fn set_cursor_icon(&mut self, icon: CursorIcon) {
        self.cursor_icon = icon;
        self.cursor_icon_set_this_frame = true;
        self.window.set_cursor(icon);
    }

    pub fn cursor_icon(&self) -> CursorIcon {
        self.cursor_icon
    }

    pub fn show_tooltip(&mut self, text: impl Into<String>) {
        self.pending_tooltip = Some((text.into(), self.mouse_position));
    }

    pub fn show_tooltip_at(&mut self, text: impl Into<String>, position: [f32; 2]) {
        self.pending_tooltip = Some((text.into(), position));
    }

    pub fn mouse_position(&self) -> [f32; 2] {
        self.mouse_position
    }

    pub fn mouse_pressed_this_frame(&self) -> bool {
        self.mouse_pressed_this_frame
    }

    pub fn mouse_released_this_frame(&self) -> bool {
        self.mouse_released_this_frame
    }

    pub fn push_typed_text(&mut self, s: &str) {
        self.typed_text.push_str(s);
    }

    pub fn typed_text(&self) -> &str {
        &self.typed_text
    }

    pub fn click_count(&self) -> u32 {
        self.click_count
    }

    pub fn physical_key_pressed(&self, key: PhysicalKey) -> bool {
        self.physical_keys_pressed_this_frame.contains(&key)
    }

    pub fn key_pressed(&self, key: Key) -> bool {
        self.keys_pressed_this_frame.contains(&key)
    }

    pub fn mark_physical_key_pressed(&mut self, key: PhysicalKey) {
        self.physical_keys_pressed_this_frame.insert(key);
    }

    pub fn mark_key_pressed(&mut self, key: Key) {
        self.keys_pressed_this_frame.insert(key);
    }

    pub fn ctrl_held(&self) -> bool {
        self.ctrl_held
    }
    pub fn shift_held(&self) -> bool {
        self.shift_held
    }

    pub fn set_ctrl_held(&mut self, held: bool) {
        self.ctrl_held = held;
    }
    pub fn set_shift_held(&mut self, held: bool) {
        self.shift_held = held;
    }

    pub fn end_frame(&mut self) {
        self.mouse_pressed_this_frame = false;
        self.mouse_released_this_frame = false;
        self.mouse_right_pressed_this_frame = false;
        self.mouse_right_released_this_frame = false;
        self.mouse_middle_pressed_this_frame = false;
        self.mouse_middle_released_this_frame = false;
        self.typed_text.clear();
        self.keys_pressed_this_frame.clear();
        self.physical_keys_pressed_this_frame.clear();
        self.focus_requested_this_frame = false;
        self.scroll_delta_x = 0.0;
        self.scroll_delta_y = 0.0;

        if !self.cursor_icon_set_this_frame && self.cursor_icon != CursorIcon::Default {
            self.cursor_icon = CursorIcon::Default;
            self.window.set_cursor(CursorIcon::Default);
        }
        self.cursor_icon_set_this_frame = false;
        self.pending_tooltip = None;
    }

    pub fn line_height(&self) -> f32 {
        self.painter.line_height()
    }

    pub fn measure_text(&mut self, text: &str) -> f32 {
        self.painter.measure_text(text)
    }

    pub fn set_bgcolor(&mut self, color: Color) {
        self.painter.set_bgcolor(color);
    }

    pub fn window_size(&self) -> [f32; 2] {
        self.painter.window_size()
    }

    /// Sets the native window title.
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_rect(
        &mut self,
        position: [f32; 2],
        size: [f32; 2],
        fill: Fill,
        corner_radius: f32,
        border_width: f32,
        border_color: Color,
        blur_radius: f32,
        sharp: bool,
    ) {
        let clip = self.current_clip();
        self.painter.draw_rect(
            position,
            size,
            fill,
            corner_radius,
            border_width,
            border_color,
            blur_radius,
            if sharp { 1.0 } else { 0.0 },
            clip,
        );
    }

    pub fn draw_text(&mut self, text: &str, position: [f32; 2], bounds: [f32; 4]) {
        let clipped = intersect_rects(bounds, self.current_clip());
        self.painter.draw_text(text, position, clipped);
    }

    pub fn add<W: Widget>(&mut self, widget: &mut W) -> W::Output {
        widget.ui(self)
    }

    pub fn render(&mut self) {
        if let Some((text, pos)) = self.pending_tooltip.take() {
            let text_width = self.measure_text(&text);
            let pad_x = 8.0;
            let pad_y = 5.0;
            let width = text_width + pad_x * 2.0;
            let height = self.line_height() + pad_y * 2.0;

            let window_size = self.painter.window_size();
            let mut tooltip_pos = [pos[0] + 12.0, pos[1] + 18.0];
            if tooltip_pos[0] + width > window_size[0] {
                tooltip_pos[0] = (window_size[0] - width - 4.0).max(0.0);
            }
            if tooltip_pos[1] + height > window_size[1] {
                tooltip_pos[1] = (pos[1] - height - 6.0).max(0.0);
            }

            let full_clip = [0.0, 0.0, window_size[0], window_size[1]];
            self.painter.draw_rect(
                tooltip_pos,
                [width, height],
                Fill::Solid(Theme::SURFACE_ELEVATED),
                6.0,
                1.0,
                Theme::BORDER_STRONG,
                8.0,
                0.0,
                full_clip,
            );

            self.painter.draw_text(
                &text,
                [tooltip_pos[0] + pad_x, tooltip_pos[1] + pad_y],
                [
                    tooltip_pos[0],
                    tooltip_pos[1],
                    tooltip_pos[0] + width,
                    tooltip_pos[1] + height,
                ],
            );
        }

        self.painter.present();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.painter.resize(width, height);
    }
}
