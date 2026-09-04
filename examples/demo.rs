use glacex::{
    Alignment, App, Badge, BadgeVariant, Button, ButtonStyle, Card, CardStyle, Checkbox,
    CheckboxState, Color, Divider, Fill, Gradient, GradientKind, GradientStop, Label, ProgressBar,
    RadioButton, ScrollView, ShadowStyle, Slider, SliderState, Switch, SwitchState, TextArea,
    TextInput, Theme, Ui, Widget, column, row,
};

fn subtle_gradient(elapsed: f32) -> Fill {
    let angle = (elapsed * 30.0) % 360.0;
    Fill::Gradient(Gradient {
        kind: GradientKind::Linear { angle },
        stops: vec![
            GradientStop {
                position: 0.0,
                color: Color::rgb(79, 70, 229), // Indigo 600
            },
            GradientStop {
                position: 1.0,
                color: Color::rgb(147, 51, 234), // Purple 600
            },
        ],
    })
}

struct AppState {
    count: u32,
    slider_init: bool,
}

impl AppState {
    fn new() -> Self {
        AppState {
            count: 42,
            slider_init: false,
        }
    }
}

impl Widget for AppState {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Theme::BG_CANVAS);

        if !self.slider_init {
            let slider_st = ui.widget_state::<SliderState>("volume_slider");
            slider_st.value = 65.0;
            let switch_st = ui.widget_state::<SwitchState>("stream_switch");
            switch_st.enabled = true;
            let check_st = ui.widget_state::<CheckboxState>("persist_check");
            check_st.checked = true;
            self.slider_init = true;
        }

        let brand_gradient = subtle_gradient(ui.elapsed_seconds());

        // Header Section
        let mut title = Label::new("GLACEX STUDIO");
        let mut live_badge = Badge::new("LIVE PREVIEW").variant(BadgeVariant::Success);
        let mut v_badge = Badge::new("v0.1.2").variant(BadgeVariant::Outline);
        let mut header_desc = Label::new("Next-generation GPU-accelerated UI primitives in Rust.");

        // Left Panel: Metric Counter & Action Buttons
        let mut counter_header = Label::new("METRICS");
        let mut counter_val = Label::new(format!("Events Processed: {}", self.count));
        let mut primary_btn = Button::new("Deploy Trigger")
            .tooltip("Triggers a simulated deployment event")
            .style(ButtonStyle {
                fill: brand_gradient,
                hover_fill: Fill::Solid(Theme::ACTIVE_HOVER),
                pressed_fill: Fill::Solid(Theme::ACTIVE),
                border_width: 1.0,
                border_color: Color::WHITE.with_alpha(0.2),
                corner_radius: 8.0,
                shadow: Some(ShadowStyle {
                    color: Theme::ACTIVE.with_alpha(0.4),
                    blur_radius: 12.0,
                    offset: [0.0, 3.0],
                }),
                sharp: false,
            });

        let mut reset_btn = Button::new("Reset Counter")
            .tooltip("Resets processed events to zero")
            .style(ButtonStyle {
                fill: Fill::Solid(Theme::SURFACE_SUBTLE),
                hover_fill: Fill::Solid(Theme::HOVERED),
                pressed_fill: Fill::Solid(Theme::ACTIVE),
                border_width: 1.0,
            border_color: Theme::BORDER,
            corner_radius: 8.0,
            shadow: None,
            sharp: false,
        });

        // Form Section
        let mut form_header = Label::new("ENDPOINT SETTINGS");
        let mut url_label = Label::new("Deployment Hostname");
        let mut url_input = TextInput::new("host_input", 280.0);

        let mut payload_label = Label::new("Configuration Payload (JSON)");
        let mut payload_area = TextArea::new("config_area", 280.0, 80.0);

        // Right Panel: Sliders, Switches, Progress
        let mut controls_header = Label::new("SYSTEM CONTROLS");

        let mut switch_label = Label::new("Live Telemetry Stream");
        let mut stream_switch = Switch::new("stream_switch");
        let mut stream_badge = Badge::new("ACTIVE").variant(BadgeVariant::Success);

        let mut check_label = Label::new("Persist Sessions");
        let mut persist_checkbox = Checkbox::new("persist_check");

        let volume_val = ui.widget_state::<SliderState>("volume_slider").value;
        let mut slider_caption = Label::new(format!("Bandwidth Throttling: {:.0}%", volume_val));
        let mut vol_slider = Slider::new("volume_slider", 0.0, 100.0, 280.0);
        let mut progress_indicator = ProgressBar::new(volume_val / 100.0, 280.0);

        let mut env_label = Label::new("Target Cluster");
        let mut env_prod = RadioButton::new("cluster_group", "prod");
        let mut env_prod_label = Label::new("Production (us-east)");
        let mut env_staging = RadioButton::new("cluster_group", "staging");
        let mut env_staging_label = Label::new("Staging (eu-central)");

        // Logs ScrollView
        let mut logs_header = Label::new("CONTAINER LOG STREAM");
        let mut log1 = Label::new("[00:01:02] GPU Pipeline initialized via wgpu WebGPU backend");
        let mut log2 = Label::new("[00:01:03] Glyph cache warmed: glyphon 0.12 swash integration");
        let mut log3 =
            Label::new("[00:01:04] SDF Quad rasterization pipeline online (anti-aliased)");
        let mut log4 = Label::new("[00:01:05] Taffy flexbox layout tree verified and cached");
        let mut log5 = Label::new("[00:01:06] Connection established to vertex buffers");

        let mut logs_col = column![&mut log1, &mut log2, &mut log3, &mut log4, &mut log5,]
            .spacing(4.0)
            .align(Alignment::Start);

        let mut log_scroll = ScrollView::new("activity_scroll", [590.0, 90.0], &mut logs_col);

        let mut divider_top = Divider::horizontal(620.0);
        let mut divider_mid = Divider::horizontal(620.0);

        let mut btn_row = row![&mut primary_btn, &mut reset_btn].spacing(10.0);

        if let Some(btn) = btn_row.get_mut::<Button>(0) {
            if btn.clicked() {
                self.count += 1;
            }
        }
        if let Some(btn) = btn_row.get_mut::<Button>(1) {
            if btn.clicked() {
                self.count = 0;
            }
        }

        let mut left_content = column![
            &mut counter_header,
            &mut counter_val,
            &mut btn_row,
            &mut form_header,
            &mut url_label,
            &mut url_input,
            &mut payload_label,
            &mut payload_area,
        ]
        .spacing(10.0)
        .align(Alignment::Start);

        let mut stream_row = row![&mut stream_switch, &mut switch_label, &mut stream_badge]
            .spacing(8.0)
            .align(Alignment::Center);
        let mut persist_row = row![&mut persist_checkbox, &mut check_label]
            .spacing(8.0)
            .align(Alignment::Center);
        let mut env_prod_row = row![&mut env_prod, &mut env_prod_label]
            .spacing(8.0)
            .align(Alignment::Center);
        let mut env_staging_row = row![&mut env_staging, &mut env_staging_label]
            .spacing(8.0)
            .align(Alignment::Center);

        let mut right_content = column![
            &mut controls_header,
            &mut stream_row,
            &mut persist_row,
            &mut slider_caption,
            &mut vol_slider,
            &mut progress_indicator,
            &mut env_label,
            &mut env_prod_row,
            &mut env_staging_row,
        ]
        .spacing(10.0)
        .align(Alignment::Start);

        let mut left_card = Card::new(&mut left_content).style(CardStyle {
            padding: [18.0, 18.0],
            ..Default::default()
        });
        let mut right_card = Card::new(&mut right_content).style(CardStyle {
            padding: [18.0, 18.0],
            ..Default::default()
        });

        let mut header_row = row![&mut title, &mut live_badge, &mut v_badge]
            .spacing(12.0)
            .align(Alignment::Center);
        let mut cards_row = row![&mut left_card, &mut right_card]
            .spacing(16.0)
            .align(Alignment::Start);

        let mut root_column = column![
            &mut header_row,
            &mut header_desc,
            &mut divider_top,
            &mut cards_row,
            &mut divider_mid,
            &mut logs_header,
            &mut log_scroll,
        ]
        .spacing(12.0)
        .align(Alignment::Start);

        root_column.arrange_at([40.0, 30.0], ui);
    }
}

fn main() {
    let state = AppState::new();
    App::new(state).run();
}
