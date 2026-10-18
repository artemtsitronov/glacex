use glacex::{
    App, Card, CardStyle, Fill, MeasurablePath, Motion, Path, ScrollConfig, ScrollView,
    ScrollViewStyle, TabItem, Tabs, TabsStyle, Theme, Ui, Widget, column, row,
};
use kurbo::BezPath;

struct CustomizationExample;

impl Widget for CustomizationExample {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_theme(Theme::DARK);
        let window_size = ui.window_size();
        let theme = ui.theme();

        let gpui_style = TabsStyle {
            height: 32.0.into(),
            tab_padding_x: 12.0.into(),
            gap: 0.0.into(),
            sharp: true,
            path: Path::rect([0.0; 4]).into(),
            tab_path: Path::rect([0.0; 4]).into(),
            indicator_path: Path::rect([0.0; 4]).into(),
            pill_inset_y: 0.0.into(),
            fill: Fill::Solid(theme.surface).into(),
            tab_fill: TabsStyle::underline(theme).tab_fill,
            indicator_fill: Fill::Solid(theme.bg_canvas).into(),
            indicator_motion: Motion::MICRO,
            hover_motion: Motion::MICRO,
            ..TabsStyle::from_theme(theme)
        };

        let items: Vec<TabItem> = (1..=10)
            .map(|i| TabItem::new(format!("Tab {i}")).id(format!("tab1{i}")))
            .collect();
        let mut tabs1 = Tabs::new(items).style(gpui_style.clone()).id("tab1");
        let mut tabs2 = Tabs::new(vec![
            TabItem::new("Tab 1").id("tab21"),
            TabItem::new("Tab 2").id("tab22"),
            TabItem::new("Tab 3").id("tab23"),
        ])
        .style(TabsStyle {
            indicator_path: Path::from_fn(move |[x, y], [w, h]| {
                let (x, y, w, h, s) = (x as f64, y as f64, w as f64, h as f64, 8.0);

                let mut p = BezPath::new();
                p.move_to((x, y + h)); // bottom left
                p.line_to((x + s, y)); // top left shifted in
                p.line_to((x + w - s, y)); // top right shifted in
                p.line_to((x + w, y + h)); // bottom right
                p.close_path();

                MeasurablePath::free(p)
            })
            .into(),
            ..TabsStyle::from_theme(ui.theme())
        })
        .id("tab2");

        let mut scroll_view = ScrollView::new(&mut tabs1)
            .style(ScrollViewStyle {
                scrollbar: ScrollConfig {
                    thickness: 0.0,
                    ..Default::default()
                }
                .into(),
                ..ScrollViewStyle::from_theme(ui.theme())
            })
            .id("scroll-view")
            .size([500.0, gpui_style.height.idle]);

        let mut card = Card::new(&mut scroll_view).style(CardStyle {
            padding: [0.0; 2].into(),
            clip_content: true,
            border_color: ui.theme().surface.into(),
            border_width: 2.0.into(),
            path: Path::rect([2.0; 4]).into(),
            ..CardStyle::from_theme(ui.theme())
        });

        let mut view = column![&mut card, &mut tabs2].spacing(20.0);

        column![&mut row![&mut view].height(window_size[1])]
            .size(window_size)
            .arrange_at([0.0; 2], ui);
    }
}

fn main() {
    App::new(CustomizationExample {}).run()
}
