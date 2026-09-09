use glacex::{
    Alignment, App, Badge, BadgeVariant, Button, ButtonStyle, Card, CardStyle, Color, Divider,
    Fill, Label, ShadowStyle, TextInput, Theme, Ui, Widget, column, row,
};

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
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Theme::BG_CANVAS);

        let mut hex_input = TextInput::new("color_hex_input")
            .width(320.0)
            .default_text(&self.default_hex);
        let parsed_color = Color::hex_str(&hex_input.text(ui));

        let mut badge = Badge::new("COLOR").variant(BadgeVariant::Success);
        let mut title = Label::new("title", "Dynamic Color Preview");
        let mut subtitle = Label::new(
            "subtitle",
            "Enter a 6-digit hex code to update the button style in real time.",
        );
        let mut divider = Divider::horizontal(360.0);

        let mut input_label = Label::new("input_label", "Hex Code (#RRGGBB)");

        let mut preview_label = Label::new("preview_label", "Styled Preview");
        let mut preview_btn = Button::new("_a", "Sample Button").style(ButtonStyle {
            fill: Fill::Solid(parsed_color),
            hover_fill: Fill::Solid(parsed_color.lighten(0.15)),
            pressed_fill: Fill::Solid(parsed_color.darken(0.2)),
            text_color: Color::WHITE,
            border_width: 1.0,
            border_color: Color::WHITE.with_alpha(0.2),
            corner_radius: Theme::RADIUS_MD,
            padding: [14.0, 8.0],
            shadow: Some(ShadowStyle {
                color: parsed_color.with_alpha(0.35),
                blur_radius: 16.0,
                offset: [0.0, 4.0],
            }),
            sharp: false,
        });

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
    App::new(ColorDemo::new()).run();
}
