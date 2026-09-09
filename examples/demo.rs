use glacex::*;

struct DemoApp {
    current_theme_idx: usize,
}

impl DemoApp {
    fn new() -> Self {
        DemoApp {
            current_theme_idx: 1,
        }
    }
}

#[derive(Default)]
struct TripleToggleOrder {
    order: Vec<&'static str>,
}

impl Widget for DemoApp {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let themes = Theme::all();

        // Check if theme was selected via SelectBox
        if let Some(idx_str) = ui
            .widget_state::<SelectBoxState>("theme_select")
            .selected
            .clone()
        {
            if let Ok(idx) = idx_str.parse::<usize>() {
                if idx < themes.len() {
                    self.current_theme_idx = idx;
                }
            }
        }

        let current_theme = themes[self.current_theme_idx % themes.len()];
        ui.set_theme(current_theme);

        let window_size = ui.window_size();

        let mut window_size_label = Label::new(
            "window_size_label",
            format!("Window size: {:.2} x {:.2}", window_size[0], window_size[1]),
        );
        let mut theme_label = Label::new(
            "theme_label",
            format!("Current theme: {}", current_theme.name),
        );
        let mut theme_btn = Button::new("theme_btn", "Change Theme");

        let mut good_switch = Switch::new("good_switch");
        let mut fast_switch = Switch::new("fast_switch");
        let mut cheap_switch = Switch::new("cheap_switch");

        let mut good_state = ui.take_widget_state::<SwitchState>("good_switch");
        let mut fast_state = ui.take_widget_state::<SwitchState>("fast_switch");
        let mut cheap_state = ui.take_widget_state::<SwitchState>("cheap_switch");
        let mut order_state = ui.take_widget_state::<TripleToggleOrder>("triple_toggle_order");

        let states = [
            ("good_switch", &mut good_state.enabled),
            ("fast_switch", &mut fast_state.enabled),
            ("cheap_switch", &mut cheap_state.enabled),
        ];

        for (id, enabled) in states {
            if *enabled && !order_state.order.contains(&id) {
                order_state.order.push(id);
            }
            if !*enabled {
                order_state.order.retain(|&x| x != id);
            }
        }

        while order_state.order.len() > 2 {
            let evicted = order_state.order.remove(0);
            match evicted {
                "good_switch" => good_state.enabled = false,
                "fast_switch" => fast_state.enabled = false,
                "cheap_switch" => cheap_state.enabled = false,
                _ => {}
            }
        }

        ui.put_widget_state("good_switch", good_state);
        ui.put_widget_state("fast_switch", fast_state);
        ui.put_widget_state("cheap_switch", cheap_state);
        ui.put_widget_state("triple_toggle_order", order_state);

        let mut radio_yes_option = RadioButton::new("radio_group", "radio_button_yes");
        let mut radio_yes_label = Label::new("_19", "Yes");
        let mut radio_button_yes = row![&mut radio_yes_option, &mut radio_yes_label];

        let mut radio_no_option = RadioButton::new("radio_group", "radio_button_no");
        let mut radio_no_label = Label::new("_10", "No");
        let mut radio_button_no = row![&mut radio_no_option, &mut radio_no_label];

        let mut radio_button_group =
            glacex::column![&mut radio_button_yes, &mut radio_button_no].align(Alignment::End);

        let mut joke_label = Label::new("_11", {
            format!(
                "{}, I'm gay",
                match ui.selected_option("radio_group") {
                    Some("radio_button_yes") => "Yes",
                    Some("radio_button_no") => "No",
                    None => "...",
                    Some(&_) => "...",
                }
            )
        });

        let mut slider = Slider::new("slider", 0.0, 1.0);
        let progress = ui.widget_state::<SliderState>("slider");
        let mut progress_bar = ProgressBar::new(progress.value);

        // SelectBox widget
        let theme_options: Vec<SelectOption> = themes
            .iter()
            .enumerate()
            .map(|(i, t)| SelectOption::new(i.to_string(), t.name))
            .collect();
        let mut theme_select = SelectBox::new("theme_select", theme_options)
            .placeholder("Select theme...")
            .width(200.0)
            .show_clear(true);

        {
            row![
                &mut ScrollView::new(
                    "scroll_wrapper",
                    &mut glacex::column![
                        &mut row![
                            &mut window_size_label,
                            &mut Divider::vertical(24.0).thickness(2.0),
                            &mut theme_label,
                            &mut theme_btn
                        ],
                        &mut row![
                            &mut Card::new(
                                &mut ScrollView::new(
                                    "scroll_view",
                                    &mut glacex::column![
                                        &mut row![
                                            &mut Badge::new("Badge").variant(BadgeVariant::Outline),
                                            &mut Badge::new("Success").success(),
                                            &mut Badge::new("Warning").warning(),
                                            &mut Badge::new("Dangerous stuff").error(),
                                        ],
                                        &mut row![
                                            &mut Button::new("btn_a", "Button"),
                                            &mut Button::new("btn_b", "Fashion button").style(
                                                ButtonStyle {
                                                    fill: Fill::Gradient(Gradient {
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
                                                        kind: GradientKind::Linear { angle: 45.0 },
                                                    }),
                                                    ..Default::default()
                                                }
                                            ),
                                            &mut Button::new("btn_c", "Fat button")
                                                .size([100.0, 50.0])
                                                .tooltip(
                                                    "In case you missed it, this is a fat button."
                                                )
                                        ]
                                        .align(Alignment::Center),
                                        &mut row![
                                            &mut glacex::column![
                                                &mut row![
                                                    &mut Label::new("_3", "Checkboxy"),
                                                    &mut Checkbox::new("checkbox_1"),
                                                ],
                                                &mut row![
                                                    &mut Label::new("_4", "Another one"),
                                                    &mut Checkbox::new("checkbox_2"),
                                                ],
                                                &mut row![
                                                    &mut Label::new("_5", "Hehe"),
                                                    &mut Checkbox::new("checkbox_3"),
                                                ]
                                            ]
                                            .align(Alignment::End),
                                            &mut glacex::column![
                                                &mut row![
                                                    &mut good_switch,
                                                    &mut Label::new("_6", "Good"),
                                                ],
                                                &mut row![
                                                    &mut fast_switch,
                                                    &mut Label::new("_7", "Fast"),
                                                ],
                                                &mut row![
                                                    &mut cheap_switch,
                                                    &mut Label::new("_8", "Cheap"),
                                                ]
                                            ]
                                            .align(Alignment::Start)
                                        ]
                                        .align(Alignment::Center)
                                        .spacing(100.0),
                                        &mut theme_select,
                                        &mut TextInput::new("text_input")
                                            .placeholder("Here goes text."),
                                        &mut TextArea::new("text_area"),
                                    ]
                                    .spacing(20.0)
                                )
                                .padding([16.0; 2])
                            )
                            .size([370.0, 420.0]),
                            &mut Card::new(
                                &mut glacex::column![
                                    &mut row![&mut radio_button_group, &mut joke_label,]
                                        .spacing(24.0)
                                        .align(Alignment::Center),
                                    &mut glacex::column![
                                        &mut Label::new("_33", "Heading").heading(),
                                        &mut Label::new("_34", "Subheading").subheading(),
                                        &mut Label::new("_35", "Captionist").caption(),
                                    ]
                                    .spacing(12.0)
                                    .align(Alignment::Start),
                                    &mut slider,
                                    &mut progress_bar,
                                ]
                                .spacing(24.0)
                                .align(Alignment::Center)
                            )
                            .height(420.0)
                        ]
                        .spacing(20.0),
                        &mut glacex::column![
                            &mut Label::new(
                                "_36",
                                "And here ladies and gentleman, I'm afraid our demo ended."
                            )
                            .size_preset(18.0),
                            &mut Label::new("_37", "Be free to check out Glacex's github. <3")
                                .size_preset(14.0),
                        ],
                    ]
                    .padding([60.0; 2])
                    .width(window_size[0])
                    .spacing(36.0)
                    .align(Alignment::Center)
                )
                .size([window_size[0], window_size[1]])
            ]
            .align(Alignment::Center)
            .arrange_at([0.0; 2], ui);
        }

        if theme_btn.clicked() {
            self.current_theme_idx = (self.current_theme_idx + 1) % themes.len();
            let mut sel_state = ui.take_widget_state::<SelectBoxState>("theme_select");
            sel_state.selected = Some(self.current_theme_idx.to_string());
            ui.put_widget_state("theme_select", sel_state);
        }
    }
}

fn main() {
    App::new(DemoApp::new())
        .title("Glacex - High Performance GPU UI Demo")
        .window_size(1360, 920)
        .run();
}
