use gpu::{
    Alignment, App, Button, Checkbox, Color, Label, RadioButton, ScrollView, TextArea, TextInput,
    Ui, Widget, column, row,
};

struct AppState {
    count: u32,
}

impl AppState {
    fn new() -> Self {
        AppState { count: 0 }
    }
}

impl Widget for AppState {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Color::rgb(12, 12, 12));

        let increment_clicked;
        let reset_clicked;

        {
            let mut count_label = Label::new(format!("Counter: {}", self.count));
            let mut increment_button = Button::new("Increment");
            let mut reset_button = Button::new("Reset");
            let mut intro_label = Label::new("GPU-rendered Rust UI demo");
            let mut hint_label = Label::new("Try scrolling, tab focus, typing, and selection.");
            let mut item_one = Label::new("This scroll view contains regular widgets.");
            let mut item_two = Label::new("The counter updates when you press Increment.");
            let mut item_three = Label::new("Text inputs preserve state between frames.");

            row![
                &mut column![
                    &mut intro_label,
                    &mut hint_label,
                    &mut count_label,
                    &mut ScrollView::new(
                        "main_scroll",
                        [260.0, 110.0],
                        &mut column![
                            &mut item_one,
                            &mut item_two,
                            &mut item_three,
                            &mut increment_button,
                        ],
                    ),
                    &mut row![
                        &mut reset_button,
                        &mut column![
                            &mut Checkbox::new("checkbox_a"),
                            &mut Checkbox::new("checkbox_b"),
                        ],
                    ]
                    .align(Alignment::Center),
                    &mut TextInput::new("input_a", 220.0),
                    &mut TextInput::new("input_b", 220.0),
                    &mut TextArea::new("textarea", 220.0, 110.0),
                ]
                .align(Alignment::Start),
                &mut column![
                    &mut RadioButton::new("theme", "light"),
                    &mut RadioButton::new("theme", "dark"),
                    &mut RadioButton::new("theme", "auto"),
                ]
            ]
            .align(Alignment::Center)
            .arrange_at([50.0, 50.0], ui);

            increment_clicked = increment_button.clicked();
            reset_clicked = reset_button.clicked();
        }

        if increment_clicked {
            self.count += 1;
        }
        if reset_clicked {
            self.count = 0;
        }
    }
}

fn main() {
    let state = AppState::new();
    App::new(state).run();
}
