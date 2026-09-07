use glacex::{
    Alignment, App, Badge, BadgeVariant, Button, Card, Checkbox, Divider, Label, ProgressBar,
    RadioButton, ScrollView, Slider, Switch, TextArea, TextInput, Theme, Ui, Widget, column, row,
};

struct DemoApp {
    events_count: u32,
    current_theme_idx: usize,
}

impl DemoApp {
    fn new() -> Self {
        DemoApp {
            events_count: 148,
            current_theme_idx: 1,
        }
    }
}

impl Widget for DemoApp {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let themes = Theme::all();
        let current_theme = themes[self.current_theme_idx % themes.len()];
        ui.set_theme(current_theme);

        let cluster_selected = ui.selected_option("cluster_select").unwrap_or("us_east");
        let is_eu = cluster_selected == "eu_west";

        let mut logo_label = Label::new("logo_label", "Acme Inc.").heading();
        let mut logo_badge = Badge::new("v0.1.4").variant(BadgeVariant::Secondary);
        let mut search_input = TextInput::new("nav_search")
            .width(240.0)
            .placeholder("Search documentation...");
        let mut status_badge = Badge::new("ONLINE").variant(BadgeVariant::Success);

        let theme_label_text = format!("Theme: {}", current_theme.name);
        let mut theme_btn = Button::new("_a", theme_label_text)
            .tooltip("Cycle 9 curated palettes (Light, Dark, Catppuccin, Tokyo Night, Gruvbox, Nord, Rosé Pine)")
            .outline();

        let mut page_title = Label::new("page_title", "Dashboard Overview").title();
        let mut page_subtitle = Label::new("page_subtitle", "High-performance GPU immediate-mode UI with bundled Geist typography and SDF surfaces.").muted();

        let mut stat1_metric = Label::new("stat1_metric", "$1,250.00").metric();
        let mut stat1_title = Label::new("stat1_title", "Trending up this month ↗").medium();
        let mut stat1_sub = Label::new("stat1_sub", "Visitors for the last 6 months")
            .secondary()
            .caption();

        let mut stat2_metric =
            Label::new("stat2_metric", format!("{:#}", self.events_count)).metric();
        let mut stat2_title = Label::new("stat2_title", "Down 20% this period ↘").medium();
        let mut stat2_sub = Label::new("stat2_sub", "Acquisition needs attention")
            .secondary()
            .caption();

        let mut stat3_metric = Label::new("stat3_metric", "45,678").metric();
        let mut stat3_title = Label::new("stat3_title", "Strong user retention ↗").medium();
        let mut stat3_sub = Label::new("stat3_sub", "Engagement exceeds targets")
            .secondary()
            .caption();

        let mut stat4_metric = Label::new("stat4_metric", "4.5%").metric();
        let mut stat4_title = Label::new("stat4_title", "Steady performance increase ↗").medium();
        let mut stat4_sub = Label::new(
            "stat4_sub",
            if is_eu {
                "Meets Frankfurt growth projections"
            } else {
                "Meets growth projections"
            },
        )
        .secondary()
        .caption();

        let mut col1_title = Label::new("col1_title", "Compute & Dispatch").subheading();
        let mut primary_action_btn = Button::new("_b", "Dispatch Task")
            .tooltip("Submits high-priority worker task")
            .primary();
        let mut reset_counter_btn = Button::new("_c", "Reset Metrics")
            .tooltip("Resets processed task counters")
            .outline();
        let mut danger_btn = Button::new("_d", "Purge Queue")
            .tooltip("Clears worker cache")
            .danger();

        let mut api_title = Label::new("api_title", "Endpoint Configuration").subheading();
        let mut endpoint_label = Label::new("endpoint_label", "Ingress Gateway Host").secondary();
        let mut endpoint_input = TextInput::new("endpoint_input")
            .width(350.0)
            .placeholder("https://api.gateway.internal/v2/ingest")
            .default_text("https://gateway.internal.net/v2/ingest");

        let mut payload_label =
            Label::new("payload_label", "Telemetry Metadata (JSON)").secondary();
        let mut payload_area = TextArea::new("payload_json").size([350.0, 110.0]).default_text(
            "{\n  \"service\": \"analytics-worker\",\n  \"sample_rate\": 1.0,\n  \"batch_size\": 256,\n  \"compression\": \"zstd\"\n}",
        );

        let mut col2_title = Label::new("col2_title", "Runtime Policies").subheading();

        let mut live_stream_switch = Switch::new("live_stream_toggle").default_enabled(true);
        let mut live_stream_label = Label::new("live_stream_label", "Real-time Event Streaming");
        let mut live_stream_badge = Badge::new("ACTIVE").variant(BadgeVariant::Success);

        let mut auto_reconnect_check = Checkbox::new("auto_reconnect").default_checked(true);
        let mut auto_reconnect_label =
            Label::new("auto_reconnect_label", "Automatic Node Failover");

        let mut strict_tls_check = Checkbox::new("strict_tls").default_checked(true);
        let mut strict_tls_label = Label::new("strict_tls_label", "Enforce Mutual TLS v1.3");

        let mut compression_check = Checkbox::new("payload_compression").default_checked(true);
        let mut compression_label = Label::new("compression_label", "Wire Compression (zstd)");

        let mut slider_caption = Label::new("slider_caption", "Bandwidth Allotment").secondary();
        let mut bandwidth_slider = Slider::new("bandwidth_slider", 0.0, 100.0)
            .width(350.0)
            .default_value(80.0);
        let bandwidth_val = bandwidth_slider.state(ui).value;
        let mut bandwidth_progress_label = Label::new(
            "bandwidth_progress_label",
            format!("Allocated Capacity: {:.0}%", bandwidth_val),
        )
        .secondary();
        let mut bandwidth_progress = ProgressBar::new(bandwidth_val / 100.0)
            .width(350.0)
            .id("bandwidth_progress");

        let mut region_label = Label::new("region_label", "Deployment Cluster").secondary();
        let mut cluster_us = RadioButton::new("cluster_select", "us_east");
        let mut cluster_us_label = Label::new("cluster_us_label", "US-East-1 (Primary Region)");
        let mut cluster_eu = RadioButton::new("cluster_select", "eu_west");
        let mut cluster_eu_label = Label::new("cluster_eu_label", "EU-West-1 (Failover Replica)");

        let mut activity_title =
            Label::new("activity_title", "System Diagnostic Logs").subheading();
        let mut log_line_1 = Label::new(
            "log_line_1",
            "[09:24:01] [wgpu] Initialized swapchain surface on primary GPU adapter",
        )
        .mono()
        .caption();
        let mut log_line_2 = Label::new(
            "log_line_2",
            "[09:24:02] [layout] Computed Taffy flexbox dimensions for 48 nodes",
        )
        .mono()
        .caption();
        let mut log_line_3 = Label::new(
            "log_line_3",
            "[09:24:03] [pipeline] Warmed SDF quad shaders with sub-pixel antialiasing",
        )
        .mono()
        .caption();
        let mut log_line_4 = Label::new(
            "log_line_4",
            "[09:24:04] [text] Loaded bundled Geist Sans & Geist Mono font family",
        )
        .mono()
        .caption();
        let mut log_line_5 = Label::new(
            "log_line_5",
            "[09:24:05] [network] Connected to telemetry backend: ping 1.2ms",
        )
        .mono()
        .caption();
        let mut log_line_6 = Label::new(
            "log_line_6",
            "[09:24:06] [motion] Spring & fluid physics active across all surfaces",
        )
        .mono()
        .caption();

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
            ScrollView::new("log_scroll_container", &mut log_col).size([764.0, 110.0]);

        let mut divider_top = Divider::horizontal(804.0).faint();
        let mut divider_mid = Divider::horizontal(804.0).faint();

        {
            let mut stat1_col = column![&mut stat1_metric, &mut stat1_title, &mut stat1_sub]
                .spacing(6.0)
                .align(Alignment::Start);
            let mut stat1_card = Card::new(&mut stat1_col).padding([22.0, 18.0]);

            let mut stat2_col = column![&mut stat2_metric, &mut stat2_title, &mut stat2_sub]
                .spacing(6.0)
                .align(Alignment::Start);
            let mut stat2_card = Card::new(&mut stat2_col).padding([22.0, 18.0]);

            let mut stat3_col = column![&mut stat3_metric, &mut stat3_title, &mut stat3_sub]
                .spacing(6.0)
                .align(Alignment::Start);
            let mut stat3_card = Card::new(&mut stat3_col).padding([22.0, 18.0]);

            let mut stat4_col = column![&mut stat4_metric, &mut stat4_title, &mut stat4_sub]
                .spacing(6.0)
                .align(Alignment::Start);
            let mut stat4_card = Card::new(&mut stat4_col).padding([22.0, 18.0]);

            let mut stats_row = row![
                &mut stat1_card,
                &mut stat2_card,
                &mut stat3_card,
                &mut stat4_card
            ]
            .spacing(14.0)
            .align(Alignment::Start);

            let mut btn_row = row![
                &mut primary_action_btn,
                &mut reset_counter_btn,
                &mut danger_btn
            ]
            .spacing(8.0)
            .align(Alignment::Center);

            let mut left_card_content = column![
                &mut col1_title,
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
            .spacing(8.0)
            .align(Alignment::Start);

            let mut left_card = Card::new(&mut left_card_content);
            let mut right_card = Card::new(&mut right_card_content);

            let mut nav_brand = row![&mut logo_label, &mut logo_badge]
                .spacing(8.0)
                .align(Alignment::Center);

            let mut nav_row = row![
                &mut nav_brand,
                &mut search_input,
                &mut status_badge,
                &mut theme_btn
            ]
            .spacing(16.0)
            .align(Alignment::Center);

            let mut cards_row = row![&mut left_card, &mut right_card]
                .spacing(16.0)
                .align(Alignment::Start);

            let mut root_column = column![
                &mut nav_row,
                &mut page_title,
                &mut page_subtitle,
                &mut divider_top,
                &mut stats_row,
                &mut cards_row,
                &mut divider_mid,
                &mut activity_title,
                &mut log_scroll_view,
            ]
            .spacing(12.0)
            .align(Alignment::Start);

            let window_size = ui.window_size();
            ScrollView::new("full_view", &mut root_column)
                .size(window_size)
                .padding([16.0; 2])
                .ui(ui);
        }

        if primary_action_btn.clicked() {
            self.events_count += 1;
        }
        if reset_counter_btn.clicked() || danger_btn.clicked() {
            self.events_count = 0;
        }
        if theme_btn.clicked() {
            self.current_theme_idx = (self.current_theme_idx + 1) % themes.len();
        }
    }
}

fn main() {
    App::new(DemoApp::new())
        .title("Glacex - High Performance GPU UI Demo")
        .window_size(1360, 920)
        .accessibility_enabled(true)
        .run();
}
