use glacex::*;

struct ComponentsDemo;

impl Widget for ComponentsDemo {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(ui.theme().bg_canvas);

        let mut title = Typography::new("Glacex 0.2").variant(TypographyVariant::Title);
        let mut subtitle =
            Typography::new("A restrained native component system for everyday product UI")
                .variant(TypographyVariant::Body)
                .color(ui.theme().text_secondary);
        let mut tabs = Tabs::new("demo-tabs", ["Overview", "Activity", "Settings"]).width(360.0);
        let mut chart = Chart::new([18.0, 28.0, 22.0, 38.0, 32.0]).size([420.0, 160.0]);
        let mut table = Table::new(
            "demo-table",
            ["Project", "Status"],
            [
                ["glacex", "Ready"],
                ["docs", "Draft"],
                ["examples", "Ready"],
            ],
        )
        .width(420.0);
        let mut alert = Alert::new("All systems operational").variant(AlertVariant::Success);
        let mut shortcut = Kbd::new(format!("{}  Ctrl K", NerdIcon::Search.mono()));
        let mut layout = glacex::column![
            &mut title,
            &mut subtitle,
            &mut tabs,
            &mut alert,
            &mut chart,
            &mut table,
            &mut shortcut,
        ]
        .spacing(14.0)
        .align(Alignment::Start);

        let size = ui.window_size();
        layout.arrange_at([32.0, 28.0], ui);
        let _ = size;
    }
}

fn main() {
    App::new(ComponentsDemo)
        .title("Glacex 0.2 Components")
        .window_size(760, 720)
        .run();
}
