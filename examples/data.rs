use glacex::*;

struct DataDisplay;

impl Widget for DataDisplay {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let mut title = Typography::new("Data display").variant(TypographyVariant::Title);
        let mut tabs = Tabs::new("data-tabs", ["Overview", "Activity", "Settings"]).width(420.0);
        let mut chart = Chart::new([18.0, 28.0, 22.0, 38.0, 32.0]).size([460.0, 180.0]);
        let mut table = Table::new(
            "projects",
            ["Project", "Status", "Owner"],
            [
                ["Glacex", "Ready", "Mina"],
                ["Docs", "Draft", "Ari"],
                ["Examples", "Ready", "Noah"],
            ],
        )
        .width(460.0);
        let mut pagination = Pagination::new("projects-pages", 5);
        let mut calendar = Calendar::new(
            "calendar",
            CalendarDate {
                year: 2026,
                month: 9,
                day: 9,
            },
        );
        let mut layout = glacex::column![
            &mut title,
            &mut tabs,
            &mut chart,
            &mut table,
            &mut pagination,
            &mut calendar
        ]
        .spacing(18.0)
        .align(Alignment::Start);
        layout.arrange_at([48.0, 40.0], ui);
    }
}

fn main() {
    App::new(DataDisplay)
        .title("Glacex — Data Display")
        .window_size(900, 900)
        .run();
}
