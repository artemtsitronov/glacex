//! Live style playground.
//!
//! Type a command into the text box in the form:
//!
//!     widget_type|attribute:value,attribute:value,...
//!
//! e.g.
//!
//!     checkbox|corner_radius:12.0,color:#f5656f,
//!
//! and the matching preview widget below updates immediately, every frame,
//! straight from the typed text. This exercises `ButtonStyle`,
//! `CheckboxStyle`, `TextInputStyle`, `TextAreaStyle`, `ScrollViewStyle`, and
//! `ShadowStyle`, plus `Ui::set_title` (the window title reflects whichever
//! widget you're currently editing).
//!
//! Recognized attributes (all optional, any subset may be given):
//!   - `color`                 fill color, hex e.g. `#3a3a46` (all widgets)
//!   - `hover_color`           fill while hovered (button, checkbox)
//!   - `pressed_color`         fill while pressed (button)
//!   - `checked_color`         fill while checked (checkbox)
//!   - `border_color`          border color (button, checkbox, textinput, textarea)
//!   - `border_width`          border width in px
//!   - `corner_radius`         corner radius in px
//!   - `focus_border_color`    border color while focused (textinput, textarea)
//!   - `selection_color`       text selection highlight (textinput, textarea)
//!   - `cursor_color`          blinking caret color (textinput, textarea)
//!   - `thumb_color`           scrollbar thumb color (scrollview, textarea)
//!   - `thumb_dragging_color`  scrollbar thumb color while dragging
//!   - `thumb_corner_radius`   scrollbar thumb corner radius (scrollview)
//!   - `sharp`                 `true`/`false`, hard vs anti-aliased edge
//!   - `shadow`                `true`/`false`, enable/disable the drop shadow
//!   - `shadow_color`          drop shadow color (implies `shadow:true`)
//!   - `shadow_blur`           drop shadow blur radius
//!
//! Widget types accepted: `button`, `checkbox`, `textinput` (or `input`),
//! `textarea` (or `area`), `scrollview` (or `scroll`).

use glacex::{
    Alignment, App, Button, ButtonStyle, Checkbox, CheckboxStyle, Color, Fill, Label, ScrollView,
    ScrollViewStyle, ShadowStyle, TextArea, TextAreaStyle, TextEditState, TextInput,
    TextInputStyle, Ui, Widget, column, row,
};
use std::collections::HashMap;

/// Parses `widget_type|key:value,key:value,...`. A trailing comma, and
/// extra whitespace around any piece, are both fine.
fn parse_command(input: &str) -> Option<(String, HashMap<String, String>)> {
    let (widget, rest) = input.split_once('|')?;
    let widget = widget.trim().to_ascii_lowercase();
    if widget.is_empty() {
        return None;
    }

    let mut attrs = HashMap::new();
    for pair in rest.split(',') {
        let pair = pair.trim();
        if pair.is_empty() {
            continue;
        }
        if let Some((key, value)) = pair.split_once(':') {
            attrs.insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }

    Some((widget, attrs))
}

fn attr_f32(attrs: &HashMap<String, String>, key: &str) -> Option<f32> {
    attrs.get(key)?.trim().parse::<f32>().ok()
}

fn attr_color(attrs: &HashMap<String, String>, key: &str) -> Option<Color> {
    attrs.get(key).map(|value| Color::hex_str(value.trim()))
}

fn attr_bool(attrs: &HashMap<String, String>, key: &str) -> Option<bool> {
    match attrs.get(key)?.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}

/// Shared by every style that carries an optional drop shadow: `shadow`
/// toggles it on/off, `shadow_color`/`shadow_blur` tweak an enabled one
/// (and implicitly turn it on if it was off).
fn apply_shadow_attrs(attrs: &HashMap<String, String>, shadow: &mut Option<ShadowStyle>) {
    if let Some(enabled) = attr_bool(attrs, "shadow") {
        if enabled {
            shadow.get_or_insert_with(ShadowStyle::default);
        } else {
            *shadow = None;
        }
    }
    if let Some(color) = attr_color(attrs, "shadow_color") {
        shadow.get_or_insert_with(ShadowStyle::default).color = color;
    }
    if let Some(blur) = attr_f32(attrs, "shadow_blur") {
        shadow.get_or_insert_with(ShadowStyle::default).blur_radius = blur;
    }
}

fn button_style(attrs: &HashMap<String, String>) -> ButtonStyle {
    let mut style = ButtonStyle::default();
    if let Some(c) = attr_color(attrs, "color") {
        style.fill = Fill::Solid(c);
    }
    if let Some(c) = attr_color(attrs, "hover_color") {
        style.hover_fill = Fill::Solid(c);
    }
    if let Some(c) = attr_color(attrs, "pressed_color") {
        style.pressed_fill = Fill::Solid(c);
    }
    if let Some(v) = attr_f32(attrs, "corner_radius") {
        style.corner_radius = v;
    }
    if let Some(v) = attr_f32(attrs, "border_width") {
        style.border_width = v;
    }
    if let Some(c) = attr_color(attrs, "border_color") {
        style.border_color = c;
    }
    if let Some(v) = attr_bool(attrs, "sharp") {
        style.sharp = v;
    }
    apply_shadow_attrs(attrs, &mut style.shadow);
    style
}

fn checkbox_style(attrs: &HashMap<String, String>) -> CheckboxStyle {
    let mut style = CheckboxStyle::default();
    if let Some(c) = attr_color(attrs, "color") {
        style.fill = Fill::Solid(c);
    }
    if let Some(c) = attr_color(attrs, "hover_color") {
        style.hover_fill = Fill::Solid(c);
    }
    if let Some(c) = attr_color(attrs, "checked_color") {
        style.checked_fill = Fill::Solid(c);
    }
    if let Some(v) = attr_f32(attrs, "corner_radius") {
        style.corner_radius = v;
    }
    if let Some(v) = attr_f32(attrs, "border_width") {
        style.border_width = v;
    }
    if let Some(c) = attr_color(attrs, "border_color") {
        style.border_color = c;
    }
    if let Some(v) = attr_bool(attrs, "sharp") {
        style.sharp = v;
    }
    apply_shadow_attrs(attrs, &mut style.shadow);
    style
}

fn text_input_style(attrs: &HashMap<String, String>) -> TextInputStyle {
    let mut style = TextInputStyle::default();
    if let Some(c) = attr_color(attrs, "color") {
        style.fill = Fill::Solid(c);
    }
    if let Some(v) = attr_f32(attrs, "corner_radius") {
        style.corner_radius = v;
    }
    if let Some(v) = attr_f32(attrs, "border_width") {
        style.border_width = v;
    }
    if let Some(c) = attr_color(attrs, "border_color") {
        style.border_color = c;
    }
    if let Some(c) = attr_color(attrs, "focus_border_color") {
        style.focus_border_color = c;
    }
    if let Some(c) = attr_color(attrs, "selection_color") {
        style.selection_color = c;
    }
    if let Some(c) = attr_color(attrs, "cursor_color") {
        style.cursor_color = c;
    }
    if let Some(v) = attr_bool(attrs, "sharp") {
        style.sharp = v;
    }
    apply_shadow_attrs(attrs, &mut style.shadow);
    style
}

fn text_area_style(attrs: &HashMap<String, String>) -> TextAreaStyle {
    let mut style = TextAreaStyle::default();
    if let Some(c) = attr_color(attrs, "color") {
        style.fill = Fill::Solid(c);
    }
    if let Some(v) = attr_f32(attrs, "corner_radius") {
        style.corner_radius = v;
    }
    if let Some(v) = attr_f32(attrs, "border_width") {
        style.border_width = v;
    }
    if let Some(c) = attr_color(attrs, "border_color") {
        style.border_color = c;
    }
    if let Some(c) = attr_color(attrs, "focus_border_color") {
        style.focus_border_color = c;
    }
    if let Some(c) = attr_color(attrs, "selection_color") {
        style.selection_color = c;
    }
    if let Some(c) = attr_color(attrs, "cursor_color") {
        style.cursor_color = c;
    }
    if let Some(c) = attr_color(attrs, "thumb_color") {
        style.thumb_fill = Fill::Solid(c);
    }
    if let Some(c) = attr_color(attrs, "thumb_dragging_color") {
        style.thumb_dragging_fill = Fill::Solid(c);
    }
    if let Some(v) = attr_bool(attrs, "sharp") {
        style.sharp = v;
    }
    apply_shadow_attrs(attrs, &mut style.shadow);
    style
}

fn scroll_view_style(attrs: &HashMap<String, String>) -> ScrollViewStyle {
    let mut style = ScrollViewStyle::default();
    if let Some(c) = attr_color(attrs, "thumb_color") {
        style.thumb_fill = Fill::Solid(c);
    }
    if let Some(c) = attr_color(attrs, "thumb_dragging_color") {
        style.thumb_dragging_fill = Fill::Solid(c);
    }
    if let Some(v) = attr_f32(attrs, "thumb_corner_radius") {
        style.thumb_corner_radius = v;
    }
    style
}

struct AppState {
    last_title: String,
}

impl AppState {
    fn new() -> Self {
        AppState {
            last_title: String::new(),
        }
    }
}

impl Widget for AppState {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Color::rgb(18, 18, 22));

        let command = ui
            .widget_state::<TextEditState>("command_input")
            .text()
            .to_string();
        let parsed = parse_command(&command);

        // `Ui::set_title` demo: reflect whichever widget is currently being
        // styled in the window title. Only call it when it actually changes,
        // to avoid re-issuing a native title update every single frame.
        let title = match &parsed {
            Some((widget, _)) => format!("Style Playground — editing: {widget}"),
            None => "Style Playground — type a command below".to_string(),
        };
        if title != self.last_title {
            ui.set_title(&title);
            self.last_title = title;
        }

        let mut button_style_value = ButtonStyle::default();
        let mut checkbox_style_value = CheckboxStyle::default();
        let mut input_style_value = TextInputStyle::default();
        let mut area_style_value = TextAreaStyle::default();
        let mut scroll_style_value = ScrollViewStyle::default();

        if let Some((widget, attrs)) = &parsed {
            match widget.as_str() {
                "button" => button_style_value = button_style(attrs),
                "checkbox" => checkbox_style_value = checkbox_style(attrs),
                "textinput" | "input" => input_style_value = text_input_style(attrs),
                "textarea" | "area" => area_style_value = text_area_style(attrs),
                "scrollview" | "scroll" => scroll_style_value = scroll_view_style(attrs),
                _ => {}
            }
        }

        let mut title_label = Label::new("Style Playground");
        let mut hint_label = Label::new(
            "Type: widget|key:value,key:value   e.g. checkbox|corner_radius:12.0,color:#f5656f,",
        );
        let mut command_caption = Label::new("Command");
        let mut preview_caption = Label::new("Preview (updates live as you type)");
        let mut command_input = TextInput::new("command_input", 460.0);

        let mut button_caption = Label::new("button");
        let mut checkbox_caption = Label::new("checkbox");
        let mut input_caption = Label::new("textinput");
        let mut area_caption = Label::new("textarea");
        let mut scroll_caption = Label::new("scrollview");

        let mut demo_button = Button::new("Button").style(button_style_value);
        let mut demo_checkbox = Checkbox::new("demo_checkbox").style(checkbox_style_value);
        let mut demo_text_input = TextInput::new("demo_text_input", 180.0).style(input_style_value);
        let mut demo_text_area =
            TextArea::new("demo_text_area", 180.0, 70.0).style(area_style_value);

        let mut scroll_item_1 = Label::new("Line one");
        let mut scroll_item_2 = Label::new("Line two");
        let mut scroll_item_3 = Label::new("Line three");
        let mut scroll_item_4 = Label::new("Line four");
        let mut scroll_item_5 = Label::new("Line five");

        column![
            &mut title_label,
            &mut hint_label,
            &mut command_caption,
            &mut command_input,
            &mut preview_caption,
            &mut row![
                &mut column![&mut button_caption, &mut demo_button].align(Alignment::Start),
                &mut column![&mut checkbox_caption, &mut demo_checkbox].align(Alignment::Start),
                &mut column![&mut input_caption, &mut demo_text_input].align(Alignment::Start),
            ]
            .align(Alignment::Start),
            &mut row![
                &mut column![&mut area_caption, &mut demo_text_area].align(Alignment::Start),
                &mut column![
                    &mut scroll_caption,
                    &mut ScrollView::new(
                        "demo_scroll",
                        [180.0, 90.0],
                        &mut column![
                            &mut scroll_item_1,
                            &mut scroll_item_2
