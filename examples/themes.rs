use glacex::{
    Alignment, App, Badge, BadgeVariant, Button, Card, Checkbox, Divider, Label, ProgressBar,
    RadioButton, Slider, Switch, TextArea, TextInput, Theme, Ui, Widget, column, row,
};

struct ThemesApp {
    selected_theme: usize,
    slider_val: f32,
    checked: bool,
    switched: bool,
}

impl ThemesApp {
    fn new() -> Self {
        ThemesApp {
            selected_theme: 0,
            slider_val: 65.0,
            checked: true,
            switched: true,
        }
    }
}

impl Widget for ThemesApp {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let themes = Theme::all();
        let current = themes[self.selected_theme % themes.len()];
        ui.set_theme(current);

        let mut title = Label::new("title", "Glacex Design Tokens & Themes Showcase");
        let mut subtitle = Label::new(
            "subtitle",
            "Toggle between 9 meticulously calibrated professional color palettes.",
        )
        .muted();

        let mut btn_0 = theme_button("_a", "shadcn Light", self.selected_theme == 0);
        let mut btn_1 = theme_button("_b", "Catppuccin Latte", self.selected_theme == 1);
        let mut btn_2 = theme_button("_c", "Gruvbox Light", self.selected_theme == 2);
        let mut btn_3 = theme_button("_d", "shadcn Dark", self.selected_theme == 3);
        let mut btn_4 = theme_button("_e", "Catppuccin Mocha", self.selected_theme == 4);
        let mut btn_5 = theme_button("_f", "Tokyo Night", self.selected_theme == 5);
        let mut btn_6 = theme_button("_g", "Gruvbox Dark", self.selected_theme == 6);
        let mut btn_7 = theme_button("_h", "Nord", self.selected_theme == 7);
        let mut btn_8 = theme_button("_i", "Rosé Pine", self.selected_theme == 8);

        let mut active_badge =
            Badge::new(format!("Active Theme: {}", current.name)).variant(BadgeVariant::Success);
        let mut mode_badge = Badge::new(if current.is_dark {
            "DARK MODE"
        } else {
            "LIGHT MODE"
        })
        .variant(BadgeVariant::Outline);

        let mut left_heading = Label::new("left_heading", "Interactive Surface & Controls");
        let mut primary_btn = Button::new("_j", "Primary Action").primary();
        let mut outline_btn = Button::new("_k", "Outline Button").outline();
        let mut ghost_btn = Button::new("_l", "Ghost Button").ghost();
        let mut danger_btn = Button::new("_m", "Destructive Action").danger();

        let mut input_label =
            Label::new("input_label", "Input Field with Floating Focus").secondary();
        let mut text_input = TextInput::new("showcase_input")
            .width(280.0)
            .placeholder("Enter text here...")
            .default_text("Glacex immediate UI");

        let mut area_label = Label::new("area_label", "Multi-line Text Surface").secondary();
        let mut text_area = TextArea::new("showcase_area")
            .size([280.0, 75.0])
            .default_text(
                "Minimal design tokens.\nGPU-accelerated typography.\nSDF rounded corners.",
            );

        let mut right_heading = Label::new("right_heading", "Toggles, Sliders & Badges");
        let mut switch = Switch::new("showcase_switch").default_enabled(self.switched);
        let mut switch_label = Label::new("switch_label", "Hardware Acceleration");

        let mut check = Checkbox::new("showcase_check").default_checked(self.checked);
        let mut check_label = Label::new("check_label", "Sub-pixel Text Antialiasing");

        let mut slider_label = Label::new(
            "slider_label",
            format!("Surface Blur Radius: {:.0}px", self.slider_val),
        )
        .secondary();
        let mut slider = Slider::new("showcase_slider", 0.0, 100.0)
            .width(280.0)
            .default_value(self.slider_val);
        let slider_resp = slider.state(ui);
        self.slider_val = slider_resp.value;

        let mut progress = ProgressBar::new(self.slider_val / 100.0)
            .width(280.0)
            .id("showcase_prog");

        let mut radio_label = Label::new("radio_label", "Rendering Backend").secondary();
        let mut radio_wgpu = RadioButton::new("backend_group", "wgpu");
        let mut radio_wgpu_lbl = Label::new("radio_wgpu_lbl", "wgpu WebGPU / Vulkan");
        let mut radio_dx12 = RadioButton::new("backend_group", "metal");
        let mut radio_dx12_lbl = Label::new("radio_dx12_lbl", "Metal / DirectX 12");

        let mut div_1 = Divider::horizontal(700.0).faint();
        let mut div_2 = Divider::horizontal(700.0).faint();

        {
            let mut top_btns = row![&mut btn_0, &mut btn_1, &mut btn_2, &mut btn_3, &mut btn_4]
                .spacing(8.0)
                .align(Alignment::Center);
            let mut bot_btns = row![&mut btn_5, &mut btn_6, &mut btn_7, &mut btn_8]
                .spacing(8.0)
                .align(Alignment::Center);

            let mut header_badges = row![&mut active_badge, &mut mode_badge]
                .spacing(8.0)
                .align(Alignment::Center);

            let mut btns_row = row![
                &mut primary_btn,
                &mut outline_btn,
                &mut ghost_btn,
                &mut danger_btn
            ]
            .spacing(8.0)
            .align(Alignment::Center);

            let mut left_content = column![
                &mut left_heading,
                &mut btns_row,
                &mut input_label,
                &mut text_input,
                &mut area_label,
                &mut text_area,
            ]
            .spacing(10.0)
            .align(Alignment::Start);

            let mut switch_row = row![&mut switch, &mut switch_label]
                .spacing(8.0)
                .align(Alignment::Center);
            let mut check_row = row![&mut check, &mut check_label]
                .spacing(8.0)
                .align(Alignment::Center);
            let mut radio_1_row = row![&mut radio_wgpu, &mut radio_wgpu_lbl]
                .spacing(8.0)
                .align(Alignment::Center);
            let mut radio_2_row = row![&mut radio_dx12, &mut radio_dx12_lbl]
                .spacing(8.0)
                .align(Alignment::Center);

            let mut right_content = column![
                &mut right_heading,
                &mut switch_row,
                &mut check_row,
                &mut slider_label,
                &mut slider,
                &mut progress,
                &mut radio_label,
                &mut radio_1_row,
                &mut radio_2_row,
            ]
            .spacing(10.0)
            .align(Alignment::Start);

            let mut left_card = Card::new(&mut left_content).padding([18.0, 18.0]);
            let mut right_card = Card::new(&mut right_content).padding([18.0, 18.0]);

            let mut cards_row = row![&mut left_card, &mut right_card]
                .spacing(16.0)
                .align(Alignment::Start);

            let mut layout = column![
                &mut title,
                &mut subtitle,
                &mut header_badges,
                &mut div_1,
                &mut top_btns,
                &mut bot_btns,
                &mut div_2,
                &mut cards_row,
            ]
            .spacing(12.0)
            .align(Alignment::Start);

            layout.arrange_at([48.0, 40.0], ui);
        }

        if btn_0.clicked() {
            self.selected_theme = 0;
        }
        if btn_1.clicked() {
            self.selected_theme = 1;
        }
        if btn_2.clicked() {
            self.selected_theme = 2;
        }
        if btn_3.clicked() {
            self.selected_theme = 3;
        }
        if btn_4.clicked() {
            self.selected_theme = 4;
        }
        if btn_5.clicked() {
            self.selected_theme = 5;
        }
        if btn_6.clicked() {
            self.selected_theme = 6;
        }
        if btn_7.clicked() {
            self.selected_theme = 7;
        }
        if btn_8.clicked() {
            self.selected_theme = 8;
        }
    }
}

fn theme_button(id: &str, label: &str, selected: bool) -> Button {
    let button = Button::new(id, label);
    if selected {
        button.primary()
    } else {
        button.outline()
    }
}

fn main() {
    App::new(ThemesApp::new())
        .title("Glacex - Preset Themes Showcase")
        .window_size(1180, 760)
        .run();
}
