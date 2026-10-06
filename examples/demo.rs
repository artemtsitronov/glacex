//! demo -- the kitchen sink. every widget, one scrollable dashboard.
//!
//! this is the "see everything at once" example. it demonstrates:
//!   - theme switching two ways (button cycles, SelectBox jumps)
//!   - loading an image in `on_start` and using it as a card fill
//!   - raw `Ui` state (`take_widget_state`) for cross-widget logic
//!     (the good/fast/cheap pick-two-of-three switches)
//!   - radio groups, sliders driving progress bars, tabs, tooltips,
//!     gradient button styles, nested scroll views
//!
//! new here? start with `example1` instead. this file is a buffet, not a meal.
//!
//! run it: `cargo run --example demo`

use glacex::*;

// our data: which theme is active, plus an image loaded once at startup.
struct DemoApp {
    current_theme_idx: usize,
    kitty_image: Option<ImageHandle>,
}

impl DemoApp {
    fn new() -> Self {
        DemoApp {
            current_theme_idx: 1,
            kitty_image: None,
        }
    }
}

// plain struct showing how to keep YOUR OWN state in Ui's map.
// it tracks the order switches were flipped, so we can un-flip the oldest.
#[derive(Default)]
struct TripleToggleOrder {
    order: Vec<&'static str>,
}

impl Widget for DemoApp {
    type Output = ();

    // runs ONCE before the first frame. load images (or do other setup) here --
    // doing it in ui() would reload every frame, which is how you melt a gpu.
    fn on_start(&mut self, ui: &mut Ui) {
        // We load an image in our state using ui.load_image().
        self.kitty_image = Some(ui.load_image("assets/demo_images/1.jpg"));
    }

    fn ui(&mut self, ui: &mut Ui) {
        // -- theme first: everything below follows it -------------------------
        let themes = Theme::all();
        let current_theme = themes[self.current_theme_idx % themes.len()];
        ui.set_theme(current_theme);

        let window_size = ui.window_size();

        let mut window_size_label = Label::new(format!(
            "Window size: {:.2} x {:.2}",
            window_size[0], window_size[1]
        ));
        let mut theme_label = Label::new(format!("Current theme: {}", current_theme.name));
        let mut theme_btn = Button::new("Change Theme").id("theme_btn");

        // -- the pick-two-of-three switches ------------------------------------
        // three switches, but at most two may be on. the rule needs memory
        // spanning all three widgets, so it lives in Ui under our own key --
        // not in any one widget's state.
        let mut good_switch = Switch::new().id("good_switch");
        let mut fast_switch = Switch::new().id("fast_switch");
        let mut cheap_switch = Switch::new().id("cheap_switch");

        // take our custom state OUT of Ui (owned now, Ui forgot it for a bit)...
        let mut order_state = ui.take_widget_state::<TripleToggleOrder>("triple_toggle_order");

        // ...record which switches are currently on, in flip order...
        // (reading .enabled() BEFORE layout sees last frame's value -- fine,
        // the switches below were already arranged... last frame. memory!)
        let states = [
            ("good_switch", good_switch.enabled(ui)),
            ("fast_switch", fast_switch.enabled(ui)),
            ("cheap_switch", cheap_switch.enabled(ui)),
        ];

        for (id, enabled) in states {
            if enabled && !order_state.order.contains(&id) {
                order_state.order.push(id);
            }
            if !enabled {
                order_state.order.retain(|&x| x != id);
            }
        }

        // ...evict the oldest if all three ended up on...
        while order_state.order.len() > 2 {
            let evicted = order_state.order.remove(0);
            match evicted {
                "good_switch" => good_switch.set_enabled(ui, false),
                "fast_switch" => fast_switch.set_enabled(ui, false),
                "cheap_switch" => cheap_switch.set_enabled(ui, false),
                _ => {}
            }
        }

        // ...and hand the updated state BACK to Ui. take → modify → put.
        ui.put_widget_state("triple_toggle_order", order_state);

        // -- radio group: one group id, one option id each ---------------------
        // read the winner anywhere with ui.selected_option("radio_group").
        let mut radio_yes_option = RadioButton::new("radio_group").id("yes");
        let mut radio_yes_label = Label::new("Yes");
        let mut radio_button_yes = row![&mut radio_yes_option, &mut radio_yes_label];

        let mut radio_no_option = RadioButton::new("radio_group").id("no");
        let mut radio_no_label = Label::new("No");
        let mut radio_button_no = row![&mut radio_no_option, &mut radio_no_label];

        let mut radio_button_group =
            glacex::column![&mut radio_button_yes, &mut radio_button_no].align(Alignment::End);

        // rebuilt every frame from the group's state -- always correct, never stale.
        let mut verdict_label = Label::new({
            match ui.selected_option("radio_group") {
                Some("yes") => "Yes. Excellent taste.",
                Some("no") => "No. Bold. Wrong, but bold.",
                _ => "Pick one. No pressure.",
            }
        });

        let mut slider = Slider::new(0.0, 1.0).id("slider");
        let mut progress_bar = ProgressBar::new(slider.value(ui)).id("slider_progress_bar"); // In this widget, id is optional, so we define it using .id()

        // -- theme picker dropdown: one SelectOption per theme -------------------
        let theme_options: Vec<SelectOption> = themes
            .iter()
            .enumerate()
            .map(|(i, t)| SelectOption::new(i.to_string(), t.name))
            .collect();
        let mut theme_select = SelectBox::new(theme_options)
            .id("theme_select")
            .placeholder("Select theme...")
            .width(200.0);

        // the image was loaded in on_start; unwrap is safe -- startup ran first.
        let kitty_image = self.kitty_image.unwrap();

        // -- the big layout: rows in columns in cards in a scroll view -----------
        {
            row![
                &mut ScrollView::new(
                    &mut glacex::column![
                        // tabs with an animated sliding pill
                        &mut Tabs::new(vec![
                            TabItem::new("Tab 1").id("tab_1"),
                            TabItem::new("Tab 2").id("tab_2"),
                            TabItem::new("Tab 3").id("tab_3"),
                        ])
                        .id("tabs"),
                        &mut row![
                            &mut window_size_label,
                            &mut Divider::vertical(24.0).thickness(2.0),
                            &mut theme_label,
                            &mut theme_btn
                        ],
                        &mut row![
                            &mut Card::new(
                                &mut ScrollView::new(
                                    &mut glacex::column![
                                        // a card FILLED WITH AN IMAGE. fancy.
                                        &mut Card::new(&mut Label::new("Kitty!"))
                                            .style(CardStyle {
                                                fill: Fill::Image(kitty_image),
                                                ..CardStyle::subtle()
                                            })
                                            .size(kitty_image.scale(0.2)),
                                        // badges in all flavors
                                        &mut row![
                                            &mut Badge::new("Badge").variant(BadgeVariant::Outline),
                                            &mut Badge::new("Success").success(),
                                            &mut Badge::new("Warning").warning(),
                                            &mut Badge::new("Dangerous stuff").error(),
                                        ],
                                        // buttons: plain, gradient + pill-shaped, fat + tooltip
                                        &mut row![
                                            &mut Button::new("Button").id("plain_btn"),
                                            &mut Button::new("Fashion button")
                                                .id("fashion_btn")
                                                .style(ButtonStyle {
                                                    fill: Fill::Gradient(Gradient {
                                                        kind: GradientKind::Linear { angle: 45.0 },
                                                        stops: vec![
                                                            GradientStop {
                                                                position: 0.0,
                                                                color: Color::rgb(230, 230, 230)
                                                            },
                                                            GradientStop {
                                                                position: 1.0,
                                                                color: Color::rgb(120, 200, 180)
                                                            }
                                                        ],
                                                    }),
                                                    path: Path::ellipse(0.0),
                                                    ..Default::default()
                                                }),
                                            &mut Button::new("Fat button")
                                                .id("fat_btn")
                                                .size([100.0, 50.0])
                                                .tooltip(
                                                    "In case you missed it, this is a fat button."
                                                )
                                        ]
                                        .align(Alignment::Center),
                                        // two columns side by side: plain checkboxes vs the triple switches
                                        &mut row![
                                            &mut glacex::column![
                                                &mut row![
                                                    &mut Label::new("Checkboxy"),
                                                    &mut Checkbox::new().id("checkbox_1"),
                                                ],
                                                &mut row![
                                                    &mut Label::new("Another one"),
                                                    &mut Checkbox::new().id("checkbox_2"),
                                                ],
                                                &mut row![
                                                    &mut Label::new("Hehe"),
                                                    &mut Checkbox::new().id("checkbox_3"),
                                                ]
                                            ]
                                            .align(Alignment::End),
                                            &mut glacex::column![
                                                &mut row![
                                                    &mut good_switch,
                                                    &mut Label::new("Good"),
                                                ],
                                                &mut row![
                                                    &mut fast_switch,
                                                    &mut Label::new("Fast"),
                                                ],
                                                &mut row![
                                                    &mut cheap_switch,
                                                    &mut Label::new("Cheap"),
                                                ]
                                            ]
                                            .align(Alignment::Start)
                                        ]
                                        .align(Alignment::Center)
                                        .spacing(100.0),
                                        // theme dropdown + text widgets to play with
                                        &mut theme_select,
                                        &mut TextInput::new()
                                            .id("text_input")
                                            .placeholder("Here goes text."),
                                        &mut TextArea::new().id("text_area"),
                                    ]
                                    .spacing(24.0)
                                )
                                .id("card_scroll")
                                .padding([12.0; 2])
                            )
                            .size([400.0, 600.0])
                            .id("gallery_scroll")
                            .padding([0.0; 2]),
                            // -- right card: radios, type scale, slider, kitty 2 ------
                            &mut Card::new(
                                &mut glacex::column![
                                    &mut row![&mut radio_button_group, &mut verdict_label,]
                                        .spacing(24.0)
                                        .align(Alignment::Center),
                                    // the label size scale, small to large
                                    &mut glacex::column![
                                        &mut Label::new("Heading").heading(),
                                        &mut Label::new("Subheading").subheading(),
                                        &mut Label::new("Caption").caption(),
                                    ]
                                    .spacing(12.0)
                                    .align(Alignment::Start),
                                    &mut slider,
                                    &mut progress_bar,
                                    // same image, second card. reuse is free.
                                    &mut Card::new(&mut Label::new("Another kitty!"))
                                        .style(CardStyle {
                                            fill: Fill::Image(kitty_image),
                                            ..CardStyle::subtle()
                                        })
                                        .size(kitty_image.scale(0.2)),
                                ]
                                .spacing(24.0)
                                .align(Alignment::Center)
                            )
                            .height(600.0)
                        ],
                        // farewell footer
                        &mut glacex::column![
                            &mut Label::new("And here, our demo ends. Thanks for scrolling.")
                                .size_preset(18.0),
                            &mut Label::new("Be free to check out glacex on github. <3")
                                .size_preset(14.0),
                        ],
                    ]
                    .padding([50.0; 2])
                    .width(window_size[0])
                    .spacing(30.0)
                    .align(Alignment::Center)
                )
                .id("main_scroll")
                .size([window_size[0], window_size[1]])
                .id("page_scroll")
            ]
            .align(Alignment::Center)
            .arrange_at([0.0; 2], ui); // measure everything, draw at the origin.
        }

        // -- react: AFTER layout ------------------------------------------------
        // the button knows it was clicked; the dropdown knows its selection.
        if theme_btn.clicked() {
            self.current_theme_idx = (self.current_theme_idx + 1) % themes.len();
        }

        if let Some(idx_str) = theme_select.selected(ui) {
            if let Ok(idx) = idx_str.parse::<usize>() {
                if idx < themes.len() {
                    self.current_theme_idx = idx;
                }
            }
        }
    }
}

fn main() {
    App::new(DemoApp::new())
        .title("Glacex - High Performance GPU UI Demo")
        .window_size(1360, 920)
        .run();
}
