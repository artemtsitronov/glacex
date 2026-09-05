use glacex::{
    Alignment, App, Badge, BadgeVariant, Button, ButtonStyle, Card, CardStyle, Checkbox, Color,
    Divider, Fill, Gradient, GradientKind, GradientStop, Label, ProgressBar, RadioButton,
    ScrollView, ShadowStyle, Slider, StatefulWidget, Switch, TextArea, TextInput, Theme, Ui,
    Widget, column, row,
};

fn primary_gradient(elapsed: f32) -> Fill {
    let angle = (elapsed * 20.0) % 360.0;
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

struct DemoApp {
    events_count: u32,
}

impl DemoApp {
    fn new() -> Self {
        DemoApp { events_count: 148 }
    }
}

impl Widget for DemoApp {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Theme::BG_CANVAS);

        let accent = primary_gradient(ui.elapsed_seconds());

        // Read currently selected cluster to reflect in header
        let cluster_selected = ui.selected_option("cluster_select").unwrap_or("us_east");
        let cluster_label_text = if cluster_selected == "eu_west" {
            "CLUSTER: EU-WEST"
        } else {
            "CLUSTER: US-EAST-1"
        };

        // Header Navigation Bar
        let mut logo_badge = Badge::new("GLACEX").variant(BadgeVariant::Success);
        let mut header_title = Label::new("Developer Dashboard");
        let mut env_badge = Badge::new(cluster_label_text).variant(BadgeVariant::Outline);
        let mut status_badge = Badge::new("ONLINE").variant(BadgeVariant::Success);
        let mut subtitle = Label::new(
            "GPU-rendered immediate-mode UI in Rust. wgpu SDF quads, taffy flexbox, glyphon text.",
        );

        // Column 1: Action Controls & Metrics
        let mut col1_title = Label::new("Compute & Dispatch");
        let mut counter_stat = Label::new(format!("Dispatched Tasks: {}", self.events_count));

        let mut primary_action_btn = Button::new("dispatch_task", "Dispatch Task")
            .tooltip("Submits a high-priority background worker task")
            .style(ButtonStyle {
                fill: accent,
                hover_fill: Fill::Solid(Theme::ACTIVE_HOVER),
                pressed_fill: Fill::Solid(Theme::ACTIVE),
                border_width: 1.0,
                border_color: Color::WHITE.with_alpha(0.2),
                corner_radius: 8.0,
                shadow: Some(ShadowStyle {
                    color: Theme::ACTIVE.with_alpha(0.35),
                    blur_radius: 12.0,
                    offset: [0.0, 3.0],
                }),
                sharp: false,
            });

        let mut reset_counter_btn = Button::new("reset_monitor", "Reset Metrics")
            .tooltip("Resets processed counters to default baseline")
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

        let mut api_title = Label::new("Endpoint Configuration");
        let mut endpoint_label = Label::new("Ingress Gateway Host");
        let mut endpoint_input = TextInput::new("endpoint_input", 320.0)
            .default_text("https://gateway.internal.net/v2/ingest");

        let mut payload_label = Label::new("Telemetry Metadata (JSON)");
        let mut payload_area = TextArea::new("payload_json", 320.0, 110.0).default_text(
            "{\n  \"service\": \"analytics-worker\",\n  \"sample_rate\": 1.0,\n  \"batch_size\": 256,\n  \"compression\": \"zstd\"\n}",
        );

        // Column 2: System Toggles & Policies
        let mut col2_title = Label::new("Runtime Policies");

        let mut live_stream_switch = Switch::new("live_stream_toggle").default_enabled(true);
        let mut live_stream_label = Label::new("Real-time Event Streaming");
        let mut live_stream_badge = Badge::new("ACTIVE").variant(BadgeVariant::Success);

        let mut auto_reconnect_check = Checkbox::new("auto_reconnect").default_checked(true);
        let mut auto_reconnect_label = Label::new("Automatic Node Failover");

        let mut strict_tls_check = Checkbox::new("strict_tls").default_checked(true);
        let mut strict_tls_label = Label::new("Enforce Mutual TLS v1.3");

        let mut compression_check = Checkbox::new("payload_compression").default_checked(true);
        let mut compression_label = Label::new("Enable Wire Compression");

        let mut slider_caption = Label::new("Bandwidth Allotment");
        let mut bandwidth_slider =
            Slider::new("bandwidth_slider", 0.0, 100.0, 320.0).default_value(80.0);
        let bandwidth_val = bandwidth_slider.state(ui).value;
        let mut bandwidth_progress_label =
            Label::new(format!("Allocated Capacity: {:.0}%", bandwidth_val));
        let mut bandwidth_progress = ProgressBar::new(bandwidth_val / 100.0, 320.0);

        let mut region_label = Label::new("Deployment Region");
        let mut cluster_us = RadioButton::new("cluster_select", "us_east");
        let mut cluster_us_label = Label::new("US-East (Primary Region)");
        let mut cluster_eu = RadioButton::new("cluster_select", "eu_west");
        let mut cluster_eu_label = Label::new("EU-West (Failover Replica)");

        // Activity & Diagnostics Log View
        let mut activity_title = Label::new("System Diagnostic Logs");
        let mut log_line_1 =
            Label::new("[09:24:01] [wgpu] Initialized swapchain surface on primary GPU adapter");
        let mut log_line_2 =
            Label::new("[09:24:02] [layout] Computed Taffy flexbox dimensions for 36 nodes");
        let mut log_line_3 = Label::new(
            "[09:24:03] [pipeline] Warmed SDF quad shaders (anti-aliased rounded rects)",
        );
        let mut log_line_4 =
            Label::new("[09:24:04] [text] Glyphon glyph cache mapped 4 font faces");
        let mut log_line_5 =
            Label::new("[09:24:05] [network] Connected to telemetry backend: ping 1.2ms");
        let mut log_line_6 =
            Label::new("[09:24:06] [animation] Smooth eased transitions active on all controls");

        let mut log_col = column![
            &mut log_line_1,
            &mut log_line_2,
            &mut log_line_3,
            &mut log_line_4,
            &mut log_line_5,
            &mut log_line_6
        ]
        .spacing(4.0)
        .align(Alignment::Start);

        let mut log_scroll_view =
            ScrollView::new("log_scroll_container", [712.0, 110.0], &mut log_col);

        let mut divider_top = Divider::horizontal(748.0);
        let mut divider_mid = Divider::horizontal(748.0);

        {
            let mut btn_row = row![&mut primary_action_btn, &mut reset_counter_btn].spacing(10.0);

            let mut left_card_content = column![
                &mut col1_title,
                &mut counter_stat,
                &mut btn_row,
                &mut api_title,
                &mut endpoint_label,
                &mut endpoint_input,
                &mut payload_label,
                &mut payload_area,
            ]
            .spacing(10.0)
            .align(Alignment::Start);

            let mut stream_row = row![
                &mut live_stream_switch,
                &mut live_stream_label,
                &mut live_stream_badge
            ]
            .spacing(8.0)
            .align(Alignment::Center);

            let mut auto_reconnect_row = row![&mut auto_reconnect_check, &mut auto_reconnect_label]
                .spacing(8.0)
                .align(Alignment::Center);

            let mut strict_tls_row = row![&mut strict_tls_check, &mut strict_tls_label]
                .spacing(8.0)
                .align(Alignment::Center);

            let mut compression_row = row![&mut compression_check, &mut compression_label]
                .spacing(8.0)
                .align(Alignment::Center);

            let mut cluster_us_row = row![&mut cluster_us, &mut cluster_us_label]
                .spacing(8.0)
                .align(Alignment::Center);

            let mut cluster_eu_row = row![&mut cluster_eu, &mut cluster_eu_label]
                .spacing(8.0)
                .align(Alignment::Center);

            let mut right_card_content = column![
                &mut col2_title,
                &mut stream_row,
                &mut auto_reconnect_row,
                &mut strict_tls_row,
                &mut compression_row,
                &mut slider_caption,
                &mut bandwidth_slider,
                &mut bandwidth_progress_label,
                &mut bandwidth_progress,
                &mut region_label,
                &mut cluster_us_row,
                &mut cluster_eu_row,
            ]
            .spacing(9.0)
            .align(Alignment::Start);

            let mut left_card = Card::new(&mut left_card_content).style(CardStyle {
                padding: [20.0, 20.0],
                ..Default::default()
            });

            let mut right_card = Card::new(&mut right_card_content).style(CardStyle {
                padding: [20.0, 20.0],
                ..Default::default()
            });

            let mut nav_row = row![
                &mut logo_badge,
                &mut header_title,
                &mut env_badge,
                &mut status_badge
            ]
            .spacing(12.0)
            .align(Alignment::Center);

            let mut cards_row = row![&mut left_card, &mut right_card]
                .spacing(16.0)
                .align(Alignment::Start);

            let mut root_column = column![
                &mut nav_row,
                &mut subtitle,
                &mut divider_top,
                &mut cards_row,
                &mut divider_mid,
                &mut activity_title,
                &mut log_scroll_view,
            ]
            .spacing(12.0)
            .align(Alignment::Start);

            // Center nicely within 1080p canvas with comfortable breathing margins
            root_column.arrange_at([56.0, 48.0], ui);
        }

        if primary_action_btn.clicked() {
            self.events_count += 1;
        }
        if reset_counter_btn.clicked() {
            self.events_count = 0;
        }
    }
}

fn main() {
    App::new(DemoApp::new())
        .title("Glacex - High Performance GPU UI Demo")
        .window_size(1280, 800)
        .run();
}
