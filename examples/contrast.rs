use glacex::*;

struct ContrastDemo;

impl Widget for ContrastDemo {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_theme(Theme::DARK);
        let theme = *ui.theme();
        let mut title = Typography::new("Dark theme contrast").variant(TypographyVariant::Title);
        let mut body =
            Typography::new("Semantic foregrounds stay readable on surfaces and active controls.")
                .color(theme.text_secondary);
        let mut primary = Button::new("primary", "Primary action").primary();
        let mut selected = Toggle::new("selected", "Selected control");
        let mut status = Alert::new("Contrast checks pass").variant(AlertVariant::Success);
        let mut ratio = Typography::new(format!(
            "Active foreground ratio: {:.2}:1",
            Theme::contrast_ratio(theme.on_active(), theme.active)
        ))
        .variant(TypographyVariant::Code)
        .color(theme.text_secondary);
        let mut layout = glacex::column![
            &mut title,
            &mut body,
            &mut primary,
            &mut selected,
            &mut status,
            &mut ratio,
        ]
        .spacing(14.0)
        .align(Alignment::Start);
        layout.arrange_at([48.0, 48.0], ui);
    }
}

fn main() {
    App::new(ContrastDemo)
        .title("Glacex - Contrast Showcase")
        .window_size(620, 420)
        .run();
}
