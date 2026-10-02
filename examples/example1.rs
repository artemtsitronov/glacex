//! example1 -- the smallest meaningful app: a live color preview card.
//!
//! what you will learn here:
//!   1. the shape of every glacex app (state struct + `impl Widget` + `main`)
//!   2. build → layout → react, the three steps inside every `ui()`
//!   3. reading a text field with `.text(ui)`
//!   4. styling a button from scratch with `ButtonStyle`
//!
//! run it: `cargo run --example example1`
//! then type a hex color like `#22c55e` and watch the button change its shirt.

use glacex::{
    Alignment, App, Badge, BadgeVariant, Button, ButtonStyle, Card, CardStyle, Color, Divider,
    Fill, Label, ShadowStyle, TextInput, Theme, Ui, Widget, column, row,
};

// ---------------------------------------------------------------------------
// your app's data. this struct outlives frames -- it is created once in
// `main` and `ui()` gets `&mut` access to it 60 times a second.
// rule of thumb: YOUR data lives here (settings, lists, counters).
// WIDGET data (checkbox state, text contents) lives inside `Ui`.
// ---------------------------------------------------------------------------
struct ColorDemo {
    default_hex: String,
}

impl ColorDemo {
    fn new() -> Self {
        ColorDemo {
            default_hex: "#4f46e5".to_string(),
        }
    }
}

impl Widget for ColorDemo {
    // the root widget answers to nobody, so its output is `()`.
    type Output = ();

    // this runs every frame. describe the ui, every time, from scratch.
    fn ui(&mut self, ui: &mut Ui) {
        // paint the window background first, before any widget draws.
        ui.set_bgcolor(Theme::BG_CANVAS);

        // -- step 1: BUILD ------------------------------------------------
        // widgets are plain structs. creating them is cheap -- do it every frame.

        let mut hex_input = TextInput::new()
            .id("hex_input") // stable id → the typed text survives across frames
            .width(320.0)
            .default_text(&self.default_hex); // shown on the very first frame

        // read the field's CURRENT text out of Ui's memory.
        // (on frame one this is `default_text`; afterwards, whatever was typed.)
        let parsed_color = Color::hex_str(&hex_input.text(ui));

        let mut badge = Badge::new("COLOR").variant(BadgeVariant::Success);
        let mut title = Label::new("Dynamic Color Preview");
        let mut subtitle =
            Label::new("Enter a 6-digit hex code to update the button style in real time.");
        let mut divider = Divider::horizontal(360.0);
        let mut input_label = Label::new("Hex Code (#RRGGBB)");
        let mut preview_label = Label::new("Styled Preview");

        // a button wearing a fully custom outfit, sewn from the typed color.
        // lighten() for hover, darken() for press -- works for ANY base color.
        let mut preview_btn = Button::new("Sample Button")
            .id("preview_btn")
            .style(ButtonStyle {
                fill: Fill::Solid(parsed_color),
                hover_fill: Fill::Solid(parsed_color.lighten(0.15)),
                pressed_fill: Fill::Solid(parsed_color.darken(0.2)),
                text_color: Color::WHITE,
                border_width: 1.0,
                border_color: Color::WHITE.with_alpha(0.2),
                padding: [14.0, 8.0],
                shadow: Some(ShadowStyle {
                    color: parsed_color.with_alpha(0.35),
                    blur_radius: 16.0,
                    offset: [0.0, 4.0],
                }),
                sharp: false,
                path: glacex::Path::rect([Theme::RADIUS_MD; 4]),
            });

        // -- step 2: LAYOUT -----------------------------------------------
        // stack everything vertically, left-aligned, with 12px gaps...
        let mut badge_row = row![&mut badge].align(Alignment::Start);

        let mut card_content = column![
            &mut badge_row,
            &mut title,
            &mut subtitle,
            &mut divider,
            &mut input_label,
            &mut hex_input,
            &mut preview_label,
            &mut preview_btn,
        ]
        .spacing(12.0)
        .align(Alignment::Start);

        // ...wrap it in a padded card with extra-round corners...
        let mut card = Card::new(&mut card_content).style(CardStyle {
            padding: [24.0, 24.0],
            path: glacex::Path::rect([16.0; 4]),
            ..Default::default()
        });

        // ...and place the whole thing at x=40, y=40.
        // nothing draws until arrange_at is called. no arrange, no pixels.
        column![&mut card]
            .align(Alignment::Center)
            .arrange_at([40.0, 40.0], ui);

        // -- step 3: REACT --------------------------------------------------
        // nothing to react to here -- the preview updates itself every frame
        // because it is rebuilt from the input's text every frame.
        // that is the immediate-mode trick: no callbacks, no refresh calls.
    }
}

fn main() {
    // App::new(state) → open window → call ui() forever → close window.
    // `.title()` and `.window_size()` are optional; defaults exist.
    App::new(ColorDemo::new()).run();
}
