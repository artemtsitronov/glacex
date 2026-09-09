use glacex::*;

struct DemoApp {
    current_theme_idx: usize,
    initialized: bool,
}

impl DemoApp {
    fn new() -> Self {
        DemoApp {
            current_theme_idx: 1, // default: Linear / pure dark
            initialized: false,
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

        // ------------------------------------------------------------------ //
        // Sync SelectBox state with active theme on initial frame
        // ------------------------------------------------------------------ //
        if !self.initialized {
            let mut sel_state = ui
                .take_widget_state_or::<SelectBoxState>("theme_select", SelectBoxState::default());
            sel_state.selected = Some(self.current_theme_idx.to_string());
            ui.put_widget_state("theme_select", sel_state);
            self.initialized = true;
        }

        let current_theme = themes[self.current_theme_idx % themes.len()];
        ui.set_theme(current_theme);

        let window_size = ui.window_size();
        let content_width = (window_size[0] - 80.0).clamp(760.0, 1140.0);

        // ------------------------------------------------------------------ //
        // Switch logic: max 2 of 3 can be active at once
        // ------------------------------------------------------------------ //
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

        // ------------------------------------------------------------------ //
        // Interactive Widgets
        // ------------------------------------------------------------------ //
        let mut good_switch = Switch::new("good_switch");
        let mut fast_switch = Switch::new("fast_switch");
        let mut cheap_switch = Switch::new("cheap_switch");

        let mut radio_yes_option = RadioButton::new("radio_group", "radio_button_yes");
        let mut radio_yes_label = Label::new("_19", "Instant Mode");
        let mut radio_button_yes = row![&mut radio_yes_option, &mut radio_yes_label];

        let mut radio_no_option = RadioButton::new("radio_group", "radio_button_no");
        let mut radio_no_label = Label::new("_10", "Fluid Spring Physics");
        let mut radio_button_no = row![&mut radio_no_option, &mut radio_no_label];

        let mut radio_button_group =
            glacex::column![&mut radio_button_yes, &mut radio_button_no].align(Alignment::Start);

        let selected_radio = ui
            .selected_option("radio_group")
            .unwrap_or("radio_button_no");
        let mut radio_status = Label::new(
            "radio_status",
            if selected_radio == "radio_button_yes" {
                "State: Zero-latency instantaneous response"
            } else {
                "State: Critically damped spring simulation (380 rad/s)"
            },
        )
        .caption()
        .muted();

        let mut slider = Slider::new("slider", 0.0, 1.0);
        let progress = ui.widget_state::<SliderState>("slider");
        let progress_val = progress.value;
        let mut progress_bar = ProgressBar::new(progress_val);

        // Theme selector (combobox) with searchable support
        let theme_options: Vec<SelectOption> = themes
            .iter()
            .enumerate()
            .map(|(i, t)| SelectOption::new(i.to_string(), t.name))
            .collect();
        let mut theme_select = SelectBox::new("theme_select", theme_options)
            .placeholder("Select theme…")
            .width(220.0)
            .searchable();

        // ------------------------------------------------------------------ //
        // Layout Tree
        // ------------------------------------------------------------------ //
        {
            row![
                &mut ScrollView::new(
                    "scroll_wrapper",
                    &mut glacex::column![
                        // ── Header Card ─────────────────────────────────── //
                        &mut Card::new(
                            &mut row![
                                &mut glacex::column![
                                    &mut row![
                                        &mut Label::new("brand", "GLACEX").title().bold(),
                                        &mut Badge::new("v0.1.6").success(),
                                    ]
                                    .spacing(10.0)
                                    .align(Alignment::Center),
                                    &mut Label::new(
                                        "sub_brand",
                                        "Pure GPU Native UI • Borderless Neumorphism • 120 FPS Fluid Motion",
                                    )
                                    .caption()
                                    .muted(),
                                ]
                                .spacing(4.0)
                                .align(Alignment::Start),

                                &mut row![
                                    &mut Label::new("theme_label", "Palette:").caption().bold(),
                                    &mut theme_select,
                                ]
                                .spacing(12.0)
                                .align(Alignment::Center),
                            ]
                            .spacing(24.0)
                            .align(Alignment::Center)
                        )
                        .padding([24.0, 18.0])
                        .width(content_width),

                        // ── Metric KPI Cards ────────────────────────────── //
                        &mut row![
                            &mut Card::new(
                                &mut glacex::column![
                                    &mut row![
                                        &mut Label::new("m1_t", "Frame Budget").caption().muted(),
                                        &mut Badge::new("120 FPS").success(),
                                    ]
                                    .spacing(8.0)
                                    .align(Alignment::Center),
                                    &mut Label::new("m1_v", "0.38 ms").metric().bold(),
                                    &mut Label::new("m1_s", "Zero-alloc render pass").caption().muted(),
                                ]
                                .spacing(6.0)
                                .align(Alignment::Start)
                            )
                            .padding([18.0, 16.0])
                            .width((content_width - 32.0) / 3.0),

                            &mut Card::new(
                                &mut glacex::column![
                                    &mut row![
                                        &mut Label::new("m2_t", "Vector Pipeline").caption().muted(),
                                        &mut Badge::new("SDF AA").variant(BadgeVariant::Outline),
                                    ]
                                    .spacing(8.0)
                                    .align(Alignment::Center),
                                    &mut Label::new("m2_v", "WGSL Ray").metric().bold(),
                                    &mut Label::new("m2_s", "Analytical sub-pixel fade").caption().muted(),
                                ]
                                .spacing(6.0)
                                .align(Alignment::Start)
                            )
                            .padding([18.0, 16.0])
                            .width((content_width - 32.0) / 3.0),

                            &mut Card::new(
                                &mut glacex::column![
                                    &mut row![
                                        &mut Label::new("m3_t", "Active Preset").caption().muted(),
                                        &mut Badge::new("Zero Border").warning(),
                                    ]
                                    .spacing(8.0)
                                    .align(Alignment::Center),
                                    &mut Label::new("m3_v", current_theme.name).metric().bold(),
                                    &mut Label::new("m3_s", "9 calibrated token palettes").caption().muted(),
                                ]
                                .spacing(6.0)
                                .align(Alignment::Start)
                            )
                            .padding([18.0, 16.0])
                            .width((content_width - 32.0) / 3.0),
                        ]
                        .spacing(16.0)
                        .align(Alignment::Center),

                        // ── Main Content Split ──────────────────────────── //
                        &mut row![
                            // Left Panel: Interactive controls
                            &mut Card::new(
                                &mut ScrollView::new(
                                    "controls_scroll",
                                    &mut glacex::column![
                                        &mut Label::new("ctrl_title", "Interactive Components")
                                            .heading()
                                            .bold(),

                                        // Status badges
                                        &mut row![
                                            &mut Badge::new("Default").variant(BadgeVariant::Outline),
                                            &mut Badge::new("Verified").success(),
                                            &mut Badge::new("Active").variant(BadgeVariant::Secondary),
                                            &mut Badge::new("Latency").warning(),
                                            &mut Badge::new("Warning").error(),
                                        ]
                                        .spacing(8.0),

                                        // Action buttons
                                        &mut row![
                                            &mut Button::new("btn_p", "Primary Action").primary(),
                                            &mut Button::new("btn_g", "Gradient").style(
                                                ButtonStyle {
                                                    fill: Fill::Gradient(Gradient {
                                                        stops: vec![
                                                            GradientStop {
                                                                position: 0.0,
                                                                color: Color::rgb(99, 102, 241), // Indigo 500
                                                            },
                                                            GradientStop {
                                                                position: 1.0,
                                                                color: Color::rgb(168, 85, 247), // Purple 500
                                                            },
                                                        ],
                                                        kind: GradientKind::Linear { angle: 135.0 },
                                                    }),
                                                    corner_radius: 12.0,
                                                    border_width: 0.0,
                                                    border_color: Color::TRANSPARENT,
                                                    text_color: Color::WHITE,
                                                    padding: [16.0, 9.0],
                                                    shadow: Some(ShadowStyle {
                                                        color: Color::rgba(99, 102, 241, 0.35),
                                                        blur_radius: 14.0,
                                                        offset: [0.0, 4.0],
                                                    }),
                                                    sharp: false,
                                                    ..Default::default()
                                                }
                                            ),
                                            &mut Button::new("btn_s", "Subtle")
                                                .tooltip("Tactile borderless button with soft depth"),
                                        ]
                                        .spacing(12.0)
                                        .align(Alignment::Center),

                                        // Checkboxes and Switches
                                        &mut row![
                                            &mut glacex::column![
                                                &mut row![
                                                    &mut Checkbox::new("cb_1"),
                                                    &mut Label::new("cb_1_l", "Accessibility AT-SPI"),
                                                ]
                                                .spacing(10.0)
                                                .align(Alignment::Center),
                                                &mut row![
                                                    &mut Checkbox::new("cb_2"),
                                                    &mut Label::new("cb_2_l", "Hardware VSync"),
                                                ]
                                                .spacing(10.0)
                                                .align(Alignment::Center),
                                                &mut row![
                                                    &mut Checkbox::new("cb_3"),
                                                    &mut Label::new("cb_3_l", "Sub-pixel Glyphs"),
                                                ]
                                                .spacing(10.0)
                                                .align(Alignment::Center),
                                            ]
                                            .spacing(12.0)
                                            .align(Alignment::Start),

                                            &mut glacex::column![
                                                &mut row![
                                                    &mut good_switch,
                                                    &mut Label::new("sw_1_l", "Luxury Aesthetic"),
                                                ]
                                                .spacing(10.0)
                                                .align(Alignment::Center),
                                                &mut row![
                                                    &mut fast_switch,
                                                    &mut Label::new("sw_2_l", "Peak Performance"),
                                                ]
                                                .spacing(10.0)
                                                .align(Alignment::Center),
                                                &mut row![
                                                    &mut cheap_switch,
                                                    &mut Label::new("sw_3_l", "Zero Bloat"),
                                                ]
                                                .spacing(10.0)
                                                .align(Alignment::Center),
                                            ]
                                            .spacing(12.0)
                                            .align(Alignment::Start),
                                        ]
                                        .spacing(36.0)
                                        .align(Alignment::Center),

                                        // Inputs
                                        &mut TextInput::new("text_input")
                                            .placeholder("Type to inspect live font rasterization…"),
                                        &mut TextArea::new("text_area"),
                                    ]
                                    .spacing(20.0)
                                    .align(Alignment::Start)
                                )
                                .padding([20.0, 20.0])
                            )
                            .size([(content_width - 16.0) * 0.52, 450.0])
                            .padding([0.0, 0.0]),

                            // Right Panel: Motion & Typography
                            &mut Card::new(
                                &mut glacex::column![
                                    &mut Label::new("rt_title", "Motion & Kinetic Physics")
                                        .heading()
                                        .bold(),

                                    // Type scale
                                    &mut glacex::column![
                                        &mut Label::new("ty_disp", "Calculated Precision").title().bold(),
                                        &mut Label::new(
                                            "ty_sub",
                                            "Geist Sans variable typography with anti-aliased SDF edges"
                                        )
                                        .subheading(),
                                        &mut Label::new(
                                            "ty_mono",
                                            "0x7F_3A_90 • 128-bit SIMD geometry"
                                        )
                                        .mono()
                                        .caption()
                                        .muted(),
                                    ]
                                    .spacing(6.0)
                                    .align(Alignment::Start),

                                    &mut Divider::horizontal(340.0),

                                    // Radio selections
                                    &mut glacex::column![
                                        &mut radio_button_group,
                                        &mut radio_status,
                                    ]
                                    .spacing(8.0)
                                    .align(Alignment::Start),

                                    &mut Divider::horizontal(340.0),

                                    // Slider + fluid progress
                                    &mut glacex::column![
                                        &mut row![
                                            &mut Label::new("sl_txt", "Dynamic Spring Damping:").caption().bold(),
                                            &mut Label::new("sl_val", format!("{:.0}%", progress_val * 100.0)).caption().bold(),
                                        ]
                                        .spacing(8.0)
                                        .align(Alignment::Center),
                                        &mut slider,
                                        &mut progress_bar,
                                    ]
                                    .spacing(10.0)
                                    .align(Alignment::Start),
                                ]
                                .spacing(16.0)
                                .align(Alignment::Start)
                            )
                            .size([(content_width - 16.0) * 0.48, 450.0])
                            .padding([24.0, 22.0]),
                        ]
                        .spacing(16.0)
                        .align(Alignment::Center),

                        // ── Footer ───────────────────────────────────────── //
                        &mut Card::new(
                            &mut row![
                                &mut Label::new(
                                    "ft_1",
                                    "Glacex — Next-Generation GPU Accelerated GUI Toolkit for Rust"
                                )
                                .caption(),
                                &mut Label::new(
                                    "ft_2",
                                    "Crafted with pure math & wgpu shaders  ♡"
                                )
                                .caption()
                                .muted(),
                            ]
                            .spacing(24.0)
                            .align(Alignment::Center)
                        )
                        .padding([20.0, 14.0])
                        .width(content_width),
                    ]
                    .padding([28.0, 36.0])
                    .width(window_size[0])
                    .spacing(16.0)
                    .align(Alignment::Center)
                )
                .size([window_size[0], window_size[1]])
            ]
            .align(Alignment::Center)
            .arrange_at([0.0; 2], ui);
        }

        // ------------------------------------------------------------------ //
        // Post-arrange: read SelectBox selection for theme switching
        // ------------------------------------------------------------------ //
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
    }
}

fn main() {
    App::new(DemoApp::new())
        .title("Glacex — Pure GPU Native UI")
        .window_size(1280, 880)
        .run();
}
