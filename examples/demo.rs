use glacex::{
    Alignment, App, Button, ButtonStyle, Checkbox, CheckboxState, Color, Fill, Gradient,
    GradientKind, GradientStop, Label, RadioButton, ScrollView, ShadowStyle, TextArea,
    TextEditState, TextInput, Ui, Widget, column, row,
};
use std::default::Default;

fn hue_gradient(hue_offset: f32) -> Fill {
    let hue_a = hue_offset % 360.0;
    let hue_b = (hue_offset + 60.0) % 360.0;
    Fill::Gradient(Gradient {
        kind: GradientKind::Conic { center: [0.0, 0.0] },
        stops: vec![
            GradientStop {
                position: 0.0,
                color: Color::hsv(hue_a, 0.8, 1.0),
            },
            GradientStop {
                position: 1.0,
                color: Color::hsv(hue_b, 0.8, 1.0),
            },
        ],
    })
}

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
        ui.set_bgcolor(Color::rgb(16, 16, 20));
        let hue = (ui.elapsed_seconds() * 60.0) % 360.0; // 60 = degrees/sec, tune to taste
        let fill = hue_gradient(hue);

        let mut title = Label::new("glacex demo");
        let mut subtitle = Label::new("Every widget below is wired to real, readable state.");

        let mut count_label = Label::new(format!("Count: {}", self.count));
        let mut increment_button = Button::new("Increment").style(ButtonStyle {
            fill: fill.clone(),
            hover_fill: fill.clone().darken(0.3),
            pressed_fill: fill.clone().darken(0.7),
            border_width: 2.0,
            border_color: Color::BLACK,
            corner_radius: 20.0,
            ..Default::default()
        });
        let mut reset_button = Button::new("Reset").style(ButtonStyle {
            shadow: Some(ShadowStyle {
                color: Color::RED,
                blur_radius: 10.0,
                offset: [0.0, 0.0],
            }),
            ..Default::default()
        });

        let mut notify_checkbox = Checkbox::new("notify_checkbox");
        let mut sound_checkbox = Checkbox::new("sound_checkbox");

        let mut theme_light = RadioButton::new("theme", "Light");
        let mut theme_dark = RadioButton::new("theme", "Dark");

        let mut name_input = TextInput::new("name_input", 220.0);

        let mut activity_1 = Label::new("Loaded widget tree");
        let mut activity_2 = Label::new("Registered focusables");
        let mut activity_3 = Label::new("Waiting for input...");
        let mut notes_area = TextArea::new("notes_area", 220.0, 80.0);

        {
            row![
                &mut column![
                    &mut title,
                    &mut subtitle,
                    &mut count_label,
                    &mut row![&mut increment_button, &mut reset_button],
                    &mut Label::new("Name"),
                    &mut name_input,
                    &mut Label::new("Recent activity"),
                    &mut ScrollView::new(
                        "activity_scroll",
                        [240.0, 70.0],
                        &mut column![&mut activity_1, &mut activity_2, &mut activity_3],
                    ),
                    &mut Label::new("Notes"),
                    &mut notes_area,
                ]
                .align(Alignment::Start),
                &mut column![
                    &mut Label::new("Notifications"),
                    &mut row![
                        &mut notify_checkbox,
                        &mut Label::new("Enable notifications")
                    ]
                    .align(Alignment::Center),
                    &mut row![&mut sound_checkbox, &mut Label::new("Play sound")]
                        .align(Alignment::Center),
                    &mut Label::new("Theme"),
                    &mut row![&mut theme_light, &mut Label::new("Light")].align(Alignment::Center),
                    &mut row![&mut theme_dark, &mut Label::new("Dark")].align(Alignment::Center),
                ]
                .align(Alignment::Start),
            ]
            .align(Alignment::Start)
            .arrange_at([40.0, 40.0], ui);
        }

        // Button state is cached on the widget itself, so this works
        // straight off the widgets — no lookup needed.
        if increment_button.clicked() {
            self.count += 1;
        }
        if reset_button.clicked() {
            self.count = 0;
        }

        // Checkbox / RadioButton / TextInput / TextArea state lives in
        // `Ui`'s persistent state map, keyed by the id string you gave
        // them — read it back like this after the layout block runs.
        let notify_on = ui.widget_state::<CheckboxState>("notify_checkbox").checked;
        let sound_on = ui.widget_state::<CheckboxState>("sound_checkbox").checked;
        let theme = ui.selected_option("theme").unwrap_or("Light").to_string();
        let name = ui
            .widget_state::<TextEditState>("name_input")
            .text()
            .to_string();
        let notes = ui
            .widget_state::<TextEditState>("notes_area")
            .text()
            .to_string();

        // Nothing to do with these yet in this demo — they're here to
        // show the read-back pattern. Wire them into real logic as needed.
        let _ = (notify_on, sound_on, theme, name, notes);
    }
}

fn main() {
    let state = AppState::new();
    App::new(state).run();
}
