use glacex::*;

struct Foundations;

#[derive(Default)]
struct FoundationState {
    saved: bool,
}

impl Widget for Foundations {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let theme = *ui.theme();
        let saved = ui.widget_state::<FoundationState>("foundation-state").saved;
        let mut title = Typography::new("Foundations").variant(TypographyVariant::Title);
        let mut body = Typography::new("Core controls, feedback, and content primitives")
            .color(theme.text_secondary);
        let mut badge_default = Badge::new("Default");
        let mut badge_success = Badge::new("Success").success();
        let mut badge_warning = Badge::new("Warning").warning();
        let mut badge_error = Badge::new("Error").error();
        let mut badges = row![
            &mut badge_default,
            &mut badge_success,
            &mut badge_warning,
            &mut badge_error
        ]
        .spacing(8.0);
        let mut primary = Button::new("primary", "Continue").primary();
        let mut secondary = Button::new("secondary", "Cancel").outline();
        let mut shortcut = Kbd::new(format!("{}  K", NerdIcon::Search.mono()));
        let mut actions = row![&mut primary, &mut secondary, &mut shortcut].spacing(10.0);
        let mut check = Checkbox::new("notifications");
        let mut switch = Switch::new("live-preview");
        let mut slider = Slider::new("intensity", 0.0, 1.0);
        let mut controls = row![&mut check, &mut switch, &mut slider]
            .spacing(18.0)
            .align(Alignment::Center);
        let mut name = TextInput::new("name").placeholder("Your name");
        let mut notes = TextArea::new("notes");
        let mut theme_select = SelectBox::new(
            "theme",
            vec![
                SelectOption::new("light", "Light"),
                SelectOption::new("dark", "Dark"),
            ],
        )
        .placeholder("Choose a theme");
        let mut fields = glacex::column![&mut name, &mut notes, &mut theme_select,].spacing(12.0);
        let mut ready = if saved {
            Alert::new("Preferences saved").variant(AlertVariant::Success)
        } else {
            Alert::new("Change a control, then continue").variant(AlertVariant::Info)
        };
        let mut progress = ProgressBar::new(0.72);
        let mut spinner = Spinner::new(18.0);
        let mut feedback = row![&mut ready, &mut progress, &mut spinner,]
            .spacing(14.0)
            .align(Alignment::Center);
        let mut layout = glacex::column![
            &mut title,
            &mut body,
            &mut badges,
            &mut actions,
            &mut controls,
            &mut fields,
            &mut feedback
        ]
        .spacing(18.0)
        .align(Alignment::Start);
        layout.arrange_at([48.0, 40.0], ui);
        drop(layout);
        drop(feedback);
        drop(fields);
        drop(controls);
        drop(actions);

        if primary.clicked() {
            ui.widget_state::<FoundationState>("foundation-state").saved = true;
        }
        if secondary.clicked() {
            ui.widget_state::<FoundationState>("foundation-state").saved = false;
        }
    }
}

fn main() {
    App::new(Foundations)
        .title("Glacex — Foundations")
        .window_size(900, 700)
        .run();
}
