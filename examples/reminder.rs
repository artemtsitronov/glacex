use glacex::*;

struct ReminderApp;

impl Widget for ReminderApp {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Theme::BG_CANVAS);

        let window_size = ui.window_size();

        let mut heading = Label::new("Simple Reminder app").heading();
        let mut subheading =
            Label::new("This app is made with Glacex, and it's meant for showcase").subheading();

        glacex::column![
            &mut row![&mut glacex::column![&mut heading, &mut subheading],]
                .align(Alignment::Center)
                .height(window_size[1])
        ]
        .align(Alignment::Center)
        .size(window_size)
        .arrange_at([0.0, 0.0], ui);
    }
}

fn main() {
    App::new(ReminderApp).run();
}
