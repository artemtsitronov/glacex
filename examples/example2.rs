//! Live style playground.
//!
//! Type a command into the text box in the form:
//!
//!     widget_type|attribute:value,attribute:value,...
//!
//! e.g.
//!
//!     checkbox|corner_radius:12.0,color:#f5656f
//!
//! and the matching preview widget below updates immediately, every frame,
//! straight from the typed text.

use glacex::{
    Alignment, App, Badge, BadgeVariant, Button, ButtonStyle, Card, CardStyle, Checkbox,
    CheckboxStyle, Color, Divider, Fill, Label, ScrollView, ScrollViewStyle, ShadowStyle, TextArea,
    TextAreaStyle, TextEditState, TextInput, TextInputStyle, Theme, Ui, Widget, column, row,
};
use std::collections::HashMap;

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
        if let Some((k, v)) = pair.split_once(':') {
            attrs.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }
    Some((widget, attrs))
}

fn attr_color(attrs: &HashMap<String, String>, key: &str) -> Option<Color> {
    attrs.get(key).map(|v| Color::hex_str(v))
}

fn attr_f32(attrs: &HashMap<String, String>, key: &str) -> Option<f32> {
    attrs.get(key).and_then(|v| v.parse::<f32>().ok())
}

fn attr_bool(attrs: &HashMap<String, String>, key: &str) -> Option<bool> {
    attrs
        .get(key)
        .and_then(|v| match v.to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" => Some(true),
            "false" | "0" | "no" => Some(false),
            _ => None,
        })
}

fn apply_shadow_attrs(attrs: &HashMap<String, String>, shadow: &mut Option<ShadowStyle>) {
    let has_shadow = attr_bool(attrs, "shadow");
    let shadow_color = attr_color(attrs, "shadow_color");
    let shadow_blur = attr_f32(attrs, "shadow_blur");

    if has_shadow == Some(false) {
        *shadow = None;
        return;
    }

    if has_shadow == Some(true) || shadow_color.is_some() || shadow_blur.is_some() {
        let mut s = shadow.unwrap_or_default();
        if let Some(c) = shadow_color {
            s.color = c;
        }
        if let Some(b) = shadow_blur {
            s.blur_radius = b;
        }
        *shadow = Some(s);
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
    if let Some(c) = attr_color(attrs, "border_color") {
        style.border_color = c;
    }
    if let Some(w) = attr_f32(attrs, "border_width") {
        style.border_width = w;
    }
    if let Some(r) = attr_f32(attrs, "corner_radius") {
        style.corner_radius = r;
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
    if let Some(c) = attr_color(attrs, "border_color") {
        style.border_color = c;
    }
    if let Some(w) = attr_f32(attrs, "border_width") {
        style.border_width = w;
    }
    if let Some(r) = attr_f32(attrs, "corner_radius") {
        style.corner_radius = r;
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
    if let Some(c) = attr_color(attrs, "border_color") {
        style.border_color = c;
    }
    if let Some(w) = attr_f32(attrs, "border_width") {
        style.border_width = w;
    }
    if let Some(r) = attr_f32(attrs, "corner_radius") {
        style.corner_radius = r;
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
    if let Some(c) = attr_color(attrs, "border_color") {
        style.border_color = c;
    }
    if let Some(w) = attr_f32(attrs, "border_width") {
        style.border_width = w;
    }
    if let Some(r) = attr_f32(attrs, "corner_radius") {
        style.corner_radius = r;
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
        ui.set_bgcolor(Theme::BG_CANVAS);

        let command = ui
            .widget_state::<TextEditState>("command_input")
            .text()
            .to_string();
        let parsed = parse_command(&command);

        let title = match &parsed {
            Some((widget, _)) => format!("Style Playground - editing: {widget}"),
            None => "Style Playground - type a command below".to_string(),
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
        let mut mode_badge = Badge::new("LIVE REPL").variant(BadgeVariant::Success);
        let mut hint_label = Label::new(
            "Format: widget|key:value,key:value   e.g. checkbox|corner_radius:12.0,color:#f5656f",
        );
        let mut divider_top = Divider::horizontal(620.0);
        let mut divider_mid = Divider::horizontal(620.0);

        let mut command_caption = Label::new("Command Input");
        let mut command_input = TextInput::new("command_input", 580.0);

        let mut preview_caption = Label::new("Live Render Preview");

        let mut button_caption = Label::new("Button");
        let mut checkbox_caption = Label::new("Checkbox");
        let mut input_caption = Label::new("TextInput");
        let mut area_caption = Label::new("TextArea");
        let mut scroll_caption = Label::new("ScrollView");

        let mut demo_button = Button::new("btn", "Button").style(button_style_value);
        let mut demo_checkbox = Checkbox::new("demo_checkbox").style(checkbox_style_value);
        let mut demo_text_input = TextInput::new("demo_text_input", 170.0).style(input_style_value);
        let mut demo_text_area =
            TextArea::new("demo_text_area", 270.0, 80.0).style(area_style_value);

        let mut scroll_item_1 = Label::new("Line one");
        let mut scroll_item_2 = Label::new("Line two");
        let mut scroll_item_3 = Label::new("Line three");
        let mut scroll_item_4 = Label::new("Line four");
        let mut scroll_item_5 = Label::new("Line five");

        let mut btn_col = column![&mut button_caption, &mut demo_button]
            .spacing(6.0)
            .align(Alignment::Start);
        let mut chk_col = column![&mut checkbox_caption, &mut demo_checkbox]
            .spacing(6.0)
            .align(Alignment::Start);
        let mut inp_col = column![&mut input_caption, &mut demo_text_input]
            .spacing(6.0)
            .align(Alignment::Start);
        let mut row_controls = row![&mut btn_col, &mut chk_col, &mut inp_col]
            .spacing(16.0)
            .align(Alignment::Start);

        let mut scroll_content = column![
            &mut scroll_item_1,
            &mut scroll_item_2,
            &mut scroll_item_3,
            &mut scroll_item_4,
            &mut scroll_item_5,
        ];

        let mut demo_scroll = ScrollView::new("demo_scroll", [270.0, 80.0], &mut scroll_content)
            .style(scroll_style_value);

        let mut area_col = column![&mut area_caption, &mut demo_text_area]
            .spacing(6.0)
            .align(Alignment::Start);
        let mut scr_col = column![&mut scroll_caption, &mut demo_scroll]
            .spacing(6.0)
            .align(Alignment::Start);
        let mut row_inputs = row![&mut area_col, &mut scr_col]
            .spacing(16.0)
            .align(Alignment::Start);

        let mut card_content = column![&mut preview_caption, &mut row_controls, &mut row_inputs,]
            .spacing(12.0)
            .align(Alignment::Start);

        let mut interactive_card = Card::new(&mut card_content).style(CardStyle {
            padding: [18.0, 18.0],
            ..Default::default()
        });

        let mut header_row = row![&mut title_label, &mut mode_badge]
            .spacing(10.0)
            .align(Alignment::Center);

        column![
            &mut header_row,
            &mut hint_label,
            &mut divider_top,
            &mut command_caption,
            &mut command_input,
            &mut divider_mid,
            &mut interactive_card,
        ]
        .spacing(10.0)
        .align(Alignment::Start)
        .arrange_at([40.0, 30.0], ui);
    }
}

fn main() {
    let state = AppState::new();
    App::new(state).run();
}
