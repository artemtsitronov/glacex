use glacex::{
    Alignment, App, Badge, BadgeVariant, Border, Button, ButtonStyle, Card, CardStyle, Color,
    Divider, FontWeight, Label, ShadowStyle, Stateful, TextInput, TextStyle, Ui, Widget, column,
    row,
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
        ui.set_bgcolor(ui.theme().bg_canvas);

        let mut hex_input = TextInput::new()
            .width(320.0)
            .id("text-input")
            .default_text(&self.default_hex);
        let parsed_color = Color::hex_str(&hex_input.text(ui));

        let mut badge = Badge::new("COLOR").variant(BadgeVariant::Success);
        let mut title = Label::new("Dynamic Color Preview");
        let mut subtitle =
            Label::new("Enter a 6-digit hex code to update the button style in real time.");
        let mut divider = Divider::horizontal(360.0);

        let mut input_label = Label::new("Hex Code (#RRGGBB)");

        let mut preview_label = Label::new("Styled Preview");
        let mut preview_btn = Button::new("Sample Button")
            .id("sample-button")
            .text_style(
                TextStyle::new()
                    .size(14.0, 20.0)
                    .weight(FontWeight::Medium)
                    .color(Color::WHITE),
            )
            .style(ButtonStyle {
                path: glacex::Path::rect([ui.theme().metrics.radius_md; 4]).into(),
                ..Default::default()
            })
            .fill(
                Stateful::new(parsed_color)
                    .hovered(parsed_color.lighten(0.15))
                    .pressed(parsed_color.darken(0.2)),
            )
            .border(Border::new(1.0, Color::WHITE.with_alpha(0.2)))
            .shadow(ShadowStyle {
                color: parsed_color.with_alpha(0.35),
                blur_radius: 16.0,
                offset: [0.0, 4.0],
                spread: 0.0,
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
            padding: [24.0, 24.0].into(),
            path: glacex::Path::rect([16.0; 4]).into(),
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
