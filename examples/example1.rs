use glacex::{
    Alignment, App, Badge, BadgeVariant, Button, ButtonStyle, Card, CardStyle, Color, Divider,
    Fill, Label, ShadowStyle, TextEditState, TextInput, Theme, Ui, Widget, column, row,
};

struct AppState {
    last_hex: String,
}

impl AppState {
    fn new() -> Self {
        AppState {
            last_hex: "#4f46e5".to_string(),
        }
    }
}

impl Widget for AppState {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Theme::BG_CANVAS);

        let input_state = ui.widget_state::<TextEditState>("fill_input");
        if input_state.text().is_empty() {
            input_state.set_text(&self.last_hex);
        }
        let current_text = input_state.text().to_string();

        let parsed_color = Color::hex_str(&current_text);

        let mut header_badge = Badge::new("COLOR ENGINE").variant(BadgeVariant::Success);
        let mut title = Label::new("Dynamic Palette & Surface Playground");
        let mut subtitle = Label::new(
            "Type any valid 6-char hex code below to dynamically re-style live GPU primitives.",
        );

        let mut divider = Divider::horizontal(360.0);

        let mut input_caption = Label::new("Hex Code (#RRGGBB)");
        let mut fill_input = TextInput::new("fill_input", 320.0);

        let mut preview_label = Label::new("Interactive Button Surface");
        let mut preview_btn = Button::new("Press to Test Fill").style(ButtonStyle {
            fill: Fill::Solid(parsed_color),
            hover_fill: Fill::Solid(parsed_color.lighten(0.15)),
            pressed_fill: Fill::Solid(parsed_color.darken(0.2)),
            border_width: 1.0,
            border_color: Color::WHITE.with_alpha(0.2),
            corner_radius: 8.0,
            shadow: Some(ShadowStyle {
                color: parsed_color.with_alpha(0.35),
                blur_radius: 16.0,
                offset: [0.0, 4.0],
            }),
            sharp: false,
        });

        let mut header_row = row![&mut header_badge].align(Alignment::Start);

        let mut card_content = column![
            &mut header_row,
            &mut title,
            &mut subtitle,
            &mut divider,
            &mut input_caption,
            &mut fill_input,
            &mut preview_label,
            &mut preview_btn,
        ]
        .spacing(12.0)
        .align(Alignment::Start);

        let mut card = Card::new(&mut card_content).style(CardStyle {
            padding: [24.0, 24.0],
            corner_radius: 16.0,
            ..Default::default()
        });

        column![&mut card]
            .align(Alignment::Center)
            .arrange_at([40.0, 40.0], ui);
    }
}

fn main() {
    let state = AppState::new();
    App::new(state).run();
}
