use glacex::{
    Alignment, App, Button, ButtonStyle, Color, Fill, Label, TextEditState, TextInput, Ui, Widget,
    column, row,
};

struct AppState {}

impl AppState {
    fn new() -> Self {
        AppState {}
    }
}

impl Widget for AppState {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Color::rgb(0, 0, 0));

        let mut title = Label::new("Example 1");
        let mut fill_input = TextInput::new("fill_input", 200.0);

        let fill = ui.widget_state::<TextEditState>("fill_input").text();
        let mut button = Button::new("Button").style(ButtonStyle {
            fill: Fill::Solid(Color::hex_str(fill)),
            ..Default::default()
        });

        {
            row![
                &mut column![
                    &mut title,
                    &mut button,
                    &mut row![&mut Label::new("Fill hex: "), &mut fill_input]
                        .align(Alignment::Center),
                ]
                .align(Alignment::Center)
            ]
            .align(Alignment::Center)
            .arrange_at([40.0, 40.0], ui);
        }
    }
}

fn main() {
    let state = AppState::new();
    App::new(state).run();
}
